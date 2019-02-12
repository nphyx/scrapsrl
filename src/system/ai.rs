use crate::component::ai_brain::MovementBehavior;
use crate::component::*;
use crate::resource::{AreaMap, GameState};
use rand::prelude::*;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct AI;
impl<'a> System<'a> for AI {
    type SystemData = (
        WriteStorage<'a, AIBrain>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, MovePlan>,
        Read<'a, AreaMap>,
        Read<'a, GameState>,
    );

    fn run(&mut self, (mut brains, positions, mut plans, map, state): Self::SystemData) {
        let mut rng = rand::thread_rng();
        if !state.ticking {
            return;
        } // AI only runs on ticks
        for (mut brain, pos, mut plan) in (&mut brains, &positions, &mut plans).join() {
            match brain.movement_state {
                MovementBehavior::BrownianWalk => {
                    let mut tries: i8 = 0;
                    let mut done: bool = false;
                    while tries < 10 && !done {
                        let to = Position {
                            x: rng.gen_range(-1, 2),
                            y: rng.gen_range(-1, 2),
                        };
                        let target = Position {
                            x: pos.x + to.x,
                            y: pos.y + to.y,
                        };
                        if let Some(tile) = map.get(target) {
                            if tile.walkable {
                                plan.x = to.x;
                                plan.y = to.y;
                                done = true;
                            }
                        }
                        tries += 1;
                    }
                    if !done {
                        // right now, just waits for the turn, then tries again
                        brain.movement_state = MovementBehavior::Idle;
                        break;
                    }
                }
                MovementBehavior::Idle => {
                    brain.movement_state = MovementBehavior::BrownianWalk;
                    break;
                }
                _ => {}
            }
        }
    }
}
