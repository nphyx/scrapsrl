use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

/**
 * Opaque objects block vision.
 */
#[derive(Copy, Clone, Component, Debug, Default, Deserialize, Serialize)]
#[storage(VecStorage)]
pub struct Opaque;
