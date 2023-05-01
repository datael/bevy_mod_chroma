use bevy::{
    prelude::{App, Commands, Component, Entity, Plugin, Query, Resource, Without},
    tasks::AsyncComputeTaskPool,
};
use crossbeam_channel::Receiver;
use reqwest::{Client, RequestBuilder};

use crate::{HttpRequestError, HttpRequestPlugin, HttpResponse};

impl Plugin for HttpRequestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HttpRequestClient>()
            .add_system(system_execute_requests)
            .add_system(system_gather_responses);
    }
}

#[derive(Resource, Default)]
pub(crate) struct HttpRequestClient {
    pub(crate) client: Client,
}

#[derive(Component)]
pub(crate) struct HttpRequest {
    builder: Option<RequestBuilder>,
}

impl HttpRequest {
    pub(crate) fn new(builder: RequestBuilder) -> Self {
        Self {
            builder: Some(builder),
        }
    }
}

#[derive(Component, Debug)]
pub(crate) struct HttpRequestInProgress {
    pub receiver: Receiver<Result<HttpResponse, HttpRequestError>>,
}

#[derive(Component, Debug)]
pub(crate) struct HttpResponseReceived {
    pub(crate) result: Result<HttpResponse, HttpRequestError>,
}

fn system_execute_requests(
    mut commands: Commands,
    mut query: Query<(Entity, &mut HttpRequest), Without<HttpRequestInProgress>>,
) {
    for (entity, mut request) in query.iter_mut() {
        let (sender, receiver) = crossbeam_channel::bounded(1);

        // SAFETY: This system is only invoked when a request builder is not
        // already in progress, and we only allow construction of a request by
        // passing in a request builder instance, so this will never be None.
        let request_builder = request.builder.take().unwrap();

        AsyncComputeTaskPool::get()
            .spawn(async move {
                async fn run_request(
                    request: RequestBuilder,
                ) -> Result<HttpResponse, HttpRequestError> {
                    let response = request.send().await?;
                    let status_code = response.status();

                    Ok(HttpResponse {
                        status_code,
                        body_bytes: response.bytes().await?,
                    })
                }

                let result = run_request(request_builder).await;
                sender.send(result).expect("sent successfully");
            })
            .detach();

        commands
            .entity(entity)
            .insert(HttpRequestInProgress { receiver })
            .remove::<HttpRequest>();
    }
}

fn system_gather_responses(
    mut commands: Commands,
    query: Query<(Entity, &HttpRequestInProgress), Without<HttpResponseReceived>>,
) {
    for (entity, in_progress) in query.iter() {
        if let Ok(result) = in_progress.receiver.try_recv() {
            commands
                .entity(entity)
                .insert(HttpResponseReceived { result })
                .remove::<HttpRequestInProgress>();
        }
    }
}
