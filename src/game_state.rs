// use crate::mapgen::Tiles;
// use tcod::map::Map;
use std::collections::HashMap;

#[derive(Default)]
pub struct GameState {
  pub collision_map: HashMap<(i32, i32), bool>,
  pub frame: u32,
  pub fullscreen: bool,
  pub close_game: bool,
  pub world_time: f32,
  pub world_day: u32,
  pub world_year: u32
}

impl GameState {
  pub fn new() -> GameState {
    GameState{
      collision_map: HashMap::new(),
      frame: 0,
      fullscreen: false,
      close_game: false,
      world_time: 6.0,
      world_day: 0,
      world_year: 0
    }
  }

  /// 1.0 is noon, 0.0 is midnight
  pub fn world_time_relative(&self) -> f32 {
    ((self.world_time * 15.0 * (std::f32::consts::PI / 180.0)).sin() + 1.0) / 2.0
  }
}
