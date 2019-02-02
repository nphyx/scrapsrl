use tcod::colors::Color;

// We'll use a basic structure to define our tiles.
#[derive(Clone)]
pub struct Tile<'a> {
  pub ch: char,
  pub color: Color,
  pub solid: bool,
  pub desc: &'a str
}
