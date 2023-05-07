use bevy::{
    log::*,
    prelude::{
        in_state, resource_exists, App, Commands, Component, Condition, Entity, In, IntoPipeSystem,
        IntoSystemConfig, Local, Plugin, Query, Res, ResMut, State, States, Without,
    },
    utils::Instant,
};
use bevy_mod_chroma_request_lib::{
    HttpRequestError, HttpRequestHandle, HttpRequestPlugin, HttpRequestSet, HttpRequests,
};
use serde::Serialize;

use crate::{
    api::{CreateEffectResponse, Effect, SessionInfo},
    heartbeat::HeartbeatPlugin,
    ChromaPlugin, ChromaRunner, ChromaRunnerInitializationSettings,
};

impl Plugin for ChromaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings.clone())
            .add_state::<RunnerState>()
            .add_plugin(HttpRequestPlugin)
            .add_plugin(HeartbeatPlugin)
            .add_system(
                system_init.pipe(system_init_error_handler).run_if(
                    resource_exists::<ChromaRunnerInitializationSettings>()
                        .and_then(in_state(RunnerState::Init)),
                ),
            )
            .add_system(
                system_create_pending_effects
                    .in_base_set(HttpRequestSet::BeforeExecuteRequests)
                    .run_if(
                        resource_exists::<ChromaRunner>().and_then(in_state(RunnerState::Running)),
                    ),
            )
            .add_system(
                system_gather_create_effect_results
                    .in_base_set(HttpRequestSet::AfterGatherResponses)
                    .run_if(
                        resource_exists::<ChromaRunner>().and_then(in_state(RunnerState::Running)),
                    ),
            )
            .add_system(
                system_apply_effects
                    .in_base_set(HttpRequestSet::BeforeExecuteRequests)
                    .run_if(
                        resource_exists::<ChromaRunner>().and_then(in_state(RunnerState::Running)),
                    ),
            )
            .add_system(
                system_apply_effects_cleanup
                    .in_base_set(HttpRequestSet::AfterGatherResponses)
                    .run_if(
                        resource_exists::<ChromaRunner>().and_then(in_state(RunnerState::Running)),
                    ),
            );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum RunnerState {
    #[default]
    Init,
    Running,
    Stopped,
    Error,
}

fn system_init_error_handler(
    In(result): In<Result<(), InitError>>,
    mut runner_state: ResMut<State<RunnerState>>,
) {
    if let Err(err) = result {
        runner_state.0 = RunnerState::Error;
        error!("failed to initialize chroma runner: {:?}", err);
    }
}

fn system_init(
    mut commands: Commands,
    mut init_request: Local<Option<HttpRequestHandle>>,
    mut requests: HttpRequests,
    mut runner_state: ResMut<State<RunnerState>>,
    init: Res<ChromaRunnerInitializationSettings>,
) -> Result<(), InitError> {
    if init_request.is_none() {
        *init_request = Some(
            requests.request(
                requests
                    .client()
                    .post(init.init_url)
                    .json(&init.init_request),
            ),
        );

        return Ok(());
    }

    // SAFETY: init_request is always Some here as verified above
    if let Some(response) = requests.get_response(init_request.as_ref().unwrap()) {
        let session_info = response.as_ref()?.json::<SessionInfo>()?;
        let root_url = if session_info.root_url.ends_with("/") {
            session_info.root_url
        } else {
            session_info.root_url + "/"
        };

        commands.insert_resource(ChromaRunner {
            root_url: root_url.as_str().try_into()?,
        });
        commands.remove_resource::<ChromaRunnerInitializationSettings>();

        // SAFETY: as above, init_request is always Some here
        let init_request = init_request.take().unwrap();
        requests.dispose(init_request);

        runner_state.0 = RunnerState::Running;
        info!("successfully opened chroma session to {}", root_url);
    }

    return Ok(());
}

#[derive(Debug)]
enum InitError {
    RequestError(HttpRequestError),
    ParseError(serde_json::Error),
    UrlError(url::ParseError),
}

impl From<&HttpRequestError> for InitError {
    fn from(error: &HttpRequestError) -> Self {
        Self::RequestError(error.clone())
    }
}

impl From<serde_json::Error> for InitError {
    fn from(error: serde_json::Error) -> Self {
        Self::ParseError(error)
    }
}

