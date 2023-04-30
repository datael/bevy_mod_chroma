use bevy::prelude::Color;
use serde::Serialize;

use crate::bgr_color::BGRColor;

#[derive(Debug, Serialize, Clone, Copy)]
pub struct KeyColor(u32);

impl From<Color> for KeyColor {
    fn from(color: Color) -> Self {
        const KEY_COLOR_MASK: u32 = 0x0100_0000;
        let color_value = BGRColor::from(color).as_u32();

        Self(KEY_COLOR_MASK | color_value)
    }
}

impl From<Option<Color>> for KeyColor {
    fn from(maybe_color: Option<Color>) -> Self {
        match maybe_color {
            Some(color) => Self::from(color),
            None => Self(0),
        }
    }
}
