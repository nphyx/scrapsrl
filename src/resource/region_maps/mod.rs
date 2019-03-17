use crate::constants::{MAP_HEIGHT, MAP_WIDTH};

mod iterators;
mod map;
mod maps;
mod tile;
pub use map::RegionMap;
pub use maps::RegionMaps;
pub use tile::Tile;

pub const WIDTH: usize = MAP_WIDTH as usize;
pub const HEIGHT: usize = MAP_HEIGHT as usize;
