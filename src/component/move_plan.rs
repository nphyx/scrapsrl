use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::util::{clamp, Coord};
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Copy, Clone, Component, Debug, Default, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[storage(VecStorage)]
pub struct MovePlan {
    pub x: i32,
    pub y: i32,
}

impl std::ops::AddAssign<MovePlan> for MovePlan {
    fn add_assign(&mut self, coord: MovePlan) {
        self.x = clamp(0, MAP_WIDTH as i32, self.x + coord.x);
        self.y = clamp(0, MAP_HEIGHT as i32, self.y + coord.y);
    }
}

impl std::ops::Add<MovePlan> for MovePlan {
    type Output = MovePlan;
    fn add(self, coord: MovePlan) -> MovePlan {
        MovePlan {
            x: self.x + coord.x,
            y: self.y + coord.y,
        }
    }
}

impl std::ops::Add<Coord<usize>> for MovePlan {
    type Output = Coord<usize>;
    fn add(self, coord: Coord<usize>) -> Coord<usize> {
        let new_x = self.x + coord.x as i32;
        let new_y = self.y + coord.y as i32;
        assert!(new_x >= 0);
        assert!(new_y >= 0);
        Coord {
            x: new_x as usize,
            y: new_y as usize,
        }
    }
}

impl std::cmp::PartialEq<Coord<usize>> for MovePlan {
    fn eq(&self, &cmp: &Coord<usize>) -> bool {
        self.x == cmp.x as i32 && self.y == cmp.y as i32
    }
}

impl std::convert::From<Coord<usize>> for MovePlan {
    fn from(coord: Coord<usize>) -> MovePlan {
        MovePlan {
            x: coord.x as i32,
            y: coord.y as i32,
        }
    }
}
