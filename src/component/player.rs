use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Copy, Clone, Component, Debug, Default, Serialize, Deserialize)]
#[storage(VecStorage)]
/// A marker component for player characters.
pub struct Player;
