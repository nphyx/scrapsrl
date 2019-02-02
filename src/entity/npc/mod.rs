use tcod::{Console};
use super::{Character, Coord, Entity, behavior};
use crate::display::DrawSelf;
use crate::game_state::GameState;

pub struct NPC {
  pub character: Character,
  pub behavior: &'static behavior::Behavior,
  target: Option<&'static Entity>
}

impl NPC {
  pub fn new(character: Character) -> NPC {
    NPC{character, target: None, behavior: &behavior::MovementBehavior::BrownianWalk}
  }
}

impl Entity for NPC {
  fn pos(&self) -> Coord {
    self.character.pos()
  }
  fn set_pos(&mut self, coord: Coord) {
    self.character.set_pos(coord)
  }
  fn tick(&mut self, state: &GameState) {
    self.behavior.execute(self, state);
    self.character.tick(state);
  }
}

impl DrawSelf for NPC {
  fn draw(&self, console: &mut Console) {
    self.character.draw(console);
  }
}
