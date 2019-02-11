use specs::{System, ReadStorage, Read, Write, Join};
use crate::resource::{GameState, UIQueue, AreaMapCollection};
use crate::component::Cursor;
use crate::resource::UserInput;

/// handles game state maintenance before a tick runs
pub struct PreTick;
impl<'a> System<'a> for PreTick {
  type SystemData = (
    ReadStorage<'a, Cursor>,
    Read<'a, UserInput>,
    Read<'a, UIQueue>,
    Read<'a, AreaMapCollection>,
    Write<'a, GameState>
  );

  fn run(&mut self, (cursors, input, ui_queue, maps, mut state): Self::SystemData) {
    if ui_queue.len() > 0 {
      state.ticking = false;
      state.paused = true;
      return;
    } else {
      state.ticking = true;
      state.paused = false;
    }

    let mut cursor_mode: bool = false;
    let mut has_input: bool = false;
    for _ in cursors.join() {
      cursor_mode = true; // if there's a cursor in play, we don't tick
    }

    match input.get() {
      // don't tick if there's no key input this frame
      Some(_) => { has_input = true; },
      None => {}
    }

    // FIXME this is getting junkier the more variables are in play 
    if !maps.populated() {
      state.ticking = false;
      state.input_enabled = false;
    } else if state.fast_forward {
      state.ticking = true;
      state.input_enabled = false;
    } else {
      state.input_enabled = true;
      if has_input { state.ticking = true; }
      else { state.ticking = false; }
      if cursor_mode { state.ticking = false; }
    }

    state.frame += 1;
    if state.ticking {
      state.tick += 1;
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
}

/// Do any game state cleanup that should happen at the end of a turn
pub struct PostTick;
impl<'a> System<'a> for PostTick {
  type SystemData = (
    Write<'a, GameState>
  );

  fn run(&mut self, mut _state: Self::SystemData) {
    /* TODO nothing so far */
  }
}
