use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Colors {
  fg: tcod::colors::Color,
  bg: tcod::colors::Color
}
