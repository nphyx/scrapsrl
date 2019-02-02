use tcod::{Console, BackgroundFlag};
use tcod::colors::Color;
use crate::entity::Coord;
use crate::display::DrawSelf;
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::util::clamp;

pub struct Cursor {
  pub pos: Coord,
  pub active: bool
}

impl Cursor {
  pub fn move_to(&mut self, to: Coord) {
    self.pos.x = clamp(0, MAP_WIDTH, to.x);
    self.pos.y = clamp(0, MAP_HEIGHT, to.y);
  }
}

impl DrawSelf for Cursor {
  fn draw(&self, console: &mut Console) {
    println!("drawing cursor");
    console.set_char_background(
      self.pos.x,
      self.pos.y,
      Color{r: 128, g: 178, b: 128},
      BackgroundFlag::Set);
  }
}
