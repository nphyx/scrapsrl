use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Clone, Component, Debug, Default, Deserialize, Serialize)]
#[storage(VecStorage)]
pub struct IconRef {
    pub name: String,
}

impl IconRef {
    pub fn new(name_str: &str) -> IconRef {
        IconRef {
            name: name_str.to_string(),
        }
    }
}
