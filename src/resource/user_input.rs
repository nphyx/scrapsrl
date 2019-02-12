use specs::{Component, DenseVecStorage};
use tcod::input::Key;

#[derive(Component, Debug)]
pub struct UserInput {
    key: Option<Key>,
}

impl Default for UserInput {
    fn default() -> UserInput {
        UserInput { key: None }
    }
}

impl UserInput {
    pub fn consume(&mut self) {
        self.key = None;
    }

    pub fn get(&self) -> Option<Key> {
        // FIXME was this supposed to do something besides passthrough?
        match self.key {
            Some(key) => Some(key),
            None => None,
        }
    }
    pub fn set(&mut self, key: Option<Key>) {
        self.key = key;
    }
}
