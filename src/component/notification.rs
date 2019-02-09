use specs::{Component, VecStorage};

/// An interaction response that raises a notification
#[derive(Clone,Component,Debug,Default)]
#[storage(VecStorage)]
pub struct NotificationInteraction {
  pub header: String,
  pub body: String
}
