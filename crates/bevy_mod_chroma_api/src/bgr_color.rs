use bevy::prelude::Color;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
pub struct BGRColor(u32);

impl From<Color> for BGRColor {
    fn from(color: Color) -> Self {
        let r = (color.r() * 255.0) as u32;
        let g = (color.g() * 255.0) as u32;
        let b = (color.b() * 255.0) as u32;

        Self((b << 16) | (g << 8) | r)
    }
}
