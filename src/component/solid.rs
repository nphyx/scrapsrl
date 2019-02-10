use specs::{Component, VecStorage};
use serde::Serialize;
/**
 * Solid objects block movement.
 */
#[derive(Copy,Clone,Component,Debug,Default,Serialize)]
#[storage(VecStorage)]
pub struct Solid;
