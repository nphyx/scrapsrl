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
                    // generate pre-apocalypse population density
                    world.set_pop(
                        Region::new(x, y),
                        rand_up(noise.get_fbm([x as f32, y as f32], 32)),
                    );

                    // generate roads
                    let o_x: [f32; 2] = [x as f32 * MAP_WIDTH as f32 / 800.0, y as f32];
                    let o_y: [f32; 2] = [x as f32, y as f32 * MAP_WIDTH as f32 / 800.0];
                    let lanes_x = ((rand_up(noise.get_fbm(o_x, 32)) * 12.0) - 5.0)
                        .max(0.0)
                        .floor() as u8;
                    let lanes_y = ((rand_up(noise.get_fbm(o_y, 32)) * 12.0) - 5.0)
                        .max(0.0)
                        .floor() as u8;
                    world.set_road(Region::new(x, y), lanes_x, lanes_y);
                }
            }
            world.ready = true;
            println!("finished world generation");
        }
    }
}
