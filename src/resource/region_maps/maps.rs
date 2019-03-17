use super::RegionMap;
use crate::component::Region;
use crate::util::Rect;
use specs::{Component, VecStorage};
use std::collections::hash_map::IterMut;
use std::collections::HashMap;

#[derive(Clone, Default, Component)]
#[storage(VecStorage)]
pub struct RegionMaps {
    maps: HashMap<Region, RegionMap>,
}

impl RegionMaps {
    /// initialize new maps for a given <center> and <radius> radius
    /// Note that radius extends from the edge of the center, so a "size 2" map is 5x5
    pub fn init(&mut self, center: Region, size: u8) {
        let s = i32::from(size); // size is only u8 to enforce an unsigned parameter
        let mut count: i32 = 0;
        let mut surrounding_maps: Rect<i32> = Rect {
            t_l: center.into(),
            b_r: center.into(),
        };
        surrounding_maps.expand_perimeter(s);

        for region in surrounding_maps.iter() {
            self.maps.entry(region.into()).or_insert_with(|| {
                count += 1;
                RegionMap::default()
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
    pub fn get(&self, region: Region) -> &RegionMap {
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
        let mut bounds = Rect::new(center.into(), center.into());
        bounds.expand_perimeter(s);
        for (region, _) in self.maps.iter() {
            if !bounds.includes((*region).into()) {
                marked.push(*region);
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

    pub fn iter_mut(&mut self) -> IterMut<'_, Region, RegionMap> {
        self.maps.iter_mut()
    }
}
