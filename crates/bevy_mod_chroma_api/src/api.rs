use crate::{bgr_color::BGRColor, key_color::KeyColor};
use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Serialize)]
#[serde(untagged)]
pub enum Effect {
    Mouse(MouseEffect),
    Keyboard(KeyboardEffect),
}

// https://assets.razerzone.com/dev_portal/REST/html/md__r_e_s_t_external_04_mouse.html
#[derive(Debug, Serialize)]
#[serde(tag = "effect", content = "param")]
pub enum MouseEffect {
    #[serde(rename(serialize = "CHROMA_NONE"))]
    None,
    #[serde(rename(serialize = "CHROMA_STATIC"))]
    Static { color: BGRColor },
    #[serde(rename(serialize = "CHROMA_CUSTOM2"))]
    Custom([[BGRColor; 7]; 9]),
}

// https://assets.razerzone.com/dev_portal/REST/html/md__r_e_s_t_external_03_keyboard.html
#[derive(Debug, Serialize)]
#[serde(tag = "effect", content = "param")]
pub enum KeyboardEffect {
    #[serde(rename(serialize = "CHROMA_NONE"))]
    None,
    #[serde(rename(serialize = "CHROMA_STATIC"))]
    Static { color: BGRColor },
    #[serde(rename(serialize = "CHROMA_CUSTOM2"))]
    Custom2 {
        color: [[BGRColor; 24]; 8],
        key: [[KeyColor; 22]; 6],
    },
}

impl Effect {
    pub(crate) fn get_api(&self) -> &'static str {
        match self {
            Effect::Mouse(_) => "mouse",
            Effect::Keyboard(_) => "keyboard",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateEffectResponse {
    #[serde(rename(deserialize = "result"))]
    _result: i32, // TODO enum
    id: String,
}

impl CreateEffectResponse {
    pub(crate) fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct SessionInfo {
    #[serde(rename(deserialize = "sessionid"))]
    _session_id: u32,
    #[serde(rename(deserialize = "uri"))]
    pub(crate) root_url: String,
}
