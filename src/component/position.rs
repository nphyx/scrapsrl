extern crate specs;
use specs::{Component, VecStorage};
/**
 * A coordinate.
 */

#[derive(Copy,Clone,Debug,Hash)]
pub struct Position {
  pub x: i32,
  pub y: i32
}

impl Component for Position {
  type Storage = VecStorage<Self>;
}
