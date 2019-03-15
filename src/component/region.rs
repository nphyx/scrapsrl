use crate::util::Coord;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Copy, Clone, Hash, Debug, Default, Deserialize, PartialEq, Eq, Serialize, Component)]
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
        [self.x * MAP_WIDTH as i32, self.y * MAP_HEIGHT as i32]
    }

    // used in map generation
    pub fn to_unsigned(self) -> [u64; 2] {
        [
            (self.x + (WORLD_SIZE as i32 / 2)) as u64,
            (self.y + (WORLD_SIZE as i32 / 2)) as u64,
        ]
    }
}

impl From<Region> for Coord<i32> {
    fn from(region: Region) -> Coord<i32> {
        Coord {
            x: region.x,
            y: region.y,
        }
    }
}

impl From<Coord<i32>> for Region {
    fn from(coord: Coord<i32>) -> Region {
        Region {
            x: coord.x,
            y: coord.y,
        }
    }
}
