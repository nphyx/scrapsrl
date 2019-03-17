use crate::component::Region;
use crate::resource::{RegionMap, RegionMaps, Assets, GameStage, GameState, WorldState};
use tcod::noise::*;
use tcod::random::{Algo, Rng};

mod connect_tiles;
mod ground_cover;
mod roads;
mod structure;
mod trees;
pub mod util;

use connect_tiles::connect;

pub struct MapGenerator {}

impl MapGenerator {
    pub fn new() -> MapGenerator {
        MapGenerator {}
    }
}

use specs::{Read, System, Write};
impl<'a> System<'a> for MapGenerator {
    type SystemData = (
        Read<'a, Assets>,
        Write<'a, RegionMaps>,
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
    fn generate(&mut self, region: Region, map: &mut RegionMap, assets: &Assets, world: &WorldState) {
        let seed = world.seed();
        println!(
            "Generating new map with dimensions {}x{}, seed {} for region {:?}",
            map.width(),
            map.height(),
            seed,
            region
        );
        let rng = Rng::new_with_seed(Algo::CMWC, world.seed());
        let noise = Noise::init_with_dimensions(2)
            .noise_type(NoiseType::Simplex)
            .random(rng)
            .init();

        // choose a geography variant
        let geography = world.get_geography_from_assets(assets, region);
        map.geography = geography.clone();

        // lay down a basic ground cover layer
        ground_cover::base(&noise, map, region.to_offset(), 0.2, assets);
        ground_cover::scatter(&noise, map, region.to_offset(), 1.0, assets);

        let road_data = world.get_road(region);

        if road_data.lanes_x > 0 {
            roads::place_horizontal_roads(&assets, &noise, world, map, region, 0.1, 0.8);
        }

        if road_data.lanes_y > 0 {
            roads::place_vertical_roads(&assets, &noise, world, map, region, 0.1, 0.8);
        }

        structure::build(&assets, &noise, map, region, world).ok(); // always ok if this fails

        // connect connectable tiles
        connect(map);

        // mark map generation done
        map.populated = true;
    }
}
