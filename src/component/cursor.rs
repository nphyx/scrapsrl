use specs::{Component,VecStorage};

#[derive(Copy,Clone,Default,Component)]
#[storage(VecStorage)]
/// a marker for player-owned cursors
pub struct Cursor;
