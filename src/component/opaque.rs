use specs::{Component, VecStorage};
/**
 * Opaque objects block vision.
 */
#[derive(Copy,Clone,Component,Debug,Default)]
#[storage(VecStorage)]
pub struct Opaque;
