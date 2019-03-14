use crate::component::{Position, Region};
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::util::Rect;

mod iterators;
mod map;
mod tile;
pub use map::AreaMap;
pub use tile::Tile;

pub const WIDTH: usize = MAP_WIDTH as usize;
pub const HEIGHT: usize = MAP_HEIGHT as usize;

use specs::{Component, VecStorage};
use std::collections::hash_map::IterMut;
use std::collections::HashMap;
#[derive(Clone, Default, Component)]
#[storage(VecStorage)]
pub struct AreaMaps {
    maps: HashMap<Region, AreaMap>,
}

impl AreaMaps {
    /// initialize new maps for a given <center> and <radius> radius
    /// Note that radius extends from the edge of the center, so a "size 2" map is 5x5
    pub fn init(&mut self, center: Region, size: u8) {
        println!("SELECTED MAP DIMENSIONS: ({},{})", WIDTH, HEIGHT);
        let s = i32::from(size); // size is only u8 to enforce an unsigned parameter
        let mut count: i32 = 0;
        let surrounding_maps = Rect {
            t_l: Position::new(center.x - s, center.y - s),
            b_r: Position::new(center.y + s, center.y + s),
        };
        /*
        let min_x = center.x - s;
        let max_x = center.x + s + 1;
        let min_y = center.y - s;
        let max_y = center.y + s + 1;
        */
        for pos in surrounding_maps.iter() {
            let region = Region::new(pos.x, pos.y);
            self.maps.entry(region).or_insert_with(|| {
                count += 1;
                AreaMap::default()
            });
        }
        if count > 0 {
            println!(
                "initialized {} new maps at center {:?}, size {}",
                count, center, size
            );
        }
    }

    /// get the map at the given location. Will probably die if the map doesn't exist,
    /// but we want that because it shouldn't have happened.
    pub fn get(&self, region: Region) -> &AreaMap {
        match self.maps.get(&region) {
            Some(map) => map,
            None => {
                panic!(format!("no map for region {:?}", region));
            }
        }
    }

    /// checks whether a map is in play
    pub fn has(&self, region: Region) -> bool {
        self.maps.get(&region).is_some()
    }

    /// check if the map for the given region is ready for play.
    pub fn ready(&self, region: Region) -> bool {
        match self.maps.get(&region) {
            Some(map) => map.populated,
            None => false,
        }
    }

    pub fn populated(&self) -> bool {
        let mut result = true;
        for (_, map) in self.maps.iter() {
            result = result && map.populated
        }
        result
    }

    /// prunes maps in collection farther than <size> maps from <center> in a square
    pub fn prune(&mut self, center: Region, size: u8) {
        let s = i32::from(size);
        let mut count: u32 = 0;
        let mut marked: Vec<Region> = Vec::new();
        for (region, _) in self.maps.iter() {
            if (center.x - region.x).abs() > s || (center.y - region.y).abs() > s {
                marked.push(region.clone());
            }
        }
        for mark in marked {
            println!("pruning {:?}", mark);
            self.maps.remove(&mark);
            count += 1;
        }
        if count > 0 {
            println!("pruned {} maps", count);
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Region, AreaMap> {
        self.maps.iter_mut()
    }
}
