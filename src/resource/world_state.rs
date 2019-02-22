use rand::prelude::*;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::component::Region;
use crate::constants::*;

#[derive(Copy, Clone, Debug)]
pub struct RoadTile {
    /// the number of lanes for a longitudinal road on this tile
    /// (0 = no road)
    pub lanes_x: u8,
    /// the number of lanes for a latitudinal road on this tile
    /// (0 = no road)
    pub lanes_y: u8,
}

impl Default for RoadTile {
    fn default() -> RoadTile {
        RoadTile {
            lanes_x: 0,
            lanes_y: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct RoadMap {
    tiles: [[RoadTile; WORLD_SIZE]; WORLD_SIZE],
}

impl Default for RoadMap {
    fn default() -> RoadMap {
        RoadMap {
            tiles: [[RoadTile::default(); WORLD_SIZE]; WORLD_SIZE],
        }
    }
}

#[derive(Copy, Clone)]
pub struct PopMap {
    samples: [[f32; WORLD_SIZE]; WORLD_SIZE],
}

impl Default for PopMap {
    fn default() -> PopMap {
        PopMap {
            samples: [[0.0; WORLD_SIZE]; WORLD_SIZE],
        }
    }
}

#[derive(Copy, Clone, Component, Deserialize, Serialize)]
#[storage(VecStorage)]
pub struct WorldState {
    /// global RNG seed
    seed: u32,

    /// day of year
    pub day: u32,
    /// time of day in decimal form
    pub time: f32,
    /// years since apocalypse
    pub year: u32,

    size: u32,

    #[serde(skip)]
    /// map of population density, deterministic so skipped when reloading game
    pub pop: PopMap,

    #[serde(skip)]
    /// map of roads, deterministic so skipped when reloading game
    pub roads: RoadMap,

    /// true when the world is ready to be used (after init)
    pub ready: bool,
}

impl Default for WorldState {
    fn default() -> WorldState {
        assert!(WORLD_SIZE % 2 == 0, "world size must be a multiple of 2!");
        let mut rng = rand::thread_rng();
        WorldState {
            seed: rng.gen_range(0, std::u32::MAX),
            day: 0,
            time: 6.0,
            year: 70,
            pop: PopMap::default(),
            roads: RoadMap::default(),
            size: WORLD_SIZE as u32,
            ready: false,
        }
    }
}

impl WorldState {
    /// needs a getter because changing this causes the world to change ready state
    pub fn seed(&self) -> u32 {
        self.seed.clone()
    }

    /// needs a setter because changing this causes the world to change ready state
    pub fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
        self.ready = false;
    }

    /// 1.0 is noon, 0.0 is midnight
    pub fn time_relative(&self) -> f32 {
        ((self.time * 15.0 * (std::f32::consts::PI / 180.0)).sin() + 1.0) / 2.0
    }

    pub fn to_abs_pos(&self, region: Region) -> (usize, usize) {
        let x = region.x + (self.size as i32 / 2);
        let y = region.y + (self.size as i32 / 2);
        assert!(x >= 0, "got a negative x in set_road");
        assert!(y >= 0, "got a negative y in set_road");
        assert!((x as usize) < WORLD_SIZE, "got an oversized x in set_road");
        assert!((y as usize) < WORLD_SIZE, "got an oversized y in set_road");
        (x as usize, y as usize)
    }

    pub fn set_pop(&mut self, region: Region, density: f32) {
        let (x, y) = self.to_abs_pos(region);
        self.pop.samples[x as usize][y as usize] = density;
    }

    pub fn get_pop(&self, region: Region) -> f32 {
        let (x, y) = self.to_abs_pos(region);
        self.pop.samples[x as usize][y as usize]
    }

    pub fn get_road(&self, region: Region) -> RoadTile {
        let (x, y) = self.to_abs_pos(region);
        self.roads.tiles[x as usize][y as usize]
    }

    pub fn set_road(&mut self, region: Region, lanes_x: u8, lanes_y: u8) {
        let (x, y) = self.to_abs_pos(region);
        self.roads.tiles[x][y].lanes_x = lanes_x;
        self.roads.tiles[x][y].lanes_y = lanes_y;
    }

    pub fn min_x(&self) -> i32 {
        -(self.size as i32 / 2) as i32
    }

    pub fn max_x(&self) -> i32 {
        (self.size as i32 / 2) as i32
    }

    pub fn min_y(&self) -> i32 {
        -(self.size as i32 / 2) as i32
    }

    pub fn max_y(&self) -> i32 {
        (self.size as i32 / 2) as i32
    }
}
