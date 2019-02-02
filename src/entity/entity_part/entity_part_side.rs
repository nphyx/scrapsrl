#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum EntityComponentSide {
  Left,
  Right,
  Front,
  Behind,
  NoSide
}

impl EntityComponentSide {
  pub fn to_string(&self) -> &'static str {
    match self {
      EntityComponentSide::Left => "left",
      EntityComponentSide::Right => "right",
      EntityComponentSide::Front => "front",
      EntityComponentSide::Behind => "behind",
      EntityComponentSide::NoSide => ""
    }
  }
}

