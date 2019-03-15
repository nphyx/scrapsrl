use rand::prelude::*;
use rand_pcg::*;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
use tcod::noise::*;

use crate::component::Region;
use crate::constants::*;
use crate::resource::{Assets, GeographyTemplate};
use crate::util::*;

#[derive(Copy, Clone, Debug)]
pub struct RoadTile {
    /// the number of lanes for a longitudinal road on this tile
    /// (0 = no road)
    pub lanes_x: u8,
    /// the number of lanes for a latitudinal road on this tile
    /// (0 = no road)
    pub lanes_y: u8,
}

/// a road tile specifying the x and y lanes of a road
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

/// a map of road tiles
impl Default for RoadMap {
    fn default() -> RoadMap {
        RoadMap {
            tiles: [[RoadTile::default(); WORLD_SIZE]; WORLD_SIZE],
        }
    }
}

#[derive(Copy, Clone)]
/// a map of geography template indexes
pub struct GeographyMap {
    indexes: [[usize; WORLD_SIZE]; WORLD_SIZE],
}

impl Default for GeographyMap {
    fn default() -> GeographyMap {
        GeographyMap {
            indexes: [[0; WORLD_SIZE]; WORLD_SIZE],
        }
    }
}

#[derive(Copy, Clone)]
/// a map of geography icons for drawing a world map
pub struct IconMap {
    chars: [[char; WORLD_SIZE]; WORLD_SIZE],
}

impl Default for IconMap {
    fn default() -> IconMap {
        IconMap {
            chars: [[' '; WORLD_SIZE]; WORLD_SIZE],
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

    /// world size (world is always square)
    size: u32,

    #[serde(skip)]
    /// map of population density, deterministic so skipped when reloading game
    pub pop: PopMap,

    #[serde(skip)]
    /// map of roads, deterministic so skipped when reloading game
    pub roads: RoadMap,

    #[serde(skip)]
    /// map of geographies
    pub geographies: GeographyMap,

    #[serde(skip)]
    /// map of icons
    pub icons: IconMap,

    /// true when the world is ready to be used (after init)
    pub ready: bool,
}

impl Default for WorldState {
    fn default() -> WorldState {
        let mut rng = rand::thread_rng();
        WorldState {
            seed: rng.gen_range(0, std::u32::MAX),
            day: 0,
            time: 6.0,
            year: 70,
            pop: PopMap::default(),
            roads: RoadMap::default(),
            geographies: GeographyMap::default(),
            icons: IconMap::default(),
            size: WORLD_SIZE as u32,
            ready: false,
        }
    }
}

impl WorldState {
    /// needs a getter because changing this causes the world to change ready state
    pub fn seed(&self) -> u32 {
        self.seed
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

    pub fn get_icon(&self, region: Region) -> char {
        let (x, y) = self.to_abs_pos(region);
        self.icons.chars[x][y]
    }

    pub fn set_pop(&mut self, region: Region, density: f32) {
        let (x, y) = self.to_abs_pos(region);
        self.pop.samples[x][y] = density;
    }

    pub fn get_pop(&self, region: Region) -> f32 {
        let (x, y) = self.to_abs_pos(region);
        self.pop.samples[x][y]
    }

    pub fn get_road(&self, region: Region) -> RoadTile {
        let (x, y) = self.to_abs_pos(region);
        self.roads.tiles[x][y]
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

    /// chooses a random geography based on a random number <selector>
    pub fn choose_geography<'a>(&mut self, noise: &Noise, assets: &'a Assets, region: Region) {
        let sample: f32 =
            rand_up(noise.get_fbm([region.x as f32 * 0.1, region.y as f32 * 0.1], 16));
        let pop = self.get_pop(region);
        let choices: Vec<(usize, &GeographyTemplate)> = assets
            .get_geographies()
            .values()
            .enumerate()
            .filter(|item| item.1.population_range[0] <= pop && item.1.population_range[1] >= pop)
            .collect();
        let len = choices.len() as f32;
        let choice = *choices
            .get((len * (sample % len)).floor() as usize)
            .expect("no available geographies matching the given tag");
        let (x, y) = self.to_abs_pos(region);
        self.geographies.indexes[x][y] = choice.0;
        if let Some(icon) = &choice.1.icon {
            self.icons.chars[x][y] = assets
                .get_icon(&icon.name)
                .variant_ch(noise.get_fbm([region.x as f32, region.y as f32], 8));
        }
    }

    pub fn get_geography_from_assets(&self, assets: &Assets, region: Region) -> GeographyTemplate {
        let (x, y) = self.to_abs_pos(region);
        let index = self.geographies.indexes[x][y];
        let geographies: Vec<&GeographyTemplate> = assets.get_geographies().values().collect();
        if let Some(geography) = geographies.get(index) {
            return (*geography).clone();
        } else {
            GeographyTemplate::default()
        }
    }

    /// gets a seed based on the world seed and the given region
    pub fn region_seed(&self, region: Region) -> u64 {
        // TODO is this too sloppy? probably works fine
        let off = region.to_unsigned();
        (u64::from(self.seed) / 32) + (off[0] << 3) + off[1]
    }

    /// makes a region-specific RNG
    pub fn region_rng(&self, region: Region) -> Pcg32 {
        Pcg32::seed_from_u64(self.region_seed(region))
    }
}
