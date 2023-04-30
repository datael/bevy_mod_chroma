use std::time::Duration;

use bevy::{
    prelude::{
        resource_exists, App, Commands, Component, Condition, IntoSystemConfig, Plugin, Query, Res,
    },
    time::common_conditions::on_timer,
    utils::Instant,
};
use bevy_mod_chroma_request_lib::{HttpRequestHandle, HttpRequests};
use serde::{Deserialize, Serialize};

use crate::ChromaRunner;

static HEARTBEAT_INTERVAL: f32 = 1.0;
static HEARTBEAT_TIMEOUT: f32 = 10.0;
static HEARTBEAT_API: &str = "heartbeat";

pub(crate) struct HeartbeatPlugin;

impl Plugin for HeartbeatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            system_heartbeat_keepalive.run_if(
                resource_exists::<ChromaRunner>()
                    .and_then(on_timer(Duration::from_secs_f32(HEARTBEAT_INTERVAL))),
            ),
        )
        .add_system(
            system_heartbeat_cleanup.run_if(on_timer(Duration::from_secs_f32(HEARTBEAT_INTERVAL))),
        );
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct HeartbeatRequest;

#[derive(Debug, Deserialize)]
pub(crate) struct HeartbeatResponse {
    #[serde(rename(deserialize = "tick"))]
    _tick: u32,
}

#[derive(Component)]
struct InFlightHeartbeatRequest {
    spawned_at: Instant,
    request_handle: Option<HttpRequestHandle>,
}

impl InFlightHeartbeatRequest {
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.spawned_at) > Duration::from_secs_f32(HEARTBEAT_TIMEOUT)
    }
}

fn system_heartbeat_keepalive(
    mut commands: Commands,
    mut requests: HttpRequests,
    runner: Res<ChromaRunner>,
) {
    let handle = requests.request(
        requests
            .client()
            .put(runner.get_session_url(HEARTBEAT_API))
            .json(&HeartbeatRequest),
    );

    commands
        .entity(handle.entity())
        .insert(InFlightHeartbeatRequest {
            spawned_at: Instant::now(),
            request_handle: Some(handle),
        });
}

fn system_heartbeat_cleanup(
    mut requests: HttpRequests,
    mut in_flight_requests: Query<&mut InFlightHeartbeatRequest>,
) {
    for mut in_flight_request in in_flight_requests.iter_mut() {
        if !in_flight_request.is_expired() {
            continue;
        }

        let request_handle = in_flight_request.request_handle.take().unwrap();
        requests.dispose(request_handle);
    }
}
