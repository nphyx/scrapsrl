use serde::{Serialize,Deserialize};
use specs::{Component, VecStorage, Entity};

#[derive(Clone,Serialize,Deserialize)]
pub enum MovementBehavior {
  Idle,
  BrownianWalk,
  Pursue
}

#[derive(Clone,Serialize,Deserialize)]
pub enum Attitude {
  Passive
}

/// the "brain" of an NPC, aimed at being a basic finite state machine
#[derive(Component,Clone,Serialize,Deserialize)]
#[storage(VecStorage)]
pub struct AIBrain {
  pub movement_state: MovementBehavior,
  pub attitude: Attitude,
  #[serde(skip)] 
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
