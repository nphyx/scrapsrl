use crate::component::{Cursor, MovePlan, Pos};
use crate::resource::{GameState, InteractionTarget, UserInput};
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};
use tcod::input::Key;
use tcod::input::KeyCode::*;

use super::movement_util::get_movement;

/// handle input that controls the player's cursor
pub struct CursorInput;
impl<'a> System<'a> for CursorInput {
    type SystemData = (
        ReadStorage<'a, Pos>,
        WriteStorage<'a, MovePlan>,
        WriteStorage<'a, Cursor>,
        Write<'a, UserInput>,
        Write<'a, GameState>,
        Write<'a, InteractionTarget>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
      positions,
      mut plans,
      cursors,
      mut input,
      mut state,
      mut target,
      entities): Self::SystemData,
    ) {
        state.looking = false;

        for (pos, to, entity, _) in (&positions, &mut plans, &entities, &cursors).join() {
            state.looking = true;

            if let Some(plan) = get_movement(&input) {
                to.x = plan.x;
                to.y = plan.y;
            }

            match input.get() {
                Some(Key { code: Escape, .. }) => {
                    entities
                        .delete(entity)
                        .expect("tried to delete a non-existent cursor");
                }
                Some(Key { code: Enter, .. })
                | Some(Key {
                    code: NumPadEnter, ..
                }) => {
                    target.pos = Some(*pos);
                    entities
                        .delete(entity)
                        .expect("tried to delete a non-existent cursor");
                }
                _ => {}
            }
            input.consume(); // in any case, cursor prevents further handling
        }
    }
}
