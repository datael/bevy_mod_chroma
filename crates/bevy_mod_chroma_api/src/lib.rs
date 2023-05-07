use std::time::Duration;

use api::Effect;
use bevy::{
    ecs::system::SystemParam,
    prelude::{Commands, Entity, Resource},
    utils::Instant,
};
use plugin::ApplyEffectRequest;
use reqwest::Url;
use serde::{Deserialize, Serialize};

pub mod api;
pub mod bgr_color;
pub mod key_color;

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

#[derive(SystemParam)]
pub struct Chroma<'w, 's> {
    commands: Commands<'w, 's>,
}

impl<'w, 's> Chroma<'w, 's> {
    pub fn create_effect(&mut self, effect: Effect) -> EffectHandle {
        EffectHandle {
            entity: self.commands.spawn(effect).id(),
        }
    }

    pub fn apply_effect_with_deadline(&mut self, effect_handle: &EffectHandle, deadline: Instant) {
        self.commands.spawn(ApplyEffectRequest {
            effect_entity: effect_handle.entity(),
            deadline,
        });
    }

    pub fn apply_effect(&mut self, effect_handle: &EffectHandle) {
        self.apply_effect_with_deadline(effect_handle, Instant::now() + Duration::from_secs(60));
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct EffectHandle {
    entity: Entity,
}

impl EffectHandle {
    pub fn entity(&self) -> Entity {
        self.entity
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

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InitRequest {
    pub title: &'static str,
    pub description: &'static str,
    pub author: Author,
    pub device_supported: Vec<&'static str>,
    pub category: &'static str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Author {
    pub name: &'static str,
    pub contact: &'static str,
}
