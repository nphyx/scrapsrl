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

#[derive(Copy, Clone, Component, Default, Debug, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Colors {
    pub fg: Color,
    pub bg: Color,
}

/*
impl Serialize for Colors {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let mut s = serializer.serialize_struct("Colors", 6)?;
    s.serialize_field("fg_r", &self.fg.r)?;
    s.serialize_field("fg_g", &self.fg.g)?;
    s.serialize_field("fg_b", &self.fg.b)?;
    s.serialize_field("bg_r", &self.bg.r)?;
    s.serialize_field("bg_g", &self.bg.g)?;
    s.serialize_field("bg_b", &self.bg.b)?;
    s.end()
  }
}
*/
