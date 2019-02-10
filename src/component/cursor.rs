use specs::{Component,VecStorage};
use serde::Serialize;

#[derive(Copy,Clone,Default,Component,Serialize)]
#[storage(VecStorage)]
/// a marker for player-owned cursors
pub struct Cursor;
