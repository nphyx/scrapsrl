#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum EntityComponentLocation {
  Above,
  Under,
  On,
  Side,
  Inside,
  NoSide
}

impl EntityComponentLocation {
  pub fn to_string(&self) -> &'static str {
    match self {
      EntityComponentLocation::Above => "above",
      EntityComponentLocation::Under => "under",
      EntityComponentLocation::On => "on",
      EntityComponentLocation::Side => "beside",
      EntityComponentLocation::Inside => "inside",
      EntityComponentLocation::NoSide => ""
    }
  }
}
