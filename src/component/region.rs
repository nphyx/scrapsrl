use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
#[derive(Component, Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Region {
    pub x: i32,
    pub y: i32,
}

use crate::constants::{MAP_HEIGHT, MAP_WIDTH, WORLD_SIZE};
impl Region {
    pub fn new(x: i32, y: i32) -> Region {
        Region { x, y }
    }

    // used in map generation
    pub fn to_offset(self) -> [i32; 2] {
        [self.x * MAP_WIDTH, self.y * MAP_HEIGHT]
    }

    // used in map generation
    pub fn to_unsigned(self) -> [u64; 2] {
        [
            (self.x + (WORLD_SIZE as i32 / 2)) as u64,
            (self.y + (WORLD_SIZE as i32 / 2)) as u64,
        ]
    }
}
