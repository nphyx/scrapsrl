use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::component::{Region, Position};

mod iterators;
mod tile;
pub use tile::Tile;
pub mod tile_types;
pub use tile_types::{TileType, get_tile_descriptions};
use iterators::AreaMapIter;

pub const WIDTH: usize = MAP_WIDTH as usize;
pub const HEIGHT: usize = MAP_HEIGHT as usize;


#[derive(Clone)]
pub struct AreaMap {
  tiles: [[Tile; HEIGHT]; WIDTH],
  pub width: i32,
  pub height: i32,
  /// mark true when mapgen is complete
  pub populated: bool
}

impl Default for AreaMap {
  fn default() -> AreaMap {
    let tiles = [[Tile::default(); HEIGHT]; WIDTH];
    AreaMap{tiles, width: WIDTH as i32, height: HEIGHT as i32, populated: false}
  }
}

impl AreaMap { pub fn wipe(&mut self) {
    let tile = Tile::default();
    for x in 0..WIDTH {
      for y in 0..HEIGHT {
        self.tiles[x][y] = tile;
      }
    }
  }

  pub fn get(&self, pos: Position) -> Option<Tile> {
    if 0 > pos.x || pos.x >= self.width || 
       0 > pos.y || pos.y >= self.height {
         return None 
    }
    Some(self.tiles[pos.x as usize][pos.y as usize])
  }

  pub fn get_icon(&self, pos: Position) -> Option<char> {
    if 0 > pos.x || pos.x >= self.width || 
       0 > pos.y || pos.y >= self.height {
         return None 
    }
    Some(self.tiles[pos.x as usize][pos.y as usize].icon)
  }

  pub fn set(&mut self, pos: Position, tile: Tile) {
    if 0 > pos.x || pos.x >= self.width || 
       0 > pos.y || pos.y >= self.height { return; }
    self.tiles[pos.x as usize][pos.y as usize] = tile;
  }

  pub fn set_icon(&mut self, pos: Position, icon: char) {
    if 0 > pos.x || pos.x >= self.width || 
       0 > pos.y || pos.y >= self.height { return; }
    self.tiles[pos.x as usize][pos.y as usize].icon = icon;
  }

  pub fn iter(&self) -> AreaMapIter {
    AreaMapIter{
      map: self,
      cur: [0, 0]
    }
  }
}

use std::collections::HashMap;
use std::collections::hash_map::IterMut;
use specs::{Component,VecStorage};
#[derive(Clone,Default,Component)]
#[storage(VecStorage)]
pub struct AreaMapCollection {
  maps: HashMap<Region, AreaMap>
}

impl AreaMapCollection {
  /// initialize new maps for a given <center> and <radius> radius
  /// Note that radius extends from the edge of the center, so a "size 2" map is 5x5
  pub fn init(&mut self, center: &Region, size: u8) {
    let s = size as i32; // size is only u8 to enforce an unsigned parameter
    let mut count: i32 = 0;
    let min_x = center.x - s; 
    let max_x = center.x + s + 1;
    let min_y = center.y - s; 
    let max_y = center.y + s + 1; 
    for x in min_x..max_x {
      for y in min_y..max_y {
        let region = Region::new(x, y);
        if !self.maps.contains_key(&region) {
          self.maps.insert(region, AreaMap::default());
          count += 1;
        }
      }
    }
    if count > 0 { println!("initialized {} new maps at center {:?}, size {}", count, center, size); }
  }

  /// get the map at the given location. Will probably die if the map doesn't exist, but
  /// we want that because it shouldn't have happened.
  pub fn get(&self, region: &Region) -> &AreaMap {
    match self.maps.get(&region) {
      Some(map) => { return map; },
      None => { panic!(format!("no map for region {:?}", region)); }
    }
  }

  /// checks whether a map is in play
  pub fn has(&self, region: &Region) -> bool {
    match self.maps.get(&region) {
      Some(_) => { return true; }
      None => { return false; }
    }
  }

  /// check if the map for the given region is ready for play.
  pub fn ready(&self, region: &Region) -> bool {
    match self.maps.get(&region) {
      Some(map) => { return map.populated; },
      None => { return false; }
    }
  }

  pub fn populated(&self) -> bool {
    let mut result = true;
    for (_, map) in self.maps.iter() {
      result = result && map.populated
    }
    return result;
  }

  /// prunes maps in collection farther than <size> maps from <center> in a square
  pub fn prune(&mut self, center: &Region, size: u8) {
    let s = size as i32;
    let mut count: u32 = 0;
    let mut marked: Vec<Region> = Vec::new();
    for (region, _) in self.maps.iter() {
      if (center.x - region.x).abs() > s ||
         (center.y - region.y).abs() > s {
           marked.push(region.clone());
      }
    }
    for mark in marked {
      println!("pruning {:?}", mark);
      self.maps.remove(&mark);
      count += 1;
    }
    if count > 0 { println!("pruned {} maps", count); }
  }

  pub fn iter_mut(&mut self) -> IterMut<Region, AreaMap> {
    self.maps.iter_mut()
  }
}
