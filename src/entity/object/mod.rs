use tcod::{Console, BackgroundFlag};
use tcod::colors::Color;
use crate::entity::{Entity, Coord};
use crate::display::DrawSelf;
use crate::game_state::GameState;

pub struct Object {
  pos: Coord,
  ch: char,
  color: Color
}

impl Object {
  pub fn new() -> Object {
    return Object{
      pos: Coord{x: 0, y: 0},
      ch: '!',
      color: Color{r: 128, g: 128, b: 128}
    }
  }

  pub fn set_ch(&mut self, ch: char) {
    self.ch = ch;
  }

}

impl Entity for Object {
  fn pos(&self) -> Coord { self.pos }
  fn set_pos(&mut self, pos: Coord) { self.pos = pos }
  fn tick(&mut self, _state: &GameState) {}
}

impl DrawSelf for Object {
  fn draw(&self, console: &mut Console) {
    console.put_char(self.pos().x, self.pos().y, self.ch, BackgroundFlag::None);
  }
}
