use super::MovePlan;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::util::clamp;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

/**
 * A positional coordinate.
 */

#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Default,
    Component,
    Deserialize,
    Serialize,
    Ord,
    PartialOrd,
)]
#[storage(VecStorage)]
pub struct Position {
    // y, x reversed here for row-wise ordering
    pub y: i32,
    pub x: i32,
}

impl std::ops::AddAssign<Position> for Position {
    fn add_assign(&mut self, coord: Position) {
        self.x = clamp(0, MAP_WIDTH, self.x + coord.x);
        self.y = clamp(0, MAP_HEIGHT, self.y + coord.y);
    }
}

impl std::ops::Add<Position> for Position {
    type Output = Position;
    fn add(self, coord: Position) -> Self::Output {
        Position {
            x: self.x + coord.x,
            y: self.y + coord.y,
        }
    }
}

impl std::ops::Add<MovePlan> for Position {
    type Output = Position;
    fn add(self, coord: MovePlan) -> Self::Output {
        Position {
            x: self.x + coord.x,
            y: self.y + coord.y,
        }
    }
}

impl std::ops::Sub<Position> for Position {
    type Output = Position;
    fn sub(self, pos: Position) -> Self::Output {
        Position {
            x: self.x - pos.x,
            y: self.y - pos.y,
        }
    }
}

impl std::cmp::PartialEq<MovePlan> for Position {
    fn eq(&self, &cmp: &MovePlan) -> bool {
        self.x == cmp.x && self.y == cmp.y
    }
}

use wfc::Coord;
impl From<Coord> for Position {
    fn from(coord: Coord) -> Position {
        Position {
            x: coord.x,
            y: coord.y,
        }
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
    pub fn to_array(self) -> [i32; 2] {
        [self.x, self.y]
    }
}
