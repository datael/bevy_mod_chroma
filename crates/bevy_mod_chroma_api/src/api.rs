use crate::{bgr_color::BGRColor, key_color::KeyColor};
use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(
    Component, Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,
)]
#[serde(untagged)]
pub enum Effect {
    Keyboard(KeyboardEffect),
    Mouse(MouseEffect),
    Mousepad(MousepadEffect),
    Headset(HeadsetEffect),
    Keypad(KeypadEffect),
    ChromaLink(ChromaLinkEffect),
}

// https://assets.razerzone.com/dev_portal/REST/html/md__r_e_s_t_external_03_keyboard.html
#[allow(clippy::large_enum_variant)]
#[derive(
    Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default,
)]
#[serde(tag = "effect", content = "param")]
pub enum KeyboardEffect {
    #[default]
    #[serde(rename = "CHROMA_NONE")]
    None,
    #[serde(rename = "CHROMA_STATIC")]
    Static { color: BGRColor },
    #[serde(rename = "CHROMA_CUSTOM")]
    Custom([[BGRColor; 22]; 6]),
    #[serde(rename = "CHROMA_CUSTOM2")]
    Custom2 {
        color: [[BGRColor; 24]; 8],
        key: [[KeyColor; 22]; 6],
    },
    #[serde(rename = "CHROMA_CUSTOM_KEY")]
    CustomKey {
        color: [[BGRColor; 22]; 6],
        key: [[KeyColor; 22]; 6],
    },
}

// https://assets.razerzone.com/dev_portal/REST/html/md__r_e_s_t_external_04_mouse.html
#[allow(clippy::large_enum_variant)]
#[derive(
    Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default,
)]
#[serde(tag = "effect", content = "param")]
pub enum MouseEffect {
    #[default]
    #[serde(rename = "CHROMA_NONE")]
    None,
    #[serde(rename = "CHROMA_STATIC")]
    Static { color: BGRColor },
    #[serde(rename = "CHROMA_CUSTOM2")]
    Custom([[BGRColor; 7]; 9]),
}

// https://assets.razerzone.com/dev_portal/REST/html/md__r_e_s_t_external_05_mousemat.html
#[allow(clippy::large_enum_variant)]
#[derive(
    Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default,
)]
#[serde(tag = "effect", content = "param")]
pub enum MousepadEffect {
    #[default]
    #[serde(rename = "CHROMA_NONE")]
    None,
    #[serde(rename = "CHROMA_STATIC")]
    Static { color: BGRColor },
    #[serde(rename = "CHROMA_CUSTOM")]
    Custom([BGRColor; 15]),
}

// https://assets.razerzone.com/dev_portal/REST/html/md__r_e_s_t_external_06_headset.html
#[allow(clippy::large_enum_variant)]
#[derive(
    Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default,
)]
#[serde(tag = "effect", content = "param")]
pub enum HeadsetEffect {
    #[default]
    #[serde(rename = "CHROMA_NONE")]
    None,
    #[serde(rename = "CHROMA_STATIC")]
    Static { color: BGRColor },
    #[serde(rename = "CHROMA_CUSTOM")]
    Custom([BGRColor; 5]),
}

// https://assets.razerzone.com/dev_portal/REST/html/md__r_e_s_t_external_07_keypad.html
#[allow(clippy::large_enum_variant)]
#[derive(
    Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default,
)]
#[serde(tag = "effect", content = "param")]
pub enum KeypadEffect {
    #[default]
    #[serde(rename = "CHROMA_NONE")]
    None,
    #[serde(rename = "CHROMA_STATIC")]
    Static { color: BGRColor },
    #[serde(rename = "CHROMA_CUSTOM")]
    Custom([[BGRColor; 5]; 4]),
}

// https://assets.razerzone.com/dev_portal/REST/html/md__r_e_s_t_external_08_chromalink.html
#[allow(clippy::large_enum_variant)]
#[derive(
    Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default,
)]
#[serde(tag = "effect", content = "param")]
pub enum ChromaLinkEffect {
    #[default]
    #[serde(rename = "CHROMA_NONE")]
    None,
    #[serde(rename = "CHROMA_STATIC")]
    Static { color: BGRColor },
    #[serde(rename = "CHROMA_CUSTOM")]
    Custom([BGRColor; 5]),
}

impl Effect {
    #[must_use]
    pub(crate) fn get_api(&self) -> &'static str {
        match self {
            Effect::Keyboard(_) => "keyboard",
            Effect::Mouse(_) => "mouse",
            Effect::Mousepad(_) => "mousepad",
            Effect::Headset(_) => "headset",
            Effect::Keypad(_) => "keypad",
            Effect::ChromaLink(_) => "chromalink",
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct CreateEffectResponse {
    #[serde(rename(deserialize = "result"))]
    _result: i32, // TODO enum
    id: String,
}

impl CreateEffectResponse {
    #[must_use]
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
