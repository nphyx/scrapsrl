use crate::system::mapgen::util::*;
use specs::{Read, System, Write};
use tcod::noise::*;
use tcod::random::{Algo, Rng};

use crate::component::Region;
use crate::constants::*;
use crate::resource::{GameStage, GameState, WorldState};

pub struct WorldGen;

impl<'a> System<'a> for WorldGen {
    type SystemData = (Read<'a, GameState>, Write<'a, WorldState>);

    fn run(&mut self, (state, mut world): Self::SystemData) {
        if !world.ready && state.stage == GameStage::Initializing {
            println!("generating new world with seed {}", world.seed());
            let rng = Rng::new_with_seed(Algo::CMWC, world.seed());
            let noise = Noise::init_with_dimensions(2)
                .noise_type(NoiseType::Simplex)
                .random(rng)
                .init();

            for x in world.min_x()..world.max_x() {
                for y in world.min_y()..world.max_y() {
                    let region = Region::new(x, y);
                    let pop = rand_up(noise.get_fbm([x as f32 * 0.0125, y as f32 * 0.0125], 32));

                    // generate pre-apocalypse population density
                    world.set_pop(region, pop);

                    // generate roads
                    let o_x: [f32; 2] = [x as f32 * MAP_WIDTH as f32 / 800.0, y as f32];
                    let o_y: [f32; 2] = [x as f32, y as f32 * MAP_HEIGHT as f32 / 800.0];
                    let lanes_x = road_lanes(&noise, o_x, pop);
                    let lanes_y = road_lanes(&noise, o_y, pop);
                    world.set_road(region, lanes_x, lanes_y);
                }
            }
            clean_roads(&mut world);
            extend_roads_x(&mut world);
            extend_roads_y(&mut world);
            // run extend x again to give it something to connect to
            extend_roads_x(&mut world);
            world.ready = true;
            println!("finished world generation");
        }
    }
}

fn road_lanes(noise: &Noise, sample_coord: [f32; 2], pop: f32) -> u8 {
    let base_sample = rand_up(noise.get_fbm(sample_coord, 8));
    let mut adj_sample: f32;
    // we bias the road lanes based on the population density
    // these are fiddly magic numbers, just messed with them until happy
    if pop > 0.8 {
        // very high population has fewer, larger roads
        // with smaller roads and alleys in between
        adj_sample = (base_sample) * (10.0 * pop);
        // trim mid-sized roads
        if adj_sample < 7.0 && adj_sample > 3.0 {
            adj_sample = 0.0
        }
    } else if pop > 0.795 {
        // create large highways around population centers
        adj_sample = (base_sample) * (6.0 * (1.0 - pop));
    } else if pop > 0.4 {
        // mid size pop has frequent mid-sized roads, biased toward edge regions
        adj_sample = (base_sample) * (6.0 * (1.0 - pop));
    } else if pop > 0.2 {
        // suburban populations have smaller roads
        adj_sample = (0.1 + base_sample.powf(2.0)) * (8.0 * pop);
    } else {
        // rural areas have infrequent, small roads
        adj_sample = (base_sample * pop).powf(2.0) - 0.1;
    }
    return adj_sample.min(7.0).max(0.0).round() as u8;
}

/// cleans up orphaned roads
fn clean_roads(world: &mut WorldState) {
    let mut orphans: u32 = 0;
    // sweep through and remove all orphans
    for y in world.min_y() + 1..world.max_y() - 1 {
        for x in world.min_x() + 1..world.max_x() - 1 {
            let left = world.get_road(Region::new(x - 1, y)).lanes_x;
            let right = world.get_road(Region::new(x + 1, y)).lanes_x;
            let above = world.get_road(Region::new(x, y - 1)).lanes_y;
            let below = world.get_road(Region::new(x, y + 1)).lanes_y;
            if left + right + above + below == 0 {
                world.set_road(Region::new(x, y), 0, 0);
                orphans += 1;
            }
        }
    }
    println!("road orphan pass: removed {} orphans", orphans);
}

