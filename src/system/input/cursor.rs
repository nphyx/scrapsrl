use tcod::input::Key;
use tcod::input::KeyCode::*;
use specs::{System, Write, WriteStorage, Join, Entities};
use crate::component::{MovePlan, UserInput, Cursor};

use super::movement_util::get_movement;

/// handle input that controls the player's cursor
pub struct CursorInput;
impl<'a> System<'a> for CursorInput {
  type SystemData = (
    WriteStorage<'a, MovePlan>,
    WriteStorage<'a, Cursor>,
    Write<'a, UserInput>,
    Entities<'a>
  );

  fn run(&mut self, (
      mut plans,
      cursors,
      mut input,
      entities): Self::SystemData) {
    for (to, entity, _) in (&mut plans, &entities, &cursors).join() {
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
        /*
        // TODO reimplement me
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
        _ => {}
      }
      input.consume(); // in any case, cursor prevents further handling
    }
  }
}
