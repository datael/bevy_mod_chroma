use std::fmt::{Debug, Formatter, Result};

use bevy::prelude::Color;
use serde::Serialize;

use crate::bgr_color::BGRColor;

const KEY_COLOR_MASK: u32 = 0x0100_0000;

#[derive(Serialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct KeyColor(u32);

impl Debug for KeyColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "KeyColor({:#06X}|{})",
            self.0 & !KEY_COLOR_MASK,
            (self.0 & KEY_COLOR_MASK) >> 24,
        )
    }
}

impl From<BGRColor> for KeyColor {
    fn from(color: BGRColor) -> Self {
        let color_value = color.as_u32();

        Self(KEY_COLOR_MASK | color_value)
    }
}

impl From<Option<BGRColor>> for KeyColor {
    fn from(maybe_color: Option<BGRColor>) -> Self {
        match maybe_color {
            Some(color) => Self::from(color),
            None => Self(0),
        }
    }
}

impl From<Color> for KeyColor {
    fn from(color: Color) -> Self {
        Self::from(BGRColor::from(color))
    }
}

impl From<Option<Color>> for KeyColor {
    fn from(maybe_color: Option<Color>) -> Self {
        Self::from(maybe_color.map(|color| BGRColor::from(color)))
    }
}
