use api::SessionInfo;
use bevy::prelude::Resource;
use serde::Serialize;

mod api;
mod plugin;

pub struct ChromaPlugin {
    settings: ChromaRunnerInitializationSettings,
}

impl ChromaPlugin {
    pub fn new(settings: ChromaRunnerInitializationSettings) -> Self {
        Self { settings }
    }
}

#[derive(Resource, Default)]
pub struct ChromaRunner {
    pub(crate) session_info: Option<SessionInfo>,
}

#[derive(Resource, Debug, Clone)]
pub struct ChromaRunnerInitializationSettings {
    init_url: &'static str,
    init_request: InitRequest,
}

impl ChromaRunnerInitializationSettings {
    pub fn new(init_request: InitRequest) -> Self {
        Self::new_with_init_url("http://localhost:54235/razer/chromasdk", init_request)
    }

    pub fn new_with_init_url(init_url: &'static str, init_request: InitRequest) -> Self {
        Self {
            init_url,
            init_request,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct InitRequest {
    pub title: &'static str,
    pub description: &'static str,
    pub author: Author,
    pub device_supported: Vec<&'static str>,
    pub category: &'static str,
}

#[derive(Debug, Serialize, Clone)]
pub struct Author {
    pub name: &'static str,
    pub contact: &'static str,
}
