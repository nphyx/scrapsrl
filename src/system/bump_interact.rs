/// handles auto-interact when a player bumps against a solid
use specs::{System, ReadStorage, WriteStorage, Write, Join};
use crate::component::*;
use crate::resource::{InteractionTarget};

pub struct BumpInteract;
impl<'a> System<'a> for BumpInteract {
  type SystemData = (
    ReadStorage<'a, Player>,
    ReadStorage<'a, MovePlan>,
    ReadStorage<'a, Solid>,
    ReadStorage<'a, NotificationInteraction>,
    WriteStorage<'a, Position>,
    Write<'a, InteractionTarget>
  );

  fn run(&mut self, (
      players,
      plans,
      solids,
      interactions,
      mut positions,
      mut target): Self::SystemData) {
    // look up player position & plan
    let mut p_pos: Position = Position::default();
    let mut p_plan: MovePlan = MovePlan::default();
    for(plan, pos, _player) in (&plans, &positions, &players).join() {
      if plan.x == 0 && plan.y == 0 { return; } // player isn't moving
      p_pos = *pos;
      p_plan = *plan;
    }
    for(pos, _interaction, _solid) in (&mut positions, &interactions, &solids).join() {
      if p_plan + p_pos == *pos {
        println!("found a bump interaction target at {:?}", pos);
        target.pos = Some(*pos);
      }
    }
  }
}
