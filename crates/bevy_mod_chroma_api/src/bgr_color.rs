use std::fmt::{Debug, Formatter, Result};

use bevy::prelude::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct BGRColor(u32);

impl BGRColor {
    #[must_use]
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    #[must_use]
    pub fn b(&self) -> u32 {
        (self.0 & 0xff0000) >> 16
    }

    #[must_use]
    pub fn g(&self) -> u32 {
        (self.0 & 0x00ff00) >> 8
    }

    #[must_use]
    pub fn r(&self) -> u32 {
        self.0 & 0x0000ff
    }
}

impl Debug for BGRColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "BGRColor({:#06X})", self.0)
    }
}

impl From<Color> for BGRColor {
    fn from(color: Color) -> Self {
        let r = (color.a() * color.r() * 255.0) as u32;
        let g = (color.a() * color.g() * 255.0) as u32;
        let b = (color.a() * color.b() * 255.0) as u32;

        Self((b << 16) | (g << 8) | r)
    }
}
