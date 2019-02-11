use tcod::random::{Rng, Algo};
use tcod::noise::*;
use crate::resource::{AreaMap, AreaMapCollection, GameState, Offset};
use crate::constants::CHUNK_RADIUS;

mod connect_tiles;
mod ground_cover;
mod roads;
mod util;
mod trees;

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
    Write<'a, AreaMapCollection>,
    Write<'a, GameState>
  );

  fn run(&mut self, (mut maps, mut state): Self::SystemData) {
    maps.init(state.area_offset, CHUNK_RADIUS);
    maps.prune(state.area_offset, CHUNK_RADIUS);
    for (offset, map) in maps.iter_mut() {
      if !map.populated {
        self.generate(offset, map, &mut state);
      }
    }
  }
}

impl MapGenerator {
  fn generate(&mut self, offset: &Offset, map: &mut AreaMap, state: &mut GameState) {
    // let map = AreaMap::default();
    println!("Generating new map with world seed {} at position {}, {}",
      state.world_seed,
      offset[0],
      offset[1]);
    map.wipe();
    let rng = Rng::new_with_seed(Algo::CMWC, state.world_seed);
    let width  = self.width;
    let height = self.height;
    let noise = Noise::init_with_dimensions(2)
      .noise_type(NoiseType::Simplex)
      .random(rng)
      .init();

    // lay down a basic grass layer
    ground_cover::grass(&noise, map, width, height, state.area_offset, 0.2);

    // place trees
    trees::place_trees(&noise, map, width, height, state.area_offset, 0.2, 0.7);

    // draw a road
    roads::place_horizontal_roads(&noise, map, state.area_offset, 0.1, 0.8, 8);

    // connect connectable tiles
    connect(map);

    // mark map generation done
    map.populated = true;
  }
}