/// sweep all the horizontal roads, connecting and smoothing out lane transitions
fn extend_roads_x(world: &mut WorldState) {
    let mut x_extends: u32 = 0;
    // cleaning up orphans
    for y in world.min_y()..world.max_y() {
        // sweep rightward and connect or delete orphans
        for x in world.min_x() + 1..world.max_x() - 1 {
            let cur_tile = world.get_road(Region::new(x, y));
            let left_tile = world.get_road(Region::new(x - 1, y));
            let right_tile = world.get_road(Region::new(x + 1, y));
            if cur_tile.lanes_x > 0 && cur_tile.lanes_y > 0 {
                // if there's a y road we can make this an intersection
                continue;
            }
            if cur_tile.lanes_x > 0
                && (left_tile.lanes_x > 0 || right_tile.lanes_x < cur_tile.lanes_x - 1)
                && world.get_pop(Region::new(x, y)) > 0.1
            {
                // we need to continue leftward
                world.set_road(
                    Region::new(x + 1, y),
                    right_tile.lanes_x.max(cur_tile.lanes_x - 1).max(1),
                    right_tile.lanes_y,
                );
                x_extends += 1;
            }
        }
        // sweep leftward and connect or delete orphans
        for x in (world.min_x() + 1..world.max_x() - 1).rev() {
            let cur_tile = world.get_road(Region::new(x, y));
            let left_tile = world.get_road(Region::new(x - 1, y));
            let right_tile = world.get_road(Region::new(x + 1, y));
            if cur_tile.lanes_x > 0 && cur_tile.lanes_y > 0 {
                // if there's a y road we can make an intersection
                continue;
            }
            if cur_tile.lanes_x > 0
                && (right_tile.lanes_x > 0 || left_tile.lanes_x < cur_tile.lanes_x)
                && world.get_pop(Region::new(x, y)) > 0.1
            {
                world.set_road(
                    Region::new(x + 1, y),
                    left_tile.lanes_x.max(cur_tile.lanes_x - 1).max(1),
                    left_tile.lanes_y,
                );
                x_extends += 1;
            }
        }
    }
    println!(
        "road extension pass: extended {} horizontal roads",
        x_extends
    );
}

/// sweep all the vertical roads, connecting gaps and smoothing transitions
fn extend_roads_y(world: &mut WorldState) {
    let mut y_extends: u32 = 0;
    // cleaning up orphans
    for x in world.min_x()..world.max_x() {
        // sweep downward and connect or delete orphans
        for y in world.min_y() + 1..world.max_y() - 1 {
            let cur_tile = world.get_road(Region::new(x, y));
            let above_tile = world.get_road(Region::new(x, y - 1));
            let below_tile = world.get_road(Region::new(x, y + 1));
            // if there's an x road, we can make this a T, so skip this
            if cur_tile.lanes_y > 0 && cur_tile.lanes_x > 0 {
                continue;
            }
            if cur_tile.lanes_y > 0
                && (above_tile.lanes_y > 0 || below_tile.lanes_y < cur_tile.lanes_y)
                && world.get_pop(Region::new(x, y)) > 0.1
            {
                // we need to continue downward
                world.set_road(
                    Region::new(x, y + 1),
                    below_tile.lanes_x,
                    below_tile.lanes_y.max(cur_tile.lanes_y - 1).max(1),
                );
                y_extends += 1;
            }
        }
        // sweep upward and connect
        for y in (world.min_y() + 1..world.max_y() - 1).rev() {
            let cur_tile = world.get_road(Region::new(x, y));
            let above_tile = world.get_road(Region::new(x, y - 1));
            let below_tile = world.get_road(Region::new(x, y + 1));
            if cur_tile.lanes_y > 0 && cur_tile.lanes_x > 0 {
                // if there's an x road, we can make this a T, so skip this
                continue;
            }
            if cur_tile.lanes_y > 0
                && (below_tile.lanes_y > 0 || above_tile.lanes_y < cur_tile.lanes_y)
                && world.get_pop(Region::new(x, y)) > 0.1
            {
                // we need to continue upward
                world.set_road(
                    Region::new(x, y - 1),
                    above_tile.lanes_x,
                    above_tile.lanes_y.max(cur_tile.lanes_y - 1).max(1),
                );
                y_extends += 1;
            }
        }
    }
    println!("road extension pass: extended {} vertical roads", y_extends);
}
