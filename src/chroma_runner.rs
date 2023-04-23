use std::{hash::Hash, time::Duration};

use bevy::{
    log::*, prelude::*, tasks::AsyncComputeTaskPool, time::common_conditions::on_timer,
    utils::HashMap,
};
use serde::{Deserialize, Serialize};

use crate::{bgr_color::*, request_support::*};

pub trait EffectIdentifier:
    'static + Hash + Sync + Send + Copy + PartialEq + Eq + std::fmt::Debug
{
}

impl<T> EffectIdentifier for T where
    T: 'static + Hash + Sync + Send + Copy + PartialEq + Eq + std::fmt::Debug
{
}

#[derive(Default)]
pub struct ChromaRunnerPlugin<T: EffectIdentifier> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: EffectIdentifier> Plugin for ChromaRunnerPlugin<T> {
    fn build(&self, app: &mut App) {
        let (
            chroma_runner_initialization_event_sender,
            chroma_runner_initialization_event_receiver,
        ) = crossbeam_channel::unbounded::<ChromaRunnerInitializedEvent>();

        let (
            chroma_runner_effect_created_event_sender,
            chroma_runner_effect_created_event_receiver,
        ) = crossbeam_channel::unbounded::<ChromaRunnerCreateEffectEvent<T>>();

        app.init_resource::<ChromaRunnerInitializationSettings>()
            .insert_resource(ChromaRunnerInitializationResultSender(
                chroma_runner_initialization_event_sender,
            ))
            .insert_resource(ChromaRunnerInitializationResultReceiver(
                chroma_runner_initialization_event_receiver,
            ))
            .insert_resource(ChromaRunner::<T> {
                root_url: None,
                create_effect_requests: Vec::new(),
                registered_effects: HashMap::new(),
                use_effect_requests: Vec::new(),
            })
            .add_system(chroma_runner_init::<T>)
            .insert_resource(ChromaRunnerCreateEffectEventSender::<T>(
                chroma_runner_effect_created_event_sender,
            ))
            .insert_resource(ChromaRunnerCreateEffectEventReceiver::<T>(
                chroma_runner_effect_created_event_receiver,
            ))
            .add_system(run_heartbeat::<T>.run_if(on_timer(Duration::from_secs(1))))
            .add_system(run_register_effect_requests::<T>)
            .add_system(gather_register_event_results::<T>)
            .add_system(run_use_effect_requests::<T>);
    }
}

fn run_heartbeat<T: EffectIdentifier>(chroma_runner: Res<ChromaRunner<T>>) {
    let uri = chroma_runner.to_full_url("/heartbeat");

    AsyncComputeTaskPool::get()
        .spawn(async move {
            let _ = put::<IgnoreContent>(uri, "").await;
        })
        .detach();
}

#[derive(Debug)]
pub struct EffectId(String);

