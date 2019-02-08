use tcod::colors::Color;
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::component::Position;

mod iterators;
pub mod tile_types;
pub use tile_types::{TileType, get_tile_descriptions};
use iterators::{AreaMapIter};

pub const WIDTH: usize = MAP_WIDTH as usize;
pub const HEIGHT: usize = MAP_HEIGHT as usize;

#[derive(Copy,Clone)]
pub struct Tile {
  pub icon: char,
  pub fg: Color,
  pub bg: Color,
  pub transparent: bool,
  pub walkable: bool,
  pub type_id: u32 // this references the tile descriptions in mapgen/tile_types
}

impl Default for Tile {
  fn default() -> Tile {
    Tile{
      icon: ' ',
      fg: Color::new(255, 255, 255),
      bg: Color::new(0, 0, 0),
      transparent: true,
      walkable: true,
      type_id: 0
    }
  }
}

impl Tile {
  pub fn new(icon: char, fg: Color, bg: Color, transparent: bool,
    walkable: bool, type_id: u32) -> Tile {
    Tile{
      icon,
      fg,
      bg,
      transparent,
      walkable,
      type_id
    }
  }
}

pub struct AreaMap {
  tiles: [[Tile; HEIGHT]; WIDTH],
  pub width: i32,
  pub height: i32
}

impl Default for AreaMap {
  fn default() -> AreaMap {
    let tiles = [[Tile::default(); HEIGHT]; WIDTH];
    AreaMap{tiles, width: WIDTH as i32, height: HEIGHT as i32}
  }
}

impl AreaMap {
  pub fn wipe(&mut self) {
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

  /*
  pub fn get_mut<'b>(&mut self, pos: Position) -> &mut Tile<'b> {
    if 0 > pos.x || pos.x >= self.width || 
       0 > pos.y || pos.y >= self.height {
         return &mut Tile::default()
    }
    &mut self.tiles[pos.x as usize][pos.y as usize]
  }
  */

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

  /*
   * this is broken right now so skip it
  pub fn iter_mut(&'a mut self) -> AreaMapIterMut {
    AreaMapIterMut{
      map: self,
      cur: [0, 0]
    }
  }
  */
}
