use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
/**
 * Solid objects block movement.
 */
#[derive(Copy, Clone, Component, Debug, Default, Deserialize, Serialize)]
#[storage(VecStorage)]
pub struct Solid;
