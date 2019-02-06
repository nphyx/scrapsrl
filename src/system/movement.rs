use specs::{System, ReadStorage, WriteStorage, Write, Read, Join};
use crate::util::clamp;
use crate::component::*;
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::game_state::GameState;
use crate::area_map::AreaMap;

pub struct Movement;
impl<'a> System<'a> for Movement {
  type SystemData = (
    ReadStorage<'a, Player>,
    ReadStorage<'a, MovePlan>,
    WriteStorage<'a, Position>,
    Read<'a, AreaMap<'static>>,
    Write<'a, GameState>
  );

  fn run(&mut self, (players, plans, mut positions, map, mut state): Self::SystemData) {
    // handle the case where the player is changing to a new map
    // TODO? maybe this should be its own system
    for(_player, plan, pos) in (&players, &plans, &mut positions).join() {
      let mut area_changed = false;
      let target = Position{
        x: plan.x + pos.x,
        y: plan.y + pos.y
      };
      if target.x >= MAP_WIDTH {
        state.change_area(1, 0);
        pos.x = 0;
        area_changed = true;
      }
      if target.x < 0 {
        state.change_area(-1, 0);
        pos.x = MAP_WIDTH - 1;
      }
      if target.y >= MAP_HEIGHT {
        state.change_area(0, 1);
        pos.y = 0;
        area_changed = true;
      }
      if target.y < 0 {
        state.change_area(0, -1);
        pos.y = MAP_HEIGHT - 1;
        area_changed = true;
      }
      if area_changed { return }
    }
    for(plan, pos) in (&plans, &mut positions).join() {
      let new_position = Position{
        x: clamp(0, MAP_WIDTH - 1, plan.x + pos.x),
        y: clamp(0, MAP_HEIGHT - 1, plan.y + pos.y)
      };
      let map_pair = &(new_position.x, new_position.y);
      let mut ok: bool;
      match map.get(new_position) {
        Some(tile) => { ok = tile.walkable; },
        None => { ok = false; }
      }
      if ok {
        match state.collision_map.get(map_pair) {
          Some(occupied) => {
            if *occupied {
              ok = false;
            }
          },
          _ => { ok = true; }
        }
      }
      if ok {
        state.collision_map.insert((pos.x, pos.y), false);
        pos.x = new_position.x;
        pos.y = new_position.y;
        state.collision_map.insert(*map_pair, true);
      }
    }
  }
}