#[derive(Debug, Deserialize)]
struct SessionInfo {
    #[serde(rename(deserialize = "sessionid"))]
    _session_id: u32,
    uri: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Author {
    name: &'static str,
    contact: &'static str,
}

#[derive(Debug, Serialize, Clone)]
pub struct Init {
    title: &'static str,
    description: &'static str,
    author: Author,
    device_supported: Vec<&'static str>,
    category: &'static str,
}

impl Default for Init {
    fn default() -> Self {
        Self {
            title: "Bevy Chroma",
            description: "Bevy Chroma",
            author: Author {
                name: "Darryl James",
                contact: "www.default.com",
            },
            device_supported: vec![
                "keyboard",
                "mousepad",
                "mouse",
                "headset",
                "keypad",
                "chromalink",
            ],
            category: "application",
        }
    }
}

#[derive(Resource)]
pub struct ChromaRunnerInitializationSettings {
    init_url: &'static str,
    init_request: Init,
}

impl Default for ChromaRunnerInitializationSettings {
    fn default() -> Self {
        Self {
            init_url: "http://localhost:54235/razer/chromasdk",
            init_request: Init::default(),
        }
    }
}

#[derive(Debug)]
pub enum EffectDefinition {
    Mouse(MouseRequest),
}

#[derive(Resource)]
pub struct ChromaRunner<T: EffectIdentifier> {
    pub(crate) root_url: Option<String>,
    create_effect_requests: Vec<(T, EffectDefinition)>,
    registered_effects: HashMap<T, EffectId>,
    use_effect_requests: Vec<T>,
}

struct ChromaRunnerInitializedEvent(Result<String, ()>);

#[derive(Resource, Deref, DerefMut, Clone)]
struct ChromaRunnerInitializationResultSender(
    pub crossbeam_channel::Sender<ChromaRunnerInitializedEvent>,
);

#[derive(Resource, Deref, DerefMut, Clone)]
struct ChromaRunnerInitializationResultReceiver(
    pub crossbeam_channel::Receiver<ChromaRunnerInitializedEvent>,
);

#[derive(Default)]
enum InitStage {
    #[default]
    NotStarted,
    AwaitingResponse,
    Finished,
}

fn chroma_runner_init<T: EffectIdentifier>(
    initialization_settings: Res<ChromaRunnerInitializationSettings>,
    sender: Res<ChromaRunnerInitializationResultSender>,
    receiver: Res<ChromaRunnerInitializationResultReceiver>,
    mut phase: Local<InitStage>,
    mut chroma_runner: ResMut<ChromaRunner<T>>,
) {
    match *phase {
        InitStage::Finished => {} // nothing more to do
        InitStage::NotStarted => {
            let sender = sender.clone();

            let ChromaRunnerInitializationSettings {
                init_url,
                init_request,
            } = initialization_settings.into_inner();

            let (init_url, init_request) = (init_url.to_string(), init_request.clone());

            AsyncComputeTaskPool::get()
                .spawn(async move {
                    if let Ok(response) =
                        post::<SessionInfo>(init_url.to_string(), init_request).await
                    {
                        info!("response: {:?}", response);
                        sender
                            .send(ChromaRunnerInitializedEvent(Ok(response.uri)))
                            .expect("sending of success successful");
                    } else {
                        error!("init failed");
                        sender
                            .send(ChromaRunnerInitializedEvent(Err(())))
                            .expect("sending of failure unsuccessful");
                    }
                })
                .detach();

            *phase = InitStage::AwaitingResponse;
        }
        InitStage::AwaitingResponse => {
            if let Ok(event) = receiver.try_recv() {
                if let Ok(root_url) = &event.0 {
                    chroma_runner.root_url = Some(root_url.to_owned());
                }
                *phase = InitStage::Finished;
            }
        }
    }
}

#[derive(Debug, Serialize, Clone, Copy)]
pub struct MouseRequest {
    pub effect: &'static str, // todo enum
    pub param: BGRColor,
}

impl<T: EffectIdentifier> ChromaRunner<T> {
    pub fn use_effect(&mut self, identifier: T) {
        self.use_effect_requests.push(identifier);
    }

