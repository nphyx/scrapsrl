use specs::{Component, VecStorage};
use serde::Serialize;

/// An interaction response that raises a notification
#[derive(Clone,Component,Debug,Default,Serialize)]
#[storage(VecStorage)]
pub struct NotificationInteraction {
  pub header: String,
  pub body: String
}
