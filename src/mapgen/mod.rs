use rand::prelude::*;
use std::cmp::max;
use tcod::map::Map;
use tcod::random::{Rng, Algo};
use tcod::noise::*;
use tcod::colors::{Color, lerp};
use super::constants::{MAP_WIDTH, MAP_HEIGHT};
use super::entity::Coord;
use super::util::clamp;
use super::util::icons::*;

mod tile;
pub use self::tile::{Tile, Tiles};

const SEED: u32 = 2234567890;

/*
const PLANT: Tile = Tile{
  color: Color{r:24, g:180, b:78},
  ch: ICON_HERB, solid: false,
  desc: "A small plant."};
const GRASS: Tile = Tile{
  color: Color{r:89, g:97, b:15},
  ch: ',', solid: false,
  desc: };
const TALL_GRASS: Tile = Tile{color: Color{r:89, g:97, b:15}, 
  ch: '"', solid: false,
  desc: "Some tall grass."};
*/


pub fn place_tree(tiles: &mut Tiles, cx: i32, cy: i32) {
  let min_x = clamp(0, MAP_WIDTH, cx - 1);
  let min_y = clamp(0, MAP_HEIGHT, cy - 1);
  let max_x = clamp(0, MAP_WIDTH, cx + 2);
  let max_y = clamp(0, MAP_HEIGHT, cy + 2);
  let fg = Color{r:98, g:27, b:15};
  let bg = Color{r:12, g:14, b:3};
  let tree_bark: Tile = Tile{fg, bg, 
    ch: LINE_DBL, walkable: false, transparent: false,
    desc: "The bark of a tree."};
  let tree_trunk: Tile = Tile{fg, bg,
    ch: '0', walkable: false, transparent: false,
    desc: "The trunk of a tree."};

  for x in min_x..max_x {
    for y in min_y..max_y {
      tiles.insert(Coord{x, y}, tree_bark.clone());
    }
  }
  tiles.insert(Coord{x:cx, y:cy}, tree_trunk.clone());
}

fn check_tree_placement(tree_places: &Vec<(i32, i32)>, cx: i32, cy: i32) -> bool {
  for x in cx-3..cx+3 {
    for y in cy-3..cy+3 {
      if tree_places.contains(&(x, y)) { return false; }
    }
  }
  return true;
}

fn place_trees(tiles: &mut Tiles, width: i32, height: i32, noise: &Noise, noise_scale: f32) {
  let mut tree_places: Vec<(i32, i32)> = vec![];
  for x in 0..width {
    for y in 0..height {
      if check_tree_placement(&tree_places, x, y) {
        let i = rand_up(noise.get_fbm([x as f32 * noise_scale, y as f32 * noise_scale], 32));
        if i > 0.99 {
          place_tree(tiles, x, y);
          tree_places.push((x, y));
        }
      }
    }
  }
}

pub fn rand_up(v: f32) -> f32 { (v + 1.0) / 2.0 }

pub fn lay_grass(tiles: &mut Tiles, width: i32, height: i32, noise: &Noise, noise_scale: f32) {
  let desc_sg = "Just some ordinary grass.";
  let desc_tg = "Some tall grass.";
  let transparent = true;
  let walkable = true;
  let color_sg_fg = Color{r:112, g:181, b:15};
  let color_tg_fg = Color{r:118, g:121, b:15};
  let color_sg_bg = Color{r:4, g:14, b:8};
  let color_tg_bg = Color{r:9, g:19, b:5};
  for x in 0..width {
    for y in 0..height {
      let i = rand_up(noise.get_fbm([x as f32 * noise_scale, y as f32 * noise_scale], 32));
      let fg = lerp(color_sg_fg, color_tg_fg, i);
      let bg = lerp(color_sg_bg, color_tg_bg, i);
      if i < 0.7 {
        tiles.insert(
          Coord{x, y},
          Tile{ch: ',', fg, bg, transparent, walkable, desc: desc_sg});
      } else {
        tiles.insert(
          Coord{x, y},
          Tile{ch: '"', fg, bg, transparent, walkable, desc: desc_tg});
      }
    }
  }
}

pub fn generate<'a>(width: i32, height: i32) -> (Map, Tiles<'a>) {
  let mut map = Map::new(width, height);
  let mut tiles: Tiles = Tiles::new();
  let rng = Rng::new_with_seed(Algo::CMWC, SEED);
  let noise = Noise::init_with_dimensions(2)
    .noise_type(NoiseType::Simplex)
    .random(rng)
    .init();

  // lay down a basic grass layer
  lay_grass(&mut tiles, width, height, &noise, 0.3);
  place_trees(&mut tiles, width, height, &noise, 0.3);
  // Set the map.
  // place trees
  /*
  let count:i32 = rng.get_int(5, 40);
  for _ in 0..count {
    let cx:i32 = rng.get_int(2, width - 2);
    let cy:i32 = rng.get_int(2, height - 2);
    // put_tree(&mut map, &mut tiles, cx, cy);
  }
  */

  // connect connectable tiles
  tiles.connect_tiles();

  // assign passability based on final tile layout
  for x in 0..width {
    for y in 0..height {
      match tiles.get(Coord{x, y}) {
        Some(tile) => map.set(x, y, tile.transparent, tile.walkable),
        None => {}
      }
    }
  }

  return (map, tiles);
}
