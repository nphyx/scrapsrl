use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::component::Position;

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

pub type Offset = [i32; 2];


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
  maps: HashMap<Offset, AreaMap>
}

impl AreaMapCollection {
  /// initialize new maps for a given <center> and <radius> radius
  /// Note that radius extends from the edge of the center, so a "size 2" map is 5x5
  pub fn init(&mut self, center: Offset, size: u8) {
    let s = size as i32; // size is only u8 to enforce an unsigned parameter
    let min_x = center[0] - s; 
    let max_x = center[0] + s + 1;
    let min_y = center[1] - s; 
    let max_y = center[1] + s + 1; 
    for x in min_x..max_x {
      for y in min_y..max_y {
        let offset = [x, y];
        if !self.maps.contains_key(&offset) {
          self.maps.insert(offset, AreaMap::default());
        }
      }
    }
  }

  pub fn get(&self, offset: Offset) -> &AreaMap {
    return self.maps.get(&offset).unwrap();
  }

  pub fn populated(&self) -> bool {
    let mut result = true;
    for (_, map) in self.maps.iter() {
      result = result && map.populated
    }
    return result;
  }
  /// prunes maps in collection farther than <size> maps from <center> in a square
  pub fn prune(&mut self, center: Offset, size: u8) {
    let mut marked: Vec<Offset> = Vec::new();
    for (offset, map) in self.maps.iter() {
      if (center[0] - offset[0]).abs() > size as i32 ||
         (center[1] - offset[1]).abs() > size as i32 {
           marked.push(offset.clone());
      }
    }
    for mark in marked {
      self.maps.remove(&mark);
    }
  }
  pub fn insert(&mut self, offset: Offset, map: AreaMap) {
    self.maps.insert(offset, map);
  }

  pub fn iter_mut(&mut self) -> IterMut<Offset, AreaMap> {
    self.maps.iter_mut()
  }
}
