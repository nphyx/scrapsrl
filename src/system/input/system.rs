use specs::{System, Write};
use tcod::input::Key;
use tcod::input::KeyCode::*;

use crate::resource::{AreaMapCollection, GameState, RenderMode, UserInput};

/// handle input that should work regardless of game state
pub struct SystemInput;
impl<'a> System<'a> for SystemInput {
    type SystemData = (
        Write<'a, UserInput>,
        Write<'a, AreaMapCollection>,
        Write<'a, GameState>,
    );

    fn run(&mut self, (mut input, mut maps, mut state): Self::SystemData) {
        match input.get() {
            // toggle fullscreen
            Some(Key { code: F11, .. }) => {
                state.fullscreen = !state.fullscreen;
                state.ticking = false; // we need to let the loop execute once for fullscreen to toggle
                input.consume();
                return;
            }
            // toggle fast-forward mode
            Some(Key {
                code: Char,
                printable: '.',
                ..
            }) => {
                println!("toggling fast-forward mode");
                state.fast_forward = !state.fast_forward;
                input.consume();
                return;
            }
            // TODO command line switch to enable/disable debug keys
            // debug render mode toggle
            Some(Key { code: F9, .. }) => match state.render_mode {
                RenderMode::Normal => state.render_mode = RenderMode::Collision,
                RenderMode::Collision => state.render_mode = RenderMode::Normal,
            },
            // regenerate the game map (debug only)
            Some(Key { code: F4, .. }) => {
                println!("DEBUG COMMAND: re-generating map");
                for (_, map) in maps.iter_mut() {
                    map.populated = false;
                }
                state.ticking = false;
                input.consume();
            }
            // change the world seed and regen the map (debug only)
            Some(Key { code: F8, .. }) => {
                println!("DEBUG COMMAND: generating map with new world seed");
                state.world_seed += 1;
                for (_, map) in maps.iter_mut() {
                    map.populated = false;
                }
                state.ticking = false;
                input.consume();
            }
            _ => {}
        }
        // if input is disabled, we'll consume the key now regardless
        if !state.input_enabled {
            input.consume();
        }
    }
}
