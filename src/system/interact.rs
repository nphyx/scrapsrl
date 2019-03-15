/// support player interactions with objects
use crate::component::{NotificationInteraction, Pos, Region};
use crate::resource::{GameState, InteractionMethod, InteractionTarget, Notification, UIQueue};

#[derive(Default)]
pub struct Notify;

use specs::{Join, Read, ReadStorage, System, Write};
impl<'a> System<'a> for Notify {
    type SystemData = (
        ReadStorage<'a, Region>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, NotificationInteraction>,
        Write<'a, InteractionTarget>,
        Write<'a, UIQueue>,
        Read<'a, GameState>,
    );

    fn run(
        &mut self,
        (regions, positions, notifications, mut target, mut queue, state): Self::SystemData,
    ) {
        let mut matched: bool = false;
        if let Some(t_pos) = target.pos {
            for (e_region, e_pos, notice) in (&regions, &positions, &notifications).join() {
                if matched {
                    break;
                }
                match target.method {
                    InteractionMethod::Check => {
                        if *e_pos == t_pos && *e_region == state.region {
                            queue.add(Notification::new(
                                notice.header.clone(),
                                notice.body.clone(),
                            ));
                            matched = true;
                        }
                    }
                }
            }
        }
        target.pos = None;
        target.entity = None;
    }
}
