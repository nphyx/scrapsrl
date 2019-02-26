use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Component, Deserialize, Serialize)]
#[storage(VecStorage)]
pub struct Orientation {
    pub dir: Direction,
}

impl Default for Orientation {
    fn default() -> Orientation {
        Orientation {
            dir: Direction::South,
        }
    }
}

impl Orientation {
    pub fn new(dir: Direction) -> Orientation {
        Orientation { dir }
    }
}
