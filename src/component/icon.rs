use specs::{Component, VecStorage};

#[derive(Copy,Clone,Component,Debug,Default)]
#[storage(VecStorage)]
pub struct Icon {
  pub ch: char
}