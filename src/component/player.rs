use specs::{Component, VecStorage};
use serde::{Deserialize,Serialize};

#[derive(Copy,Clone,Component,Debug,Default,Serialize,Deserialize)]
#[storage(VecStorage)]
/// A marker component for player characters.
pub struct Player;
