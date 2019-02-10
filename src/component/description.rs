use specs::{Component, VecStorage};
use serde::{Serialize,Deserialize};

#[derive(Clone,Component,Debug,Default,Serialize,Deserialize)]
#[storage(VecStorage)]
pub struct Description { 
  pub short: String,
  pub long: String
}
