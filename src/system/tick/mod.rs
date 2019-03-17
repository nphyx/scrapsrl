use crate::component::Cursor;
use crate::resource::UserInput;
use crate::resource::{RegionMaps, GameState, UIQueue, WorldState};
use specs::{Join, Read, ReadStorage, System, Write};

/// handles game state maintenance before a tick runs
pub struct PreTick;
impl<'a> System<'a> for PreTick {
    type SystemData = (
        ReadStorage<'a, Cursor>,
        Read<'a, UserInput>,
        Read<'a, UIQueue>,
        Read<'a, RegionMaps>,
        Write<'a, GameState>,
        Write<'a, WorldState>,
    );

    fn run(&mut self, (cursors, input, ui_queue, maps, mut state, mut world): Self::SystemData) {
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

        if input.get().is_some() {
            has_input = true;
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
            if has_input {
                state.ticking = true;
            } else {
                state.ticking = false;
            }
            if cursor_mode {
                state.ticking = false;
            }
        }

        state.frame += 1;
        if state.ticking {
            state.tick += 1;
            world.time += (100.0 / 60.0) / 100.0;
            if world.time >= 24.0 {
                world.time = 0.0;
                world.day += 1;
            }
            if world.day >= 365 {
                if (world.year + 1) % 4 == 0 {
                    // it was a leap year! but don't make the first year a leap year, that would be lame
                    if world.day >= 366 {
                        world.day = 0;
                        world.year += 1;
                    }
                } else {
                    world.day = 0;
                    world.year += 1;
                }
            }
        }
    }
}

/// Do any game state cleanup that should happen at the end of a turn
pub struct PostTick;
impl<'a> System<'a> for PostTick {
    type SystemData = (Write<'a, GameState>);

    fn run(&mut self, mut _state: Self::SystemData) {
        /* TODO nothing so far */
    }
}
