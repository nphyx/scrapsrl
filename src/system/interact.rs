/// support player interactions with objects
use crate::component::{Position, NotificationInteraction, Region};
use crate::resource::{InteractionTarget, InteractionMethod, UIQueue, Notification, GameState};

#[derive(Default)]
pub struct Notify;

use specs::{System, ReadStorage, Read, Write, Join};
impl<'a> System<'a> for Notify {
  type SystemData = (
    ReadStorage<'a, Region>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, NotificationInteraction>,
    Write<'a, InteractionTarget>,
    Write<'a, UIQueue>,
    Read<'a, GameState>
  );

  fn run(&mut self, (regions, positions, notifications, mut target, mut queue, state): Self::SystemData) {
    let mut matched: bool = false;
    match target.pos {
      Some(t_pos) => {
        for (e_region, e_pos, notice) in (&regions, &positions, &notifications).join() {
          if matched { break; }
          match target.method {
            InteractionMethod::Check => {
              if *e_pos == t_pos && *e_region == state.region {
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
