use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
#[derive(Component, Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Region {
    pub x: i32,
    pub y: i32,
}

use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
impl Region {
    pub fn new(x: i32, y: i32) -> Region {
        Region { x, y }
    }

    /* FIXME unused maybe not needed? replaced with to_offset
    pub fn to_array(self) -> [i32; 2] {
      [self.x, self.y]
    }
    */

    // used in map generation
    pub fn to_offset(self) -> [i32; 2] {
        [self.x * MAP_WIDTH, self.y * MAP_HEIGHT]
    }
}
