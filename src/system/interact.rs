/// support player interactions with objects
use crate::component::{Position, NotificationInteraction};
use crate::resource::{InteractionTarget, InteractionMethod, UIQueue, Notification};

#[derive(Default)]
pub struct Notify;

use specs::{System, ReadStorage, Write, Join};
impl<'a> System<'a> for Notify {
  type SystemData = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, NotificationInteraction>,
    Write<'a, InteractionTarget>,
    Write<'a, UIQueue>,
  );

  fn run(&mut self, (positions, notifications, mut target, mut queue): Self::SystemData) {
    let mut matched: bool = false;
    match target.pos {
      Some(t_pos) => {
        for (e_pos, notice) in (&positions, &notifications).join() {
          if matched { break; }
          match target.method {
            InteractionMethod::Check => {
              if *e_pos == t_pos {
                queue.add(Notification::new(notice.header.clone(), notice.body.clone()));
                matched = true;
              }
            }
          }
        }
      },
      None => {}
    }
    target.pos = None;
    target.entity = None;
  }
}
