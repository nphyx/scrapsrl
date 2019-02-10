use specs::{Component, VecStorage};
use serde::{Deserialize,Serialize};

#[derive(Copy,Clone,Component,Debug,Default,Deserialize,Serialize)]
#[storage(VecStorage)]
pub struct Icon {
  pub ch: char
}
