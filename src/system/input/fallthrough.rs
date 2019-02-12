use crate::resource::{GameState, UserInput};
use specs::{System, Write};
use tcod::input::Key;
use tcod::input::KeyCode::*;

/// handle hotkeys that should work only if nothing else has consumed them
pub struct FallthroughInput;
impl<'a> System<'a> for FallthroughInput {
    type SystemData = (Write<'a, UserInput>, Write<'a, GameState>);

    fn run(&mut self, (mut input, mut state): Self::SystemData) {
        if let Some(Key { code: Escape, .. }) = input.get() {
            state.close_game = true;
        }
        input.consume();
    }
}
