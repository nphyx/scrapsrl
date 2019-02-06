use tcod::random::{Rng, Algo};
use tcod::noise::*;
use crate::area_map::{AreaMap};
use crate::game_state::GameState;

mod connect_tiles;
mod ground_cover;
mod roads;
mod util;
mod trees;
mod tile_types;

use connect_tiles::connect;

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
    Write<'a, AreaMap>,
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

    // place trees (ok for them to grow through the road, it's been a long time)
    trees::place_trees(&noise, &mut map, width, height, state.area_offset, 0.2, 0.7);

    // draw a road
    roads::place_horizontal_roads(&noise, &mut map, state.area_offset, 0.1, 0.8, 8);

    // connect connectable tiles
    connect(&mut map);


    state.map_gen_queued = false;
  }
}
