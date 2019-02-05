use specs::{Component, VecStorage};

#[derive(Copy,Clone,Component,Debug,Default)]
#[storage(VecStorage)]
/// A marker component for tiles that have already been processed for connectivity.
pub struct ConnectedTile;
