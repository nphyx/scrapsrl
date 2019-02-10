use specs::{Component, VecStorage};
use serde::{Deserialize,Serialize};

/**
 * Opaque objects block vision.
 */
#[derive(Copy,Clone,Component,Debug,Default,Deserialize,Serialize)]
#[storage(VecStorage)]
pub struct Opaque;
