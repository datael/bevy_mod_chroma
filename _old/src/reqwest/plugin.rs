use bevy::prelude::{App, Commands, Entity, Plugin, Query, Res, Resource, Without};
use bevy::tasks::AsyncComputeTaskPool;
use reqwest::RequestBuilder;

use super::RequestError;
use super::Response;
use super::{HTTPMethod, Request};
use super::{InProgress, ReqwestRunner};

#[derive(Resource, Default)]
pub(crate) struct ReqwestClient {
    pub(crate) client: reqwest::Client,
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
        app.init_resource::<ReqwestClient>()
            .add_system(system_execute_request_builder_requests::<ResponseBody>);
        // .add_system(system_gather_responses::<RequestBody, ResponseBody>);
    }
}

// fn system_execute_requests<RequestBody, ResponseBody>(
//     mut commands: Commands,
//     runner: Res<ReqwestClient>,
//     mut query: Query<
//         (Entity, &Request<RequestBody, ResponseBody>),
//         (
//             Without<InProgress<ResponseBody>>,
//             Without<Response<ResponseBody>>,
//         ),
//     >,
// ) where
//     RequestBody: super::RequestBody,
//     ResponseBody: super::ResponseBody,
// {
//     for (entity, request) in query.iter_mut() {
//         let body = request.body.clone();
//         let request = match request.method {
//             HTTPMethod::Post => runner.client.post(request.url.clone()).json(&body),
//             HTTPMethod::Put => runner.client.put(request.url.clone()).json(&body),
//             _ => todo! {},
//         };

//         let (sender, receiver) = crossbeam_channel::bounded(1);

//         AsyncComputeTaskPool::get()
//             .spawn(async move {
//                 let result = run_reqest::<ResponseBody>(request).await;
//                 sender.send(result).expect("sent successfully");
//             })
//             .detach();

//         commands.entity(entity).insert(InProgress { receiver });
//     }
// }

fn system_execute_request_builder_requests<ResponseBody>(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Request<ResponseBody>),
        (
            Without<InProgress<ResponseBody>>,
            Without<Response<ResponseBody>>,
        ),
    >,
) where
    ResponseBody: super::ResponseBody,
{
    for (entity, mut request) in query.iter_mut() {
        let (sender, receiver) = crossbeam_channel::bounded(1);

        let mut request_builder = None;
        std::mem::swap(&mut request.builder, &mut request_builder);

        // SAFETY: This system is only invoked when a request builder is not
        // already in progress, and we only allow construction of a request by
        // passing in a request builder instance, so this will never be None.
        let request_builder = request_builder.unwrap();

        AsyncComputeTaskPool::get()
            .spawn(async move {
                let result = run_request::<ResponseBody>(request_builder).await;
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

async fn run_request<T: super::ResponseBody>(request: RequestBuilder) -> Result<T, RequestError> {
    Ok(serde_json::from_str::<T>(
        request.send().await?.text().await?.as_str(),
    )?)
}
