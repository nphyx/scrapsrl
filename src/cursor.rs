use specs::{World, Entity, Builder};
// use tcod::{Console, BackgroundFlag};
use tcod::colors::Color;

// use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::component::{Position, Colors};
/*
use crate::display::DrawSelf;
use crate::entity::Position;
use crate::util::clamp;
*/

// TODO reimplement drawing a cursor

pub fn make_cursor_entity(world: &mut World, pos: Position) -> Entity {
  world.create_entity()
    .with(pos)
    .with(Colors{fg: Color{r: 128, g: 178, b: 128}, bg: Color{r: 128, g: 178, b: 128}})
    .build()
}

/*
#[derive(Default)]
pub struct Cursor {
  pub pos: Position,
  pub active: bool
}

impl Cursor {
  pub fn move_to(&mut self, to: Position) {
    self.pos.x = clamp(0, MAP_WIDTH, to.x);
    self.pos.y = clamp(0, MAP_HEIGHT, to.y);
  }
}

impl DrawSelf for Cursor {
  fn draw(&self, console: &mut Console) {
    console.set_char_background(
      self.pos.x,
      self.pos.y,
      Color,
      BackgroundFlag::Set);
  }
  fn draw_at(&self, console: &mut Console, x:i32, y:i32) {
    console.set_char_background(x, y, Color{r: 128, g: 178, b: 128}, BackgroundFlag::Set);
  }
}
*/
