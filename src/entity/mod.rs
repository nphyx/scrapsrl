mod character;
mod object;
pub use crate::entity::character::Character;
pub use crate::entity::object::Object;
use crate::display::DrawSelf;

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
}

pub type EntityCollection = Vec<Box<Entity>>;
