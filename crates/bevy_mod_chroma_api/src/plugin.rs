use bevy::{
    log::*,
    prelude::{
        in_state, resource_exists, App, Commands, Condition, In, IntoPipeSystem, IntoSystemConfig,
        Local, Plugin, Res, ResMut, State, States,
    },
};
use bevy_mod_chroma_request_lib::{
    HttpRequestError, HttpRequestHandle, HttpRequestPlugin, HttpRequests,
};
use reqwest::Url;

use crate::{
    api::SessionInfo, heartbeat::HeartbeatPlugin, ChromaPlugin, ChromaRunner,
    ChromaRunnerInitializationSettings,
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

    // SAFETY: init_request is always some here as verified above
    if let Some(response) = requests.get_response(init_request.as_ref().unwrap()) {
        let session_info = response.as_ref()?.json::<SessionInfo>()?;
        let root_url = Url::parse(session_info.root_url.as_str())?;

        requests.dispose_option(&mut init_request);

        commands.insert_resource(ChromaRunner { root_url });
        commands.remove_resource::<ChromaRunnerInitializationSettings>();

        runner_state.0 = RunnerState::Running;
        info!("successfully opened chroma session");
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
