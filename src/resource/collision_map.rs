use crate::component::{Pos, Region};
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::util::{Grid, Rect};
use std::collections::HashMap;

use specs::{Component, VecStorage};
#[derive(Clone, Component, Default)]
#[storage(VecStorage)]
pub struct CollisionMaps {
    maps: HashMap<Region, Grid<bool>>,
}

impl CollisionMaps {
    pub fn get(&self, region: Region, position: Pos) -> bool {
        if let Some(grid) = self.maps.get(&region) {
            return *grid.maybe_get(position).unwrap_or(&true);
        }
        true
    }

    pub fn set(&mut self, region: Region, pos: Pos, v: bool) {
        if let Some(grid) = self.maps.get_mut(&region) {
            grid.try_set(pos, v).ok();
        }
    }

    pub fn init(&mut self, center: Region, size: u8) {
        let s = i32::from(size); // size is only u8 to enforce an unsigned parameter
        let mut count: i32 = 0;
        let mut regions: Rect<i32> = Rect::new(center.into(), center.into());
        regions.expand_perimeter(s);
        for region in regions {
            self.maps.entry(Region::from(region)).or_insert_with(|| {
                count += 1;
                let bounds = Rect::new(Pos::new(0, 0), Pos::new(MAP_WIDTH, MAP_HEIGHT));
                Grid::with_bounds(bounds)
            });
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
        let mut bounds: Rect<i32> = Rect::new(center.into(), center.into());
        bounds.expand_perimeter(s);
        let mut marked: Vec<Region> = Vec::new();

        for (region, _) in self.maps.iter() {
            if !bounds.includes((*region).into()) {
                marked.push(*region);
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
}
