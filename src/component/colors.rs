use specs::{Component, VecStorage};

#[derive(Copy,Clone,Component,Default,Debug)]
#[storage(VecStorage)]
pub struct Colors {
  pub fg: tcod::colors::Color,
  pub bg: tcod::colors::Color
}
