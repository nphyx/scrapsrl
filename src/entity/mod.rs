mod character;
mod object;
mod entity_part;
pub mod body_layout;
pub use self::entity_part::EntityComponent;
pub use crate::entity::character::Character;
pub use crate::entity::object::Object;
use crate::display::DrawSelf;
use crate::game_state::GameState;

#[derive(Copy,Clone)]
pub struct Coord {
  pub x: i32,
  pub y: i32
}

impl std::cmp::PartialEq for Coord {
  fn eq(&self, &cmp: &Coord) -> bool {
    return self.x == cmp.x && self.y == cmp.y;
  }
}

pub trait Entity: DrawSelf {
  fn pos(&self) -> Coord;
  fn set_pos(&mut self, pos: Coord);
  fn tick(&mut self, state: &GameState);
}

pub type EntityCollection = Vec<Box<Entity>>;