impl From<url::ParseError> for InitError {
    fn from(error: url::ParseError) -> Self {
        Self::UrlError(error)
    }
}

#[derive(Component)]
struct InFlightCreateEffectRequest {
    request_handle: Option<HttpRequestHandle>,
}

#[derive(Component, Serialize)]
struct CreatedEffect {
    id: String,
}

fn system_create_pending_effects(
    mut commands: Commands,
    mut requests: HttpRequests,
    runner: Res<ChromaRunner>,
    pending_effects: Query<
        (Entity, &Effect),
        (Without<InFlightCreateEffectRequest>, Without<CreatedEffect>),
    >,
) {
    for (entity, effect) in pending_effects.iter() {
        let request_handle = requests.request(
            requests
                .client()
                .post(runner.get_session_url(effect.get_api()))
                .json(effect),
        );

        commands.entity(entity).insert(InFlightCreateEffectRequest {
            request_handle: Some(request_handle),
        });
    }
}

fn system_gather_create_effect_results(
    mut commands: Commands,
    mut requests: HttpRequests,
    mut in_flight_create_requests: Query<
        (Entity, &mut InFlightCreateEffectRequest),
        Without<CreatedEffect>,
    >,
) {
    for (entity, mut in_flight_request) in in_flight_create_requests.iter_mut() {
        if let Some(result) =
            requests.get_response(in_flight_request.request_handle.as_ref().unwrap())
        {
            // TODO error check result body
            if let Ok(success_result) = result {
                let id = success_result
                    .json::<CreateEffectResponse>()
                    .unwrap()
                    .id()
                    .into();

                commands
                    .entity(entity)
                    .insert(CreatedEffect { id })
                    .remove::<InFlightCreateEffectRequest>();
            } else {
                error!("failed to create effect: {:?}", result);
            }

            let request_handle = in_flight_request.request_handle.take().unwrap();
            requests.dispose(request_handle);
        }
    }
}

#[derive(Component)]
pub(crate) struct ApplyEffectRequest {
    pub(crate) effect_entity: Entity,
    pub(crate) deadline: Instant,
}

impl ApplyEffectRequest {
    pub(crate) fn is_expired(&self) -> bool {
        Instant::now() > self.deadline
    }
}

#[derive(Component)]
pub(crate) struct InFlightApplyEffectRequest {
    request_handle: Option<HttpRequestHandle>,
    pub(crate) deadline: Instant,
}

impl InFlightApplyEffectRequest {
    pub(crate) fn is_expired(&self) -> bool {
        Instant::now() > self.deadline
    }
}

fn system_apply_effects(
    mut commands: Commands,
    mut requests: HttpRequests,
    runner: Res<ChromaRunner>,
    requests_query: Query<(Entity, &ApplyEffectRequest)>,
    effects_query: Query<&CreatedEffect>,
) {
    for (entity, application_request) in requests_query.iter() {
        if application_request.is_expired() {
            commands.entity(entity).despawn();
            continue;
        }

        if let Ok(created_effect) = effects_query.get(application_request.effect_entity) {
            let request_handle = requests.request(
                requests
                    .client()
                    .put(runner.get_session_url("effect"))
                    .json(created_effect),
            );

            commands
                .entity(entity)
                .insert(InFlightApplyEffectRequest {
                    request_handle: Some(request_handle),
                    deadline: application_request.deadline,
                })
                .remove::<ApplyEffectRequest>();
        }
    }
}

fn system_apply_effects_cleanup(
    mut commands: Commands,
    mut requests: HttpRequests,
    mut in_flight_requests_query: Query<(Entity, &mut InFlightApplyEffectRequest)>,
) {
    for (entity, mut in_flight_request) in in_flight_requests_query.iter_mut() {
        if in_flight_request.is_expired() {
            let request_handle = in_flight_request.request_handle.take().unwrap();
            requests.dispose(request_handle);
            commands.entity(entity).despawn();
            continue;
        }

        if let Some(result) =
            requests.get_response(in_flight_request.request_handle.as_ref().unwrap())
        {
            // TODO error check result body
            if let Err(err) = result {
                error!("failed to apply effect: {:?}", err);
            }

            let request_handle = in_flight_request.request_handle.take().unwrap();
            requests.dispose(request_handle);
            commands.entity(entity).despawn();
        }
    }
}
