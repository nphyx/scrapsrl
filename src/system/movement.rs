use specs::{System, ReadStorage, WriteStorage, Write, Read, Join, Entities};
use crate::util::clamp;
use crate::component::*;
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::resource::{GameState, AreaMap};

pub struct Movement;
impl<'a> System<'a> for Movement {
  type SystemData = (
    ReadStorage<'a, Player>,
    WriteStorage<'a, MovePlan>,
    ReadStorage<'a, Solid>,
    WriteStorage<'a, Position>,
    Read<'a, AreaMap>,
    Write<'a, GameState>,
    Entities<'a>
  );

  fn run(&mut self, (
      players,
      mut plans,
      solids,
      mut positions,
      map,
      mut state,
      entities): Self::SystemData) {
    // handle the case where the player is changing to a new map
    for(plan, pos, entity) in (&mut plans, &mut positions, &entities).join() {
      let new_position = Position{
        x: clamp(0, MAP_WIDTH - 1, plan.x + pos.x),
        y: clamp(0, MAP_HEIGHT - 1, plan.y + pos.y)
      };
      let map_pair = &(new_position.x, new_position.y);
      let mut ok: bool;
      let solid: Option<&Solid> = solids.get(entity);
      match solid { // if entity is solid check collisions
        Some(_) => { // got a solid, so run collision checks
          match map.get(new_position) { // check the tileset first
            Some(tile) => { ok = tile.walkable; },
            None => { ok = false; } // there's no tile there? don't walk on it then...
          }
          if ok { // if we're still ok, check if there's a colliding object
            match state.collision_map.get(map_pair) {
              Some(occupied) => {
                if *occupied {
                  ok = false;
                }
              },
              _ => { ok = true; }
            }
          }
          if ok { // is solid, so update collision map
            state.collision_map.insert((pos.x, pos.y), false);
            state.collision_map.insert(*map_pair, true);
          }
        }
        None => { ok = true; } // always ok to move if not solid
      }
      if ok { // do the move if all checks passed
        pos.x = new_position.x;
        pos.y = new_position.y;
      }
      plan.x = 0;
      plan.y = 0;
    }
  }
}
