use specs::{System, ReadStorage, WriteStorage, Write, Read, Join, Entities};
use crate::util::clamp;
use crate::component::*;
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::resource::{AreaMapCollection,CollisionMaps};

pub struct Movement;
impl<'a> System<'a> for Movement {
  type SystemData = (
    ReadStorage<'a, Region>,
    WriteStorage<'a, MovePlan>,
    ReadStorage<'a, Solid>,
    WriteStorage<'a, Position>,
    Read<'a, AreaMapCollection>,
    Write<'a, CollisionMaps>,
    Entities<'a>
  );

  fn run(&mut self, (
      regions,
      mut plans,
      solids,
      mut positions,
      area_maps,
      mut collision_maps,
      entities): Self::SystemData) {
    for(region, plan, pos, entity) in (&regions, &mut plans, &mut positions, &entities).join() {
      // guard against entities outside currently loaded map
      if !area_maps.has(region) { 
        println!("skipping an entity because it's outside the loaded area");
        continue;
      } 
      let map = area_maps.get(region);
      let new_pos = Position{
        x: clamp(0, MAP_WIDTH - 1, plan.x + pos.x),
        y: clamp(0, MAP_HEIGHT - 1, plan.y + pos.y)
      };
      let mut ok: bool;
      // only solid entities care about collisions, so let's check if it's solid
      let solid: Option<&Solid> = solids.get(entity);
      match solid {
        Some(_) => { // got a solid, so run collision checks
          match map.get(new_pos) { // check the tileset first
            Some(tile) => { ok = tile.walkable; },
            None => { ok = false; } // there's no tile there? don't walk on it then...
          }
          // if we're still ok, check if there's a colliding object 
          if ok { ok = !collision_maps.get(&region, &new_pos); }
        }
        None => { ok = true; } // always ok to move if not solid
      }
      if ok { // do the move if all checks passed
        collision_maps.set(&region, &pos, false);
        collision_maps.set(&region, &new_pos, true);
        pos.x = new_pos.x;
        pos.y = new_pos.y;
      }
      plan.x = 0;
      plan.y = 0;
    }
  }
}
