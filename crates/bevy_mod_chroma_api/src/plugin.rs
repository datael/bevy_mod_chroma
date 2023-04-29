use bevy::{
    log::*,
    prelude::{
        in_state, resource_exists, App, Commands, Condition, IntoSystemConfig, Local, Plugin, Res,
        ResMut, State, States,
    },
};
use bevy_mod_chroma_request_lib::{HttpRequestHandle, HttpRequestPlugin, HttpRequests};

use crate::{api::SessionInfo, ChromaPlugin, ChromaRunner, ChromaRunnerInitializationSettings};

impl Plugin for ChromaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.settings.clone())
            .add_state::<RunnerState>()
            .add_plugin(HttpRequestPlugin)
            .init_resource::<ChromaRunner>()
            .add_system(
                system_init.run_if(
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
}

fn system_init(
    mut commands: Commands,
    mut init_request: Local<Option<HttpRequestHandle>>,
    mut requests: HttpRequests,
    mut runner_state: ResMut<State<RunnerState>>,
    mut runner: ResMut<ChromaRunner>,
    init: Res<ChromaRunnerInitializationSettings>,
) {
    if init_request.is_none() {
        *init_request = Some(
            requests.request(
                requests
                    .client()
                    .post(init.init_url)
                    .json(&init.init_request),
            ),
        );

        commands.remove_resource::<ChromaRunnerInitializationSettings>();
        return;
    }

    // SAFETY: init_request is always some here as verified above
    if let Some(response) = requests.get_response(init_request.as_ref().unwrap()) {
        if let Ok(success_response) = response {
            if let Ok(session_info) = success_response.json::<SessionInfo>() {
                runner.session_info = Some(session_info);
                runner_state.0 = RunnerState::Running;
                info!(
                    "successfully opened chroma session: {:?}",
                    runner.session_info
                );
                return;
            } else {
                error!("failed to open chroma session: {:?}", success_response);
            }
        } else {
            error!("failed to open chroma session: {:?}", response);
        }
    }
}
