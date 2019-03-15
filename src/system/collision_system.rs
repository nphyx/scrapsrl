use crate::component::*;
use crate::resource::CollisionMaps;
use specs::{Join, ReadStorage, System, Write};

pub struct CollisionSystem;
impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        ReadStorage<'a, Region>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Solid>,
        Write<'a, CollisionMaps>,
    );

    fn run(&mut self, (regions, positions, solids, mut collision_maps): Self::SystemData) {
        // this just checks for entities that don't move and updates collision map
        for (region, pos, ..) in (&regions, &positions, &solids).join() {
            collision_maps.set(*region, *pos, true);
        }
    }
}
