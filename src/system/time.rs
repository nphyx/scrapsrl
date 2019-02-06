/// updates game system time
use specs::{System, Write};
use crate::game_state::GameState;

pub struct Time;
impl<'a> System<'a> for Time {
  type SystemData = Write<'a, GameState>;

  fn run(&mut self, mut state: Self::SystemData) {
    state.skip_next_frame = false; // coordinates map gen passes, TODO event system
    state.world_time = state.world_time + (100.0 / 60.0) / 100.0;
    if state.world_time >= 24.0 {
      state.world_time = 0.0;
      state.world_day += 1;
    } if state.world_day >= 365 {
      if (state.world_year + 1) % 4 == 0 { // it was a leap year! but don't make the first year a leap year, that would be lame
        if state.world_day >= 366 {
          state.world_day = 0;
          state.world_year += 1;
        }
      } else {
        state.world_day = 0;
        state.world_year += 1;
      }
    }
  }
}
