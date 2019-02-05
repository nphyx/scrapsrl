use specs::{System, ReadStorage, Write, Join};
use crate::component::*;
use crate::game_state::GameState;

pub struct CollisionMap;
impl<'a> System<'a> for CollisionMap {
  type SystemData = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Solid>,
    Write<'a, GameState>
  );

  fn run(&mut self, (positions, solids, mut state): Self::SystemData) {
    for(pos, ..) in (&positions, &solids).join() {
      state.collision_map.insert((pos.x, pos.y), true);
    }
    for(pos, ..) in (&positions, !&solids).join() {
      state.collision_map.insert((pos.x, pos.y), false);
    }
  }
}
