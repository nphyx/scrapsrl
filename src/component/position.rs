/*
*/
use super::MovePlan;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::util::clamp;
use crate::util::Coord;
/*
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
*/

/**
 * A positional coordinate.
 */

pub type Pos = Coord<usize>;

impl Pos {
    pub fn to_array(&self) -> [i32; 2] {
        [self.x as i32, self.y as i32]
    }
}

impl std::ops::Add<MovePlan> for Pos {
    type Output = Pos;
    fn add(self, other: MovePlan) -> Self::Output {
        Coord {
            x: self.x + other.x as usize,
            y: self.y + other.y as usize,
        }
    }
}

impl std::cmp::PartialEq<MovePlan> for Pos {
    fn eq(&self, &cmp: &MovePlan) -> bool {
        self.x == cmp.x as usize && self.y == cmp.y as usize
    }
}

impl From<MovePlan> for Pos {
    fn from(plan: MovePlan) -> Pos {
        Pos {
            x: clamp(0, MAP_WIDTH as i32, plan.x) as usize,
            y: clamp(0, MAP_HEIGHT as i32, plan.x) as usize,
        }
    }
}

impl From<[i32; 2]> for Pos {
    fn from(arr: [i32; 2]) -> Pos {
        assert!(arr[0] >= 0);
        assert!(arr[0] >= 0);
        Pos {
            x: arr[0] as usize,
            y: arr[1] as usize,
        }
    }
}
