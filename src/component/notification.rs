use specs::{Component, VecStorage};
use serde::{Deserialize,Serialize};

/// An interaction response that raises a notification
#[derive(Clone,Component,Debug,Default,Deserialize,Serialize)]
#[storage(VecStorage)]
pub struct NotificationInteraction {
  pub header: String,
  pub body: String
}
