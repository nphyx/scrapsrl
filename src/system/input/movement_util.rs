use tcod::input::{Key, KeyCode::*};
use crate::resource::UserInput;
use crate::component::MovePlan;

pub fn get_movement(input: &UserInput) -> Option<MovePlan> {
  let mut speed = 1;
  let mut to = MovePlan::default();
  match input.get() {
    Some(Key { shift: true, .. }) => {
      speed = 2;
    },
    Some(Key { code: NoKey, ..}) |
      Some(Key { code: Shift, ..}) => return None,
    _ => {}
  }
  match input.get() {
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
    _ => { return None; }
  }
  Some(to)
}
