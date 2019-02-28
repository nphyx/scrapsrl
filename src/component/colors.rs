use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage}; //, ser::{Serializer, SerializeStruct}};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Color {
    fn default() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn to_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl From<tcod::colors::Color> for Color {
    fn from(color: tcod::colors::Color) -> Color {
        Color {
            r: color.r,
            g: color.g,
            b: color.b,
        }
    }
}

impl From<Color> for tcod::colors::Color {
    fn from(color: Color) -> tcod::colors::Color {
        tcod::colors::Color {
            r: color.r,
            g: color.g,
            b: color.b,
        }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(color: (u8, u8, u8)) -> Color {
        Color {
            r: color.0,
            g: color.1,
            b: color.2,
        }
    }
}

#[derive(Copy, Clone, Component, Default, Debug, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Colors {
    pub fg: Color,
    pub bg: Color,
}