    pub fn create_mouse_effect(&mut self, identifier: T, request_body: MouseRequest) {
        self.create_effect_requests
            .push((identifier, EffectDefinition::Mouse(request_body)));
    }
}

impl<T: EffectIdentifier> ChromaRunner<T> {
    fn to_full_url(&self, relative_path: &str) -> String {
        self.root_url.clone().unwrap() + relative_path.into()
    }
}

struct ChromaRunnerCreateEffectEvent<T: EffectIdentifier>((T, Result<String, ()>));

#[derive(Resource, Deref, DerefMut, Clone)]
struct ChromaRunnerCreateEffectEventSender<T: EffectIdentifier>(
    pub crossbeam_channel::Sender<ChromaRunnerCreateEffectEvent<T>>,
);

#[derive(Resource, Deref, DerefMut, Clone)]
struct ChromaRunnerCreateEffectEventReceiver<T: EffectIdentifier>(
    pub crossbeam_channel::Receiver<ChromaRunnerCreateEffectEvent<T>>,
);

#[derive(Debug, Deserialize)]
struct CreateEffectResponse {
    #[allow(unused)]
    result: u32, // todo enum
    id: String,
}

fn run_register_effect_requests<T: EffectIdentifier>(
    mut chroma_runner: ResMut<ChromaRunner<T>>,
    create_effect_event_sender: Res<ChromaRunnerCreateEffectEventSender<T>>,
) {
    if chroma_runner.root_url.is_none() {
        info!("init not yet finished");
        return;
    }

    if chroma_runner.create_effect_requests.is_empty() {
        return;
    }

    let urls_with_futures = chroma_runner
        .create_effect_requests
        .iter()
        .map(|new_request| match new_request {
            (identifier, EffectDefinition::Mouse(request_body)) => (
                "/mouse",
                identifier,
                post::<CreateEffectResponse>(
                    chroma_runner.to_full_url("/mouse"),
                    request_body.clone(),
                ),
            ),
        });

    let task_pool = AsyncComputeTaskPool::get();

    for (uri, identifier, fut) in urls_with_futures {
        let identifier = *identifier;
        let sender = create_effect_event_sender.clone();

        task_pool
            .spawn(async move {
                match fut.await {
                    Ok(create_effect_response) => {
                        info!("{} request succeeded", uri);
                        sender
                            .send(ChromaRunnerCreateEffectEvent((
                                identifier,
                                Ok(create_effect_response.id),
                            )))
                            .expect("sending of success successful");
                    }
                    Err(error) => error!("{} request failed: {:?}", uri, error),
                }
            })
            .detach();
    }

    chroma_runner.create_effect_requests.clear();
}

fn gather_register_event_results<T: EffectIdentifier>(
    mut chroma_runner: ResMut<ChromaRunner<T>>,
    create_effect_event_receiver: Res<ChromaRunnerCreateEffectEventReceiver<T>>,
) {
    while let Ok(event) = create_effect_event_receiver.try_recv() {
        let ChromaRunnerCreateEffectEvent((identifier, result)) = event;

        if let Ok(effect_id) = result {
            chroma_runner
                .registered_effects
                .insert(identifier, EffectId(effect_id));
        } else {
            error!("error while registering effect {:?}", identifier);
        }
    }
}

#[derive(Debug, Serialize)]
struct UseOneEffectRequest {
    id: String,
}

#[derive(Debug, Deserialize)]
struct UseOneEffectResponse {
    #[allow(unused)]
    result: u32, // todo: enum
}

fn run_use_effect_requests<T: EffectIdentifier>(mut chroma_runner: ResMut<ChromaRunner<T>>) {
    if chroma_runner.root_url.is_none() {
        info!("init not yet finished");
        return;
    }

    if chroma_runner.use_effect_requests.is_empty() {
        return;
    }

    let urls_with_futures = chroma_runner.use_effect_requests.iter().map(|identifier| {
        if let Some(EffectId(effect_id)) = chroma_runner.registered_effects.get(&identifier) {
            Some((
                "/effect",
                put::<UseOneEffectResponse>(
                    chroma_runner.to_full_url("/effect"),
                    UseOneEffectRequest {
                        id: effect_id.clone(),
                    },
                ),
            ))
        } else {
            error!("effect {:?} not registered... did it fail?", identifier);
            None
        }
    });

    let task_pool = AsyncComputeTaskPool::get();

    for (uri, fut) in urls_with_futures.flatten() {
        task_pool
            .spawn(async move {
                match fut.await {
                    Ok(_) => info!("{} request succeeded", uri),
                    Err(error) => error!("{} request failed: {:?}", uri, error),
                }
            })
            .detach();
    }

    chroma_runner.use_effect_requests.clear()
}
