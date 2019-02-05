use specs::{Component, VecStorage};

#[derive(Clone,Component,Debug,Default)]
#[storage(VecStorage)]
pub struct Description { 
  pub short: String,
  pub long: String
}
