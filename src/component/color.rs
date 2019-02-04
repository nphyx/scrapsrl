use specs::{Component, VecStorage};

pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8
}

impl Component for Color {
  type Storage = VecStorage<Self>;
}

impl Color {
  pub fn new(r: u8, g: u8, b: u8) -> Color {
    Color{r, g, b}
  }

  pub fn to_tcod(&self) -> tcod::colors::Color {
    tcod::colors::Color::new(self.r, self.g, self.b)
  }
}
