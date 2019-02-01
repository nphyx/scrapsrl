use tcod::{Console, BackgroundFlag};
use crate::entity::{Entity, Coord};
use crate::display::DrawSelf;

pub struct Object {
  pos: Coord,
  ch: char
}

impl Object {
  pub fn new() -> Object {
    return Object{
      pos: Coord{x: 0, y: 0},
      ch: '!'
    }
  }

  pub fn set_ch(&mut self, ch: char) {
    self.ch = ch;
  }

}

impl Entity for Object {
  fn pos(&self) -> Coord { self.pos }
  fn set_pos(&mut self, pos: Coord) { self.pos = pos }
}

impl DrawSelf for Object {
  fn draw(&self, console: &mut Console) {
    console.put_char(self.pos().x, self.pos().y, self.ch, BackgroundFlag::None);
  }
}
