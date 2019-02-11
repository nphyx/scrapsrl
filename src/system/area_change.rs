use specs::{System, ReadStorage, WriteStorage, Write, Read, Join};
use crate::component::*;
use crate::resource::{GameState, AreaMapCollection};
use crate::constants::CHUNK_RADIUS;

pub struct AreaChange;

impl<'a> System<'a> for AreaChange {

  type SystemData = (
    ReadStorage<'a, Player>,
    WriteStorage<'a, MovePlan>,
    WriteStorage<'a, Position>,
    Write<'a, AreaMapCollection>,
    Write<'a, GameState>
  );

  fn run(&mut self, (
      players,
      mut plans,
      mut positions,
      mut maps,
      mut state): Self::SystemData) {
    for(_player, plan, pos) in (&players, &mut plans, &mut positions).join() {
      let map = maps.get(state.area_offset);
      let mut change_x: i32 = 0;
      let mut change_y: i32 = 0;
      if plan.x != 0 || plan.y != 0 {
        let target = Position{
          x: plan.x + pos.x,
          y: plan.y + pos.y
        };
        if target.x >= map.width {
          change_x = 1;
          pos.x = 0;
        }
        if target.x < 0 {
          change_x = -1;
          pos.x = map.width - 1;
        }
        if target.y >= map.height {
          change_y = 1;
          pos.y = 0;
        }
        if target.y < 0 {
          change_y = -1;
          pos.y = map.height - 1;
        }
        if change_x != 0 && change_y != 0 {
          state.change_area(change_x, change_y);
          maps.init(state.area_offset, CHUNK_RADIUS);
          plan.x = 0;
          plan.y = 0;
          return;
        }
      }
    }
  }
}
