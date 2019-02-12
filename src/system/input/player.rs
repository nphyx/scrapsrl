use crate::component::{Cursor, MovePlan, Player, Position, Region};
use crate::resource::{GameState, UserInput};
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};
use tcod::input::Key;
use tcod::input::KeyCode::*;

use super::movement_util::get_movement;

/// handle input that controls the player's character
pub struct PlayerInput;
impl<'a> System<'a> for PlayerInput {
    type SystemData = (
        WriteStorage<'a, Cursor>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, MovePlan>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Region>,
        Read<'a, GameState>,
        Write<'a, UserInput>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
      mut cursors,
      mut positions,
      mut plans,
      players,
      mut regions,
      state,
      mut input,
      entities): Self::SystemData,
    ) {
        let mut player_pos: Position = Position::default();
        let mut player_region: Region = Region::default();
        if state.paused {
            return;
        } // no moving while paused
        for (pos, to, region, ..) in (&positions, &mut plans, &mut regions, &players).join() {
            player_pos = *pos;
            player_region = *region;
            if let Some(plan) = get_movement(&input) {
                to.x = plan.x;
                to.y = plan.y;
                input.consume();
                return;
            }
        }

        match input.get() {
            Some(Key { code: Enter, .. })
            | Some(Key {
                code: NumPadEnter, ..
            })
            | Some(Key { code: NumPad5, .. }) => {
                entities
                    .build_entity()
                    .with(Cursor, &mut cursors)
                    .with(
                        Region {
                            x: player_region.x,
                            y: player_region.y,
                        },
                        &mut regions,
                    )
                    .with(MovePlan::default(), &mut plans)
                    .with(
                        Position {
                            x: player_pos.x,
                            y: player_pos.y,
                        },
                        &mut positions,
                    )
                    .build();
                input.consume();
                return;
            }
            _ => {
                return;
            }
        }
    }
}
