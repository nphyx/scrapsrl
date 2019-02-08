use tcod::input::Key;
use tcod::input::KeyCode::*;
use specs::{System, Write, WriteStorage, ReadStorage, Join, Entities};
use crate::component::{MovePlan, Player, Position, Cursor};
use crate::resource::UserInput;

use super::movement_util::get_movement;

/// handle input that controls the player's character
pub struct PlayerInput;
impl<'a> System<'a> for PlayerInput {
  type SystemData = (
    WriteStorage<'a, Cursor>,
    WriteStorage<'a, Position>,
    WriteStorage<'a, MovePlan>,
    ReadStorage<'a, Player>,
    Write<'a, UserInput>,
    Entities<'a>
  );

  fn run(&mut self, (
      mut cursors,
      mut positions,
      mut plans,
      players,
      mut input,
      entities): Self::SystemData) {
    let mut player_pos: Position = Position::default();
    for (pos, to, ..) in (&positions, &mut plans, &players).join() {
      player_pos.x = pos.x;
      player_pos.y = pos.y;
      match get_movement(&input) {
        Some(plan) => {
          to.x = plan.x;
          to.y = plan.y;
          input.consume();
          return;
        },
        _ => {}
      }
    }

    match input.get() {
      Some(Key { code: Enter, .. }) |
      Some(Key { code: NumPadEnter, ..}) | 
      Some(Key { code: NumPad5, .. }) => {
        entities.build_entity()
          .with(Cursor, &mut cursors)
          .with(MovePlan::default(), &mut plans)
          .with(Position{x:player_pos.x, y:player_pos.y}, &mut positions)
          .build();
        input.consume();
        return;
      },
      _ => { return; }
    }
  }
}
