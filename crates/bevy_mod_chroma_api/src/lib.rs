use bevy::prelude::Resource;
use reqwest::Url;
use serde::Serialize;

mod api;
mod bgr_color;
mod heartbeat;
mod plugin;

pub struct ChromaPlugin {
    settings: ChromaRunnerInitializationSettings,
}

impl ChromaPlugin {
    pub fn new(settings: ChromaRunnerInitializationSettings) -> Self {
        Self { settings }
    }
}

#[derive(Resource)]
pub struct ChromaRunner {
    pub(crate) root_url: Url,
}

impl ChromaRunner {
    pub(crate) fn get_session_url(&self, relative_path: &'static str) -> Url {
        // SAFETY: This is internal to the crate, so we assume that we aren't
        // going to be passing in bad URLs
        self.root_url.join(relative_path).unwrap()
    }
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
