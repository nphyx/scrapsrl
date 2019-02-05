use specs::{Component, VecStorage};

#[derive(Copy,Clone,Component,Debug,Default)]
#[storage(VecStorage)]
/// A marker component for player characters.
pub struct Player;
