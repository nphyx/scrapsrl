use specs::{Component, VecStorage};
/**
 * Solid objects block movement.
 */
#[derive(Copy,Clone,Component,Debug,Default)]
#[storage(VecStorage)]
pub struct Solid;
