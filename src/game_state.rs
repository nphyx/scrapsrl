// use crate::mapgen::Tiles;
// use tcod::map::Map;

#[derive(Default)]
pub struct GameState {
//  pub map: Map,
  pub frame: u32,
  pub world_time: f32,
  pub world_day: u32,
  pub world_year: u32
}

impl GameState {
  pub fn new() -> GameState {
    GameState{
      // map: map,
      frame: 0,
      world_time: 6.0,
      world_day: 0,
      world_year: 0
    }
  }

  /*
  TODO: systems for these

  pub fn tick(&mut self) {
    self.world_time = self.world_time + (100.0 / 60.0) / 100.0;
    if self.world_time >= 24.0 {
      self.world_time = 0.0;
      self.world_day += 1;
    } if self.world_day >= 365 {
      if (self.world_year + 1) % 4 == 0 { // it was a leap year! but don't make the first year a leap year, that would be lame
        if self.world_day >= 366 {
          self.world_day = 0;
          self.world_year += 1;
        }
      } else {
        self.world_day = 0;
        self.world_year += 1;
      }
    }
  }

  /// 1.0 is noon, 0.0 is midnight
  pub fn world_time_relative(&self) -> f32 {
    ((self.world_time * 15.0 * (std::f32::consts::PI / 180.0)).sin() + 1.0) / 2.0
  }
  */
}
