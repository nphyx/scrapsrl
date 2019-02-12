use specs::{System, Write};
use crate::resource::{UIQueue, UserInput, UIResponse};

#[derive(Default)]
pub struct UIInput;

impl<'a> System<'a> for UIInput {
  type SystemData = (
    Write<'a, UIQueue>,
    Write<'a, UserInput>
  );

  fn run(&mut self, (mut queue, mut input): Self::SystemData) {
    if queue.len() > 0 {
      if let Some(key) = input.get() {
        match queue.next(key) {
          UIResponse::Consumed |
          UIResponse::Completed => { input.consume(); }
          _ => {}
        }
      }
    }
  }
}
