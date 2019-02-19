use specs::{Read, System, Write};

use crate::component::Region;
use crate::resource::{GameStage, GameState, WorldState};

pub struct WorldGen;

impl<'a> System<'a> for WorldGen {
    type SystemData = (Read<'a, GameState>, Write<'a, WorldState>);

    fn run(&mut self, (state, mut world): Self::SystemData) {
        if !world.ready && state.stage == GameStage::Initializing {
            // it would be more efficient to do this on demand, but
            // later on we'll want to draw a world map and we can't be
            // doing this every frame
            for x in world.min_x()..world.max_x() {
                for y in world.min_y()..world.max_y() {
                    let lanes_x = if y % 2 == 0 { 2 } else { 0 };
                    let lanes_y = if x % 2 == 0 { 2 } else { 0 };
                    world.set_road(Region::new(x, y), lanes_x, lanes_y);
                }
            }
        }
    }
}
