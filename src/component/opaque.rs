use specs::{Component, VecStorage};
use serde::Serialize;

/**
 * Opaque objects block vision.
 */
#[derive(Copy,Clone,Component,Debug,Default,Serialize)]
#[storage(VecStorage)]
pub struct Opaque;
