use crate::component::Position;
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
mod connectable_char;
pub mod icons;
pub mod colors;
pub use self::connectable_char::ConnectableChars;

pub fn clamp<T>(a: T, b: T, x: T) -> T  where T: std::cmp::PartialOrd {
  if x < a { a } else if x > b { b } else { x }
}

pub fn distance(p: Position, d: Position) -> f32 {
  return ((d.x as f32 - p.x as f32).powf(2.0) + (d.y as f32 - p.y as f32).powf(2.0)).sqrt()
}

pub fn plan(&to: &Position, map: &tcod::map::Map) -> Option<Position> {
  let planned = Position{
    x: clamp(0, MAP_WIDTH - 1, to.x),
    y: clamp(0, MAP_HEIGHT - 1, to.y)
  };
  if map.is_walkable(planned.x, planned.y) {
    return Some(planned)
  }
  None
}
