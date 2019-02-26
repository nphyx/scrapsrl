use crate::component::*;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::resource::{AreaMaps, CollisionMaps};
use crate::util::clamp;
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};

pub struct Movement;
impl<'a> System<'a> for Movement {
    type SystemData = (
        ReadStorage<'a, Region>,
        WriteStorage<'a, MovePlan>,
        ReadStorage<'a, Solid>,
        WriteStorage<'a, Orientation>,
        WriteStorage<'a, Position>,
        Read<'a, AreaMaps>,
        Write<'a, CollisionMaps>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            regions,
            mut plans,
            solids,
            mut orientations,
            mut positions,
            area_maps,
            mut collision_maps,
            entities,
        ): Self::SystemData,
    ) {
        for (region, orientation, plan, pos, entity) in (
            &regions,
            &mut orientations,
            &mut plans,
            &mut positions,
            &entities,
        )
            .join()
        {
            // guard against entities outside currently loaded map
            if !area_maps.has(*region) {
                continue;
            }
            let map = area_maps.get(*region);
            let new_pos = Position {
                x: clamp(0, MAP_WIDTH - 1, plan.x + pos.x),
                y: clamp(0, MAP_HEIGHT - 1, plan.y + pos.y),
            };
            let mut ok: bool;
            // only solid entities care about collisions, so let's check if it's solid
            let solid: Option<&Solid> = solids.get(entity);
            match solid {
                Some(_) => {
                    // got a solid, so run collision checks
                    match map.get(new_pos) {
                        // check the tileset first
                        Some(tile) => {
                            ok = tile.walkable;
                        }
                        None => {
                            ok = false;
                        } // there's no tile there? don't walk on it then...
                    }
                    // if we're still ok, check if there's a colliding object
                    if ok {
                        ok = !collision_maps.get(*region, new_pos);
                    }
                }
                None => {
                    ok = true;
                } // always ok to move if not solid
            }
            if ok {
                if plan.x > 0 {
                    orientation.dir = Direction::East;
                }
                if plan.x < 0 {
                    orientation.dir = Direction::West;
                }
                if plan.y > 0 {
                    orientation.dir = Direction::South;
                }
                if plan.y < 0 {
                    orientation.dir = Direction::North;
                }
                // do the move if all checks passed
                collision_maps.set(*region, *pos, false);
                collision_maps.set(*region, new_pos, true);
                pos.x = new_pos.x;
                pos.y = new_pos.y;
            }
            plan.x = 0;
            plan.y = 0;
        }
    }
}
