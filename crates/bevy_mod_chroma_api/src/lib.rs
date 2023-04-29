use api::SessionInfo;
use bevy::prelude::Resource;
use serde::Serialize;

mod api;
mod plugin;

pub struct ChromaPlugin;

#[derive(Resource, Default)]
pub struct ChromaRunner {
    pub(crate) session_info: Option<SessionInfo>,
}

#[derive(Resource)]
pub struct ChromaRunnerInitializationSettings {
    init_url: &'static str,
    init_request: Init,
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

// TODO remove default impl
impl Default for Init {
    fn default() -> Self {
        Self {
            title: "Bevy Chroma",
            description: "Bevy Application",
            author: Author {
                name: "Your Name",
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
