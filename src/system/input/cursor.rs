use tcod::input::Key;
use tcod::input::KeyCode::*;
use specs::{System, Write, WriteStorage, ReadStorage, Join, Entities};
use crate::component::{MovePlan, Cursor, Position};
use crate::resource::{GameState, UserInput, InteractionTarget};

use super::movement_util::get_movement;

/// handle input that controls the player's cursor
pub struct CursorInput;
impl<'a> System<'a> for CursorInput {
  type SystemData = (
    ReadStorage<'a, Position>,
    WriteStorage<'a, MovePlan>,
    WriteStorage<'a, Cursor>,
    Write<'a, UserInput>,
    Write<'a, GameState>,
    Write<'a, InteractionTarget>,
    Entities<'a>
  );

  fn run(&mut self, (
      positions,
      mut plans,
      cursors,
      mut input,
      mut state,
      mut target,
      entities): Self::SystemData) {
    state.looking = false;

    for (pos, to, entity, _) in (&positions, &mut plans, &entities, &cursors).join() {
      state.looking = true;

      match get_movement(&input) {
        Some(plan) => {
          to.x = plan.x;
          to.y = plan.y;
        },
        _ => {}
      }

      match input.get() {
        Some(Key { code: Escape, .. }) => {
          entities.delete(entity).expect("tried to delete a non-existent cursor");
        },
        Some(Key { code: Enter, .. }) |
        Some(Key { code: NumPadEnter, ..}) => {
          target.pos = Some(pos.clone());
          entities.delete(entity).expect("tried to delete a non-existent cursor");
        }
        _ => {}
      }
      input.consume(); // in any case, cursor prevents further handling
    }
  }
}
