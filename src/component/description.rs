use specs::{Component, VecStorage};
use serde::Serialize;

#[derive(Clone,Component,Debug,Default,Serialize)]
#[storage(VecStorage)]
pub struct Description { 
  pub short: String,
  pub long: String
}
