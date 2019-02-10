use specs::{Component, VecStorage};
use serde::{Serialize, ser::{Serializer, SerializeStruct}};

#[derive(Copy,Clone,Component,Default,Debug)]
#[storage(VecStorage)]
pub struct Colors {
  pub fg: tcod::colors::Color,
  pub bg: tcod::colors::Color
}

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
