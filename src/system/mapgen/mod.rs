use tcod::random::{Rng, Algo};
use tcod::noise::*;
use tcod::colors::{Color};
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::util::{clamp, icons::*};
use crate::area_map::{AreaMap};
use crate::game_state::GameState;
use crate::component::*;

mod connect_tiles;
mod ground_cover;
mod roads;
mod util;

use connect_tiles::connect;
use util::*;

fn place_tree(map: &mut AreaMap, cx: i32, cy: i32) {
  let min_x = clamp(0, MAP_WIDTH, cx - 1);
  let min_y = clamp(0, MAP_HEIGHT, cy - 1);
  let max_x = clamp(0, MAP_WIDTH, cx + 2);
  let max_y = clamp(0, MAP_HEIGHT, cy + 2);
  let fg = Color{r:86, g:50, b:32};
  let bg = Color{r:32, g:24, b:12};
  let tree_bark = prep_tile(LINE, fg, bg, true, true, "tree bark", "The bark of a tree.");
  let tree_trunk = prep_tile('o', fg, bg, true, true, "tree trunk", "The trunk of a tree.");

  for x in min_x..max_x {
    for y in min_y..max_y {
      map.set(Position{x, y}, tree_bark.clone());
    }
  }
  map.set(Position{x:cx, y:cy}, tree_trunk.clone());
}

fn check_tree_placement(tree_places: &Vec<(i32, i32)>, cx: i32, cy: i32) -> bool {
  for x in cx-4..cx+4 {
    for y in cy-4..cy+4 {
      if tree_places.contains(&(x, y)) { return false; }
    }
  }
  return true;
}

fn place_trees(noise: &Noise, map: &mut AreaMap, width: i32, height: i32, offset: [i32; 2], scale: f32) {
  let mut tree_places: Vec<(i32, i32)> = vec![];
  for x in 0..width {
    for y in 0..height {
      if check_tree_placement(&tree_places, x, y) {
        let i = rand_up(fbm_offset(noise, [x, y], offset, scale, 32));
        if i > 0.99 {
          place_tree(map, x, y);
          tree_places.push((x, y));
        }
      }
    }
  }
}

pub struct MapGenerator {
  width: i32,
  height: i32
}

impl MapGenerator {
  pub fn new(width: i32, height: i32) -> MapGenerator {
    MapGenerator{width, height}
  }
}

use specs::{System, Write};
impl<'a> System<'a> for MapGenerator {
  type SystemData = (
    Write<'a, AreaMap<'static>>,
    Write<'a, GameState>
  );

  fn run(&mut self, (mut map, mut state): Self::SystemData) {
    if !(state.map_gen_queued) { return; }
    println!("Generating new map with world seed {} at position {}, {}",
      state.world_seed,
      state.area_offset[0],
      state.area_offset[1]);
    map.wipe();
    let rng = Rng::new_with_seed(Algo::CMWC, state.world_seed);
    let width  = self.width;
    let height = self.height;
    // let (width, height) = (&map_width, &map_height).join();
    let noise = Noise::init_with_dimensions(2)
      .noise_type(NoiseType::Simplex)
      .random(rng)
      .init();

    // lay down a basic grass layer
    ground_cover::lay_grass(&noise, &mut map, width, height, state.area_offset, 0.2);

    // draw a road
    roads::place_horizontal_roads(&noise, &mut map, state.area_offset, 0.1, 0.8, 8);

    // place trees (ok for them to grow through the road, it's been a long time)
    place_trees(&noise, &mut map, width, height, state.area_offset, 0.2);

    // connect connectable tiles
    connect(&mut map);


    state.map_gen_queued = false;
  }
}
