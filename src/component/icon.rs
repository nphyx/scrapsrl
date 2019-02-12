use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Copy, Clone, Component, Debug, Default, Deserialize, Serialize)]
#[storage(VecStorage)]
pub struct Icon {
    pub ch: char,
}
