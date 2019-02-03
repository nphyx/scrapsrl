use rand::prelude::*;
use tcod::map::Map;
use tcod::colors::Color;
use super::constants::{MAP_WIDTH, MAP_HEIGHT};
use super::entity::Coord;
use super::util::clamp;

mod tile;
pub use self::tile::{Tile, Tiles};

const PLANT: Tile = Tile{
  color: Color{r:24, g:180, b:78},
  ch: '\u{e22f}', solid: false,
  desc: "A small plant."};
const TREE_BARK: Tile = Tile{color: Color{r:128, g:97, b:15}, 
  ch: '\u{256c}', solid: true,
  desc: "The bark of a tree."};
const GRASS: Tile = Tile{
  color: Color{r:89, g:97, b:15},
  ch: ',', solid: false,
  desc: "Just some ordinary grass."};
const TALL_GRASS: Tile = Tile{color: Color{r:89, g:97, b:15}, 
  ch: '"', solid: false,
  desc: "Some tall grass."};
const TREE_TRUNK: Tile = Tile{color: Color{r: 128, g:97, b:15},
  ch: '0', solid: true,
  desc: "The trunk of a tree."};


pub fn put_tree(map: &mut Map, tiles: &mut Tiles, cx: i32, cy: i32) {

  let min_x = clamp(0, MAP_WIDTH, cx - 1);
  let min_y = clamp(0, MAP_HEIGHT, cy - 1);
  let max_x = clamp(0, MAP_WIDTH, cx + 2);
  let max_y = clamp(0, MAP_HEIGHT, cy + 2);

  for x in min_x..max_x {
    for y in min_y..max_y {
      map.set(x,y,false,false);
      tiles.insert(Coord{x, y}, TREE_BARK);
    }
  }

  let left = clamp(0, MAP_WIDTH, cx-2);
  let right = clamp(0, MAP_WIDTH, cx+2);
  map.set(left, cy,false, false);
  tiles.insert(Coord{x:left, y:cy}, TREE_BARK);
  map.set(right, cy, false, false);
  tiles.insert(Coord{x:right, y:cy}, TREE_BARK);

  map.set(cx, cy,false, false);
  tiles.insert(Coord{x:cx, y:cy}, TREE_TRUNK);
}

pub fn generate<'a>(width: i32, height: i32) -> (Map, Tiles<'a>) {
  let mut map = Map::new(width, height);
  let mut tiles: Tiles = Tiles::new();
  let mut rng = rand::thread_rng();

  // Set the map.

  // lay grass
  for x in 0..width {
    for y in 0..height {
      let r:f32 = rng.gen();
      // Place some walls randomly.
      if r < 0.3 {
        tiles.insert(Coord{x, y}, PLANT);
        map.set(x,y,true,true);
      } else if r < 0.7 {
        tiles.insert(Coord{x, y}, GRASS);
        map.set(x,y,true,true);
      } else {
        tiles.insert(Coord{x, y}, TALL_GRASS);
        map.set(x,y,true,true);
      }
    }
  }
  // place trees
  let count:i32 = rng.gen_range(5, 40);
  for _ in 0..count {
    let cx:i32 = rng.gen_range(2, width - 2);
    let cy:i32 = rng.gen_range(2, height - 2);
    put_tree(&mut map, &mut tiles, cx, cy);
  }

  tiles.connect_tiles();

  return (map, tiles);
}
