use specs::{System, ReadStorage, WriteStorage, Write, Read, Join};
use crate::component::*;
use crate::resource::{GameState, AreaMap};

pub struct AreaChange;

impl<'a> System<'a> for AreaChange {

  type SystemData = (
    ReadStorage<'a, Player>,
    WriteStorage<'a, MovePlan>,
    WriteStorage<'a, Position>,
    Read<'a, AreaMap>,
    Write<'a, GameState>
  );

  fn run(&mut self, (
      players,
      mut plans,
      mut positions,
      map,
      mut state): Self::SystemData) {
    for(_player, plan, pos) in (&players, &mut plans, &mut positions).join() {
      let mut area_changed = false;
      if plan.x != 0 || plan.y != 0 {
        let target = Position{
          x: plan.x + pos.x,
          y: plan.y + pos.y
        };
        if target.x >= map.width {
          state.change_area(1, 0);
          pos.x = 0;
          area_changed = true;
        }
        if target.x < 0 {
          state.change_area(-1, 0);
          pos.x = map.width - 1;
          area_changed = true;
        }
        if target.y >= map.height {
          state.change_area(0, 1);
          pos.y = 0;
          area_changed = true;
        }
        if target.y < 0 {
          state.change_area(0, -1);
          pos.y = map.height - 1;
          area_changed = true;
        }
        if area_changed {
          plan.x = 0;
          plan.y = 0;
          return;
        }
      }
    }
  }
}
