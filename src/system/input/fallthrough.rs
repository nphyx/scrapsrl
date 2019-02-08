use tcod::input::Key;
use tcod::input::KeyCode::*;
use specs::{System,Write};
use crate::resource::{UserInput, GameState};

/// handle hotkeys that should work only if nothing else has consumed them
pub struct FallthroughInput;
impl<'a> System<'a> for FallthroughInput {
  type SystemData = (
    Write<'a, UserInput>,
    Write<'a, GameState>
  );

  fn run(&mut self, (mut input, mut state): Self::SystemData) {
    match input.get() {
      Some(Key { code: Escape, .. }) => {
        state.close_game = true;
      },
      _ => {}
    }
    input.consume();
  }
}
