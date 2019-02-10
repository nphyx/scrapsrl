use specs::{Component, VecStorage};
use serde::Serialize;

#[derive(Copy,Clone,Debug,Hash,Default,Component,Serialize)]
#[storage(VecStorage)]
pub struct MovePlan {
  pub x: i32,
  pub y: i32
}
