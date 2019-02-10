use serde::Serialize;
use specs::{Component, VecStorage, Entity};

#[derive(Clone,Serialize)]
pub enum MovementBehavior {
  Idle,
  BrownianWalk,
  Pursue
}

#[derive(Clone,Serialize)]
pub enum Attitude {
  Passive
}

/// the "brain" of an NPC, aimed at being a basic finite state machine
#[derive(Component,Clone)]
#[storage(VecStorage)]
pub struct AIBrain {
  pub movement_state: MovementBehavior,
  pub attitude: Attitude,
  pub target: Option<Entity>
}

impl Default for AIBrain {
  fn default() -> AIBrain {
    AIBrain{
      movement_state: MovementBehavior::Idle,
      attitude: Attitude::Passive,
      target: None
    }
  }
}

use serde::ser::{Serializer, SerializeStruct};
impl Serialize for AIBrain {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let mut s = serializer.serialize_struct("AIBrain", 2)?;
    s.serialize_field("movement_state", &self.movement_state)?;
    s.serialize_field("attitude", &self.attitude)?;
    s.end()
  }
}
