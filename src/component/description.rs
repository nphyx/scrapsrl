use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Component, Debug, Default, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Description {
    pub short: String,
    pub long: String,
}

impl Clone for Description {
    fn clone(&self) -> Description {
        Description {
            short: self.short.clone(),
            long: self.long.clone(),
        }
    }
}

impl Description {
    pub fn new(short: &str, long: &str) -> Description {
        Description {
            short: short.to_string(),
            long: long.to_string(),
        }
    }
}
