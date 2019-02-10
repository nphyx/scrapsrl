use specs::{Component, VecStorage};
use serde::{Deserialize,Serialize};
/**
 * Solid objects block movement.
 */
#[derive(Copy,Clone,Component,Debug,Default,Deserialize,Serialize)]
#[storage(VecStorage)]
pub struct Solid;
