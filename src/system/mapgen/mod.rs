use crate::component::Region;
use crate::resource::{AreaMap, AreaMaps, Assets, GameStage, GameState, WorldState};
use tcod::noise::*;
use tcod::random::{Algo, Rng};

mod connect_tiles;
mod ground_cover;
mod roads;
mod trees;
mod util;

use connect_tiles::connect;

pub struct MapGenerator {
    width: i32,
    height: i32,
}

impl MapGenerator {
    pub fn new(width: i32, height: i32) -> MapGenerator {
        MapGenerator { width, height }
    }
}

use specs::{Read, System, Write};
impl<'a> System<'a> for MapGenerator {
    type SystemData = (
        Read<'a, Assets>,
        Write<'a, AreaMaps>,
        Write<'a, GameState>,
        Read<'a, WorldState>,
    );

    fn run(&mut self, (assets, mut maps, state, world): Self::SystemData) {
        if state.stage == GameStage::LoadingAssets {
            return;
        } // don't try to build map while assets loading
        for (region, map) in maps.iter_mut() {
            if !map.populated {
                self.generate(*region, map, &assets, &world);
                return; // only do one per pass, so we can show progress
            }
        }
    }
}

impl MapGenerator {
    fn generate(&mut self, region: Region, map: &mut AreaMap, assets: &Assets, world: &WorldState) {
        let seed = world.seed();
        // let map = AreaMap::default();
        println!(
            "Generating new map with dimensions {}x{}, seed {} for region {:?}",
            map.width, map.height, seed, region
        );
        map.wipe();
        let rng = Rng::new_with_seed(Algo::CMWC, world.seed());
        let noise = Noise::init_with_dimensions(2)
            .noise_type(NoiseType::Simplex)
            .random(rng)
            .init();

        // choose a geography variant
        let geography = assets.choose_geography(1.0); // TODO choose geography from noise

        // lay down a basic ground cover layer
        ground_cover::base(&noise, map, region.to_offset(), 0.2, &geography, assets);
        ground_cover::scatter(&noise, map, region.to_offset(), 1.0, &geography, assets);

        /* place trees
        trees::place_trees(&noise, map, width, height, region.to_offset(), 0.2, 0.7);
        */

        let road_data = world.get_road(region);

        if road_data.lanes_x > 0 {
            // draw a road
            roads::place_horizontal_roads(
                &assets,
                &noise,
                map,
                region.to_offset(),
                0.1,
                0.8,
                road_data.lanes_x as i32,
            );
        }

        if road_data.lanes_y > 0 {
            // draw a road
            roads::place_vertical_roads(
                &assets,
                &noise,
                map,
                region.to_offset(),
                0.1,
                0.8,
                road_data.lanes_y as i32,
            );
        }

        // connect connectable tiles
        connect(map);

        // mark map generation done
        map.populated = true;
    }
}
