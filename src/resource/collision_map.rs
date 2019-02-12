use crate::component::{Position, Region};
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use std::collections::HashMap;

/// a boolean map tracking solid entities for quick reference
/// updated by the CollisionSystem
#[derive(Clone)]
pub struct CollisionMap {
    map: [[bool; MAP_HEIGHT as usize]; MAP_WIDTH as usize],
}

impl CollisionMap {
    pub fn new() -> CollisionMap {
        CollisionMap {
            map: [[false; MAP_HEIGHT as usize]; MAP_WIDTH as usize],
        }
    }

    pub fn get(&self, position: Position) -> bool {
        self.map[position.x as usize][position.y as usize]
    }
}

impl Default for CollisionMap {
    fn default() -> CollisionMap {
        CollisionMap::new()
    }
}

use specs::{Component, VecStorage};
#[derive(Clone, Component, Default)]
#[storage(VecStorage)]
pub struct CollisionMaps {
    maps: HashMap<Region, CollisionMap>,
}

impl CollisionMaps {
    /* FIXME unused always use default instead?
    pub fn new() -> CollisionMaps {
      CollisionMaps{maps: HashMap::new()}
    }
    */
    pub fn get(&self, region: Region, position: Position) -> bool {
        match self.maps.get(&region) {
            Some(region) => region.get(position),
            None => {
                println!(
                    "tried to get a collision map for {:?} but there wasn't one",
                    region
                );
                true
            }
        }
    }
    pub fn set(&mut self, region: Region, position: Position, v: bool) {
        match self.maps.get_mut(&region) {
            Some(mut region) => region.map[position.x as usize][position.y as usize] = v,
            None => {
                println!(
                    "tried to set a collision map for {:?} but there wasn't one",
                    region
                );
            }
        }
    }

    pub fn init(&mut self, center: Region, size: u8) {
        let s = i32::from(size); // size is only u8 to enforce an unsigned parameter
        let mut count: i32 = 0;
        let min_x = center.x - s;
        let max_x = center.x + s + 1;
        let min_y = center.y - s;
        let max_y = center.y + s + 1;
        for x in min_x..max_x {
            for y in min_y..max_y {
                let region = Region::new(x, y);
                self.maps.entry(region).or_insert_with(|| {
                    count += 1;
                    CollisionMap::new()
                });
            }
        }
        if count > 0 {
            println!(
                "initialized {} new collision maps at center {:?}, size {}",
                count, center, size
            );
        }
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
            self.maps.remove(&mark);
            count += 1;
        }
        if count > 0 {
            println!("pruned {} collision maps", count);
        }
    }

    /* FIXME unused do we need this at all?
    /// wipes current collision data
    pub fn wipe(&mut self) {
      for (_, mut a_map) in self.maps.iter_mut() {
        for x in 0..MAP_WIDTH as usize {
          for y in 0..MAP_HEIGHT as usize {
            a_map.map[x][y] = false;
          }
        }
      }
    }
    */
}
