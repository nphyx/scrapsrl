use specs::{Component, VecStorage};
use serde::Serialize;
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::util::clamp;

/**
 * A positional coordinate.
 */

#[derive(Copy,Clone,Debug,Hash,Default,Component,Serialize)]
#[storage(VecStorage)]
pub struct Position {
  pub x: i32,
  pub y: i32
}

impl std::ops::AddAssign<Position> for Position {
  fn add_assign(&mut self, coord: Position) {
    self.x = clamp(0, MAP_WIDTH, self.x + coord.x);
    self.y = clamp(0, MAP_HEIGHT, self.y + coord.y);
  }
}

impl std::ops::Add<Position> for Position {
  type Output = Position;
  fn add(self, coord: Position) -> Position {
    Position{x: self.x + coord.x, y: self.y + coord.y}
  }
}

impl std::cmp::PartialEq for Position {
  fn eq(&self, &cmp: &Position) -> bool {
    return self.x == cmp.x && self.y == cmp.y;
  }
}

impl std::cmp::Eq for Position {}

impl Position {
  pub fn to_array(&self) -> [i32; 2] {
    [self.x, self.y]
  }
}
