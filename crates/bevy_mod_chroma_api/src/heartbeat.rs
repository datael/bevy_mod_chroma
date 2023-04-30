use std::time::Duration;

use bevy::{
    prelude::{resource_exists, App, Condition, IntoSystemConfig, Plugin, Res, ResMut, Resource},
    time::common_conditions::on_timer,
    utils::Instant,
};
use bevy_mod_chroma_request_lib::{HttpRequestHandle, HttpRequests};
use serde::{Deserialize, Serialize};

use crate::ChromaRunner;

static HEARTBEAT_INTERVAL: f32 = 1.0;
static HEARTBEAT_TIMEOUT: f32 = 10.0;
static HEARTBEAT_API: &str = "chromasdk/heartbeat";

pub(crate) struct HeartbeatPlugin;

impl Plugin for HeartbeatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InFlightHeartbeatRequests>()
            .add_system(
                system_heartbeat_keepalive.run_if(
                    resource_exists::<ChromaRunner>()
                        .and_then(on_timer(Duration::from_secs_f32(HEARTBEAT_INTERVAL))),
                ),
            )
            .add_system(
                system_heartbeat_cleanup
                    .run_if(on_timer(Duration::from_secs_f32(HEARTBEAT_INTERVAL))),
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

#[derive(Resource, Default)]
struct InFlightHeartbeatRequests(Vec<InFlightHeartbeatRequest>);

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
    mut requests: HttpRequests,
    mut in_flight_requests: ResMut<InFlightHeartbeatRequests>,
    runner: Res<ChromaRunner>,
) {
    in_flight_requests.0.push(InFlightHeartbeatRequest {
        spawned_at: Instant::now(),
        request_handle: Some(
            requests.request(
                requests
                    .client()
                    .put(runner.get_session_url(HEARTBEAT_API))
                    .json(&HeartbeatRequest),
            ),
        ),
    })
}

fn system_heartbeat_cleanup(
    mut requests: HttpRequests,
    mut in_flight_requests: ResMut<InFlightHeartbeatRequests>,
) {
    while let Some(in_flight_request) = in_flight_requests.0.get_mut(0) {
        if !in_flight_request.is_expired() {
            break;
        }

        let request_handle = in_flight_request.request_handle.take().unwrap();
        requests.dispose(request_handle);
        in_flight_requests.0.remove(0);
    }
}
