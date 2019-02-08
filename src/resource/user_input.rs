use specs::{Component,DenseVecStorage};
use tcod::input::Key;

#[derive(Component,Debug)]
pub struct UserInput {
  key: Option<Key>
}

impl Default for UserInput {
  fn default() -> UserInput {
    UserInput{key: None}
  }
}

impl UserInput {
  pub fn consume(&mut self) {
    self.key = None;
  }

  pub fn get(&self) -> Option<Key> {
    match self.key {
      Some(key) => { return Some(key.clone()); },
      None => { return None; }
    }
  }
  pub fn set(&mut self, key: Option<Key>) {
    self.key = key;
  }
}
