use crate::component::*;
use crate::constants::CHUNK_RADIUS;
use crate::resource::{RegionMaps, CollisionMaps, GameState};
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};

pub struct RegionSystem;

impl<'a> System<'a> for RegionSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, IconRef>,
        WriteStorage<'a, MovePlan>,
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Region>,
        Write<'a, CollisionMaps>,
        Write<'a, RegionMaps>,
        Write<'a, GameState>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            players,
            icons,
            mut plans,
            mut positions,
            mut regions,
            mut collision_maps,
            mut maps,
            mut state,
            entities,
        ): Self::SystemData,
    ) {
        let mut player_changed_region = false;
        for (mut region, plan, pos, entity) in
            (&mut regions, &mut plans, &mut positions, &entities).join()
        {
            let is_player: bool;
            match players.get(entity) {
                Some(_) => {
                    is_player = true;
                }
                None => {
                    is_player = false;
                }
            }
            let map = maps.get(state.region);
            let mut change_x: i32 = 0;
            let mut change_y: i32 = 0;
            if plan.x != 0 || plan.y != 0 {
                let ipos: MovePlan = (*pos).into();
                let target = MovePlan {
                    x: plan.x + ipos.x,
                    y: plan.y + ipos.y,
                };
                if target.x >= map.width() as i32 {
                    change_x = 1;
                    pos.x = 0;
                }
                if target.x < 0 {
                    change_x = -1;
                    pos.x = map.width() - 1;
                }
                if target.y >= map.height() as i32 {
                    change_y = 1;
                    pos.y = 0;
                }
                if target.y < 0 {
                    change_y = -1;
                    pos.y = map.height() - 1;
                }
                if change_x != 0 || change_y != 0 {
                    if is_player {
                        println!("changing region to {}, {}", change_x, change_y);
                        state.change_region(change_x, change_y);
                        maps.init(state.region, CHUNK_RADIUS);
                        maps.prune(state.region, CHUNK_RADIUS);
                        collision_maps.init(state.region, CHUNK_RADIUS);
                        collision_maps.prune(state.region, CHUNK_RADIUS);
                        player_changed_region = true;
                    }
                    region.x += change_x;
                    region.y += change_y;
                    plan.x = 0;
                    plan.y = 0;
                }
            }
        } // end entity region change loop
          // prune entities outside the currently loaded regions
        if player_changed_region {
            for (region, _icon, entity) in (&regions, &icons, &entities).join() {
                if !maps.has(*region) {
                    entities.delete(entity).expect("failed to delete entity");
                }
            }
        }
    }
}
