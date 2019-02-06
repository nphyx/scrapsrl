use tcod::input::Key;
use tcod::input::KeyCode::*;
use specs::{System,Write,WriteStorage,ReadStorage,Join};

use crate::game_state::GameState;
use crate::component::*;

/// handle hotkeys that should work regardless of game state (fullscreen toggle, etc)
pub struct HandleSystemInput;
impl<'a> System<'a> for HandleSystemInput {
  type SystemData = (
    Write<'a, UserInput>,
    Write<'a, GameState>
  );

  fn run(&mut self, (mut input, mut state): Self::SystemData) {
    match input.key {
      Some(Key { code: F11, .. }) => {
        state.fullscreen = true;
        return;
      },
      Some(_key) => {
        return;
      }
      _ => {}
    }
    input.key = None;
  }
}

/// handle input that controls the player's character
pub struct HandlePlayerInput;
impl<'a> System<'a> for HandlePlayerInput {
  type SystemData = (
    WriteStorage<'a, MovePlan>,
    ReadStorage<'a, Player>,
    Write<'a, UserInput>,
  );

  fn run(&mut self, (mut plans, players, mut input): Self::SystemData) {
    for (to, ..) in (&mut plans, &players).join() {
      to.x = 0;
      to.y = 0;
      let mut speed = 1;
      match input.key {
        Some(Key { shift: true, .. }) => {
          speed = 2;
        },
        Some(Key { code: NoKey, ..}) |
        Some(Key { code: Shift, ..}) => return,
        _ => {}
      }
      match input.key {
        Some(Key { code: NumPad7, .. }) => { // up-left
          to.x = -speed;
          to.y = -speed;
        },
        Some(Key { code: NumPad8, .. }) => { // up
          to.y = -speed;
        },
        Some(Key { code: NumPad9, .. }) => { // up-right
          to.x = speed;
          to.y = -speed;
        },
        Some(Key { code: NumPad1, .. }) => { // down-left
          to.x = -speed;
          to.y = speed;
        },
        Some(Key { code: NumPad2, .. }) => { // down
          to.y = speed;
        },
        Some(Key { code: NumPad3, .. }) => { // down-right
          to.x = speed;
          to.y = speed;
        },
        Some(Key { code: NumPad4, .. }) => { // left
          to.x = -speed;
        },
        Some(Key { code: NumPad6, .. }) => { // right
          to.x = speed;
        },
        /* TODO reimplement me
        Some(Key { code: NumPad5, .. }) => { // interact
          self.cursor.active = !self.cursor.active;
        },
        Some(Key { code: Escape, .. } => {
          if self.cursor.active {
            self.cursor.active = false; 
            return false;
          }
        },
        Some(Key { code: Enter, .. }) |
        Some(Key { code: NumPadEnter, ..}) => {
          if self.cursor.active {
            self.wants_interact_at = Some(self.cursor.pos.clone());
            self.cursor.active = false;
          }
          else {
            self.cursor.active = true;
          }
        }
        */
        _ => { return; }
      }
      input.key = None;
    } // end for
  }
}

/// handle hotkeys that should work only if nothing else has consumed them
pub struct HandleFallthroughInput;
impl<'a> System<'a> for HandleFallthroughInput {
  type SystemData = (
    Write<'a, UserInput>,
    Write<'a, GameState>
  );

  fn run(&mut self, (mut input, mut state): Self::SystemData) {
    match input.key {
      Some(Key { code: Escape, .. }) => {
        state.close_game = true;
      },
      _ => { return; }
    }
    input.key = None;
  }
}
