use specs::{Component, VecStorage};
use serde::{Serialize,Deserialize};
#[derive(Component,Copy,Clone,Debug,Default,Hash,Serialize,Deserialize)]
#[storage(VecStorage)]
pub struct Region {
  pub x: i32,
  pub y: i32 
}

impl std::cmp::PartialEq for Region {
  fn eq(&self, other: &Region) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl std::cmp::Eq for Region {}

use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
impl Region {
  pub fn new(x: i32, y: i32) -> Region {
    Region{x, y}
  }

  pub fn to_array(&self) -> [i32; 2] {
    [self.x, self.y]
  }

  // used in map generation
  pub fn to_offset(&self) -> [i32; 2] {
    [self.x * MAP_WIDTH, self.y * MAP_HEIGHT]
  }
}
