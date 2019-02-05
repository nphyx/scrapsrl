use specs::{Component,DenseVecStorage};
use tcod::input::Key;

#[derive(Component,Default,Debug)]
pub struct UserInput {
  pub key: Option<Key>
}

