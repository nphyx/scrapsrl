use specs::{Component, VecStorage};
use serde::Serialize;

#[derive(Copy,Clone,Component,Debug,Default,Serialize)]
#[storage(VecStorage)]
pub struct Icon {
  pub ch: char
}
