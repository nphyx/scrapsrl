use specs::{Component, VecStorage, Entity};

pub enum MovementBehavior {
  Idle,
  BrownianWalk,
  Pursue
}

pub enum Attitude {
  Passive
}

/// the "brain" of an NPC, aimed at being a basic finite state machine
#[derive(Component)]
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
