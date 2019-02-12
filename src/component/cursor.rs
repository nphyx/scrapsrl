use serde::Serialize;
use specs::{Component, VecStorage};

#[derive(Copy, Clone, Default, Component, Serialize)]
#[storage(VecStorage)]
/// a marker for player-owned cursors
pub struct Cursor;
