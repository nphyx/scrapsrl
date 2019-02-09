use specs::{Entity,Component,VecStorage};
use crate::component::Position;

pub enum InteractionMethod {
  /// check the object (do default action, such as display its notifcation or menu)
  Check
}

/// the target of a player interaction
#[derive(Component)]
#[storage(VecStorage)]
pub struct InteractionTarget {
  /// chosen interaction method
  pub method: InteractionMethod,
  pub pos: Option<Position>,
  pub entity: Option<Entity>
}

impl Default for InteractionTarget {
  fn default() -> InteractionTarget {
    InteractionTarget{
      method: InteractionMethod::Check,
      pos: None,
      entity: None
    }
  }
}
