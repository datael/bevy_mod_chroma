use bevy::prelude::{App, Commands, Entity, Plugin, Query, Res, Resource, Without};
use bevy::tasks::AsyncComputeTaskPool;
use reqwest::RequestBuilder;

use super::HTTPMethod;
use super::RequestError;
use super::Response;
use super::{InProgress, Request};

#[derive(Resource, Default)]
pub(crate) struct ReqwestRunner {
    client: reqwest::Client,
}

pub struct ReqwestPlugin<RequestBody, ResponseBody> {
    phantom: std::marker::PhantomData<(RequestBody, ResponseBody)>,
}

impl<RequestBody, ResponseBody> Plugin for ReqwestPlugin<RequestBody, ResponseBody>
where
    RequestBody: super::RequestBody,
    ResponseBody: super::ResponseBody,
{
    fn build(&self, app: &mut App) {
        app.init_resource::<ReqwestRunner>()
            .add_system(system_execute_requests::<RequestBody, ResponseBody>)
            .add_system(system_gather_responses::<RequestBody, ResponseBody>);
    }
}

fn system_execute_requests<RequestBody, ResponseBody>(
    mut commands: Commands,
    runner: Res<ReqwestRunner>,
    mut query: Query<
        (Entity, &Request<RequestBody, ResponseBody>),
        (
            Without<InProgress<RequestBody>>,
            Without<Response<ResponseBody>>,
        ),
    >,
) where
    RequestBody: super::RequestBody,
    ResponseBody: super::ResponseBody,
{
    for (entity, request) in query.iter_mut() {
        let body = request.body.clone();
        let request = match request.method {
            HTTPMethod::Post => runner.client.post(request.url.clone()).json(&body),
            HTTPMethod::Put => runner.client.put(request.url.clone()).json(&body),
            _ => todo! {},
        };

        let (sender, receiver) = crossbeam_channel::bounded(1);

        AsyncComputeTaskPool::get()
            .spawn(async move {
                let result = run_reqest::<ResponseBody>(request).await;
                sender.send(result).expect("sent successfully");
            })
            .detach();

        commands.entity(entity).insert(InProgress { receiver });
    }
}

fn system_gather_responses<RequestBody, ResponseBody>(
    mut commands: Commands,
    query: Query<(Entity, &InProgress<RequestBody>), Without<Response<ResponseBody>>>,
) where
    RequestBody: super::RequestBody,
    ResponseBody: super::ResponseBody,
{
    for (entity, in_progress) in query.iter() {
        if let Ok(result) = in_progress.receiver.try_recv() {
            commands
                .entity(entity)
                .insert(Response { body: result })
                .remove::<InProgress<RequestBody>>();
        }
    }
}

async fn run_reqest<T: super::ResponseBody>(request: RequestBuilder) -> Result<T, RequestError> {
    Ok(serde_json::from_str::<T>(
        request.send().await?.text().await?.as_str(),
    )?)
}
