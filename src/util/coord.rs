// use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use num::Integer;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use std::ops::{AddAssign, SubAssign};

/**
 * A positional coordinate.
 */

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Hash, Default, Deserialize, Serialize, Ord, PartialOrd,
)]
pub struct Coord<Integer> {
    // y, x reversed here for row-wise ordering
    pub y: Integer,
    pub x: Integer,
}

impl Component for Coord<u32> {
    type Storage = VecStorage<Self>;
}

impl Component for Coord<i32> {
    type Storage = VecStorage<Self>;
}

impl Component for Coord<usize> {
    type Storage = VecStorage<Self>;
}

impl<T: Integer> Coord<T> {
    pub fn new(x: T, y: T) -> Coord<T> {
        Coord { x, y }
    }
}

impl<T: Integer + AddAssign> std::ops::AddAssign<Coord<T>> for Coord<T> {
    fn add_assign(&mut self, other: Coord<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Integer> std::ops::Add<Coord<T>> for Coord<T> {
    type Output = Coord<T>;
    fn add(self, other: Coord<T>) -> Self::Output {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Integer> std::ops::Sub<Coord<T>> for Coord<T> {
    type Output = Coord<T>;
    fn sub(self, pos: Coord<T>) -> Self::Output {
        Coord {
            x: self.x - pos.x,
            y: self.y - pos.y,
        }
    }
}

impl<T: Integer + SubAssign> std::ops::SubAssign<Coord<T>> for Coord<T> {
    fn sub_assign(&mut self, other: Coord<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl From<wfc::Coord> for Coord<usize> {
    fn from(other: wfc::Coord) -> Coord<usize> {
        assert!(other.x > 0);
        assert!(other.y > 0);
        Coord {
            x: other.x as usize,
            y: other.y as usize,
        }
    }
}

impl From<Coord<u32>> for wfc::Coord {
    fn from(coord: Coord<u32>) -> wfc::Coord {
        wfc::Coord {
            x: coord.x as i32,
            y: coord.y as i32,
        }
    }
}

impl From<Coord<i32>> for wfc::Coord {
    fn from(coord: Coord<i32>) -> wfc::Coord {
        wfc::Coord {
            x: coord.x,
            y: coord.y,
        }
    }
}

impl From<Coord<usize>> for wfc::Coord {
    fn from(coord: Coord<usize>) -> wfc::Coord {
        wfc::Coord {
            x: coord.x as i32,
            y: coord.y as i32,
        }
    }
}

impl From<Coord<usize>> for Coord<i32> {
    fn from(coord: Coord<usize>) -> Coord<i32> {
        Coord {
            x: coord.x as i32,
            y: coord.y as i32,
        }
    }
}

impl From<Coord<i32>> for Coord<usize> {
    fn from(coord: Coord<i32>) -> Coord<usize> {
        assert!(coord.x >= 0);
        assert!(coord.y >= 0);
        Coord {
            x: coord.x as usize,
            y: coord.y as usize,
        }
    }
}
