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
static HEARTBEAT_API: &str = "/heartbeat";

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
pub(crate) struct HeartbeatResponse {}

#[derive(Resource, Default)]
struct InFlightHeartbeatRequests(Vec<(Instant, Option<HttpRequestHandle>)>);

fn system_heartbeat_keepalive(
    mut requests: HttpRequests,
    mut in_flight_requests: ResMut<InFlightHeartbeatRequests>,
    runner: Res<ChromaRunner>,
) {
    in_flight_requests.0.push((
        Instant::now() + Duration::from_secs_f32(HEARTBEAT_TIMEOUT),
        Some(
            requests.request(
                requests
                    .client()
                    .post(runner.get_session_url(HEARTBEAT_API))
                    .json(&HeartbeatRequest),
            ),
        ),
    ))
}

fn system_heartbeat_cleanup(
    mut requests: HttpRequests,
    mut in_flight_requests: ResMut<InFlightHeartbeatRequests>,
) {
    let now = Instant::now().elapsed();

    in_flight_requests
        .0
        .retain_mut(|(deadline, ref mut in_flight_request)| {
            if deadline.elapsed() > now {
                requests.dispose_option(in_flight_request);
                false
            } else {
                true
            }
        });
}
