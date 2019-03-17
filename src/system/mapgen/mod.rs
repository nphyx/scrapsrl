use crate::component::Region;
use crate::resource::{
    Assets, GameStage, GameState, GeographyTemplate, RegionMap, RegionMaps, WorldState,
};
use tcod::noise::*;
use tcod::random::{Algo, Rng};

mod connect_tiles;
mod ground_cover;
mod roads;
mod structure;
mod trees;
pub mod util;

use connect_tiles::connect;

/// many of the map generator functions need the same stuff, so we'll just pass
/// it around in a bundle instead of having to pass parameters deep into the
/// tree of functions
pub struct MapGenBundle<'a> {
    assets: &'a Assets,
    map: &'a mut RegionMap,
    noise: &'a mut Noise,
    region: Region,
    world: &'a WorldState,
    geography: &'a GeographyTemplate,
}

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
    fn generate(
        &mut self,
        region: Region,
        map: &mut RegionMap,
        assets: &Assets,
        world: &WorldState,
    ) {
        let seed = world.seed();
        println!(
            "Generating new map with dimensions {}x{}, seed {} for region {:?}",
            map.width(),
            map.height(),
            seed,
            region
        );
        let trng = Rng::new_with_seed(Algo::CMWC, world.seed());
        let noise = &mut Noise::init_with_dimensions(2)
            .noise_type(NoiseType::Simplex)
            .random(trng)
            .init();

        // choose a geography variant
        let geography = &world.get_geography_from_assets(assets, region).clone();

        let bundle = &mut MapGenBundle {
            assets,
            map,
            noise,
            region,
            world,
            geography,
        };

        // lay down a basic ground cover layer
        ground_cover::base(bundle, 0.2);
        ground_cover::scatter(bundle, 1.0);

        let road_data = world.get_road(region);

        if road_data.lanes_x > 0 {
            roads::place_horizontal_roads(bundle, 0.1, 0.8);
        }

        if road_data.lanes_y > 0 {
            roads::place_vertical_roads(bundle, 0.1, 0.8);
        }

        if bundle.geography.structure_len() > 0 {
            structure::build(bundle).ok(); // always ok if this fails
        }

        // connect connectable tiles
        connect(bundle);

        // mark map generation done
        map.populated = true;
    }
}
