use specs::{Component, VecStorage};
use serde::{Deserialize,Serialize};
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::util::clamp;
use super::Position; // ha!

#[derive(Copy,Clone,Debug,Hash,Default,Component,Deserialize,Serialize)]
#[storage(VecStorage)]
pub struct MovePlan {
  pub x: i32,
  pub y: i32
}

impl std::ops::AddAssign<MovePlan> for MovePlan {
  fn add_assign(&mut self, coord: MovePlan) {
    self.x = clamp(0, MAP_WIDTH, self.x + coord.x);
    self.y = clamp(0, MAP_HEIGHT, self.y + coord.y);
  }
}

impl std::ops::Add<MovePlan> for MovePlan {
  type Output = MovePlan;
  fn add(self, coord: MovePlan) -> MovePlan {
    MovePlan{x: self.x + coord.x, y: self.y + coord.y}
  }
}

impl std::ops::Add<Position> for MovePlan {
  type Output = Position;
  fn add(self, coord: Position) -> Position {
    Position{x: self.x + coord.x, y: self.y + coord.y}
  }
}

impl std::cmp::PartialEq<Position> for MovePlan {
  fn eq(&self, &cmp: &Position) -> bool {
    return self.x == cmp.x && self.y == cmp.y;
  }
}

impl std::cmp::PartialEq<MovePlan> for MovePlan {
  fn eq(&self, &cmp: &MovePlan) -> bool {
    return self.x == cmp.x && self.y == cmp.y;
  }
}

impl std::cmp::Eq for MovePlan {}
