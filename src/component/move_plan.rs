use specs::{Component, VecStorage};

#[derive(Copy,Clone,Debug,Hash,Default,Component)]
#[storage(VecStorage)]
pub struct MovePlan {
  pub x: i32,
  pub y: i32
}
