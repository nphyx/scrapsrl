use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Clone, Component, Debug, Default, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Description {
    pub short: String,
    pub long: String,
}
