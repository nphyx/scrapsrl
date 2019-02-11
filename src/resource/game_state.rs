use std::collections::HashMap;
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::resource::area_map::Offset;

pub enum GameStage {
  LoadingAssets,
  Initializing,
  Playing
}

/// game state collects a bunch of globally needed data and coordination info in one
/// place. Doesn't make a lot of sense to make these individual resources, causes too
/// much boilerplate in Systems
pub struct GameState {
  pub stage: GameStage,
  /// if true, game will close on next game loop pass
  pub close_game: bool,
  /// a boolean map of solid entities, for checking entity collision
  pub collision_map: HashMap<(i32, i32), bool>,
  /// frame counter
  pub frame: u32,
  /// tick counter
  pub tick: u32,
  /// fullscreen state
  pub fullscreen: bool,
  /// the game is completely paused. Nothing happens except system-level input
  /// (main menu, close game, etc)
  pub paused: bool,
  /// the game is being fast-forwarded, and will not accept player motion
  pub fast_forward: bool,
  /// when the game is not ticking, AI doesn't take its turn and time doesn't advance
  /// but some player actions can still be taken (navigate menus, inventory, look around)
  pub ticking: bool,
  /// player is using a cursor to look around
  pub looking: bool,
  /// game controls enabled (does not affect system-level input: fullscreen, quit, etc)
  pub input_enabled: bool,
  /// global RNG seed
  pub world_seed: u32,

  /// tracks the X,Y offset of the current map from 0, 0
  pub area_offset: Offset,

  pub world_day: u32,
  pub world_time: f32,
  pub world_year: u32,
}

impl Default for GameState {
  fn default() -> GameState {
    GameState{
      stage: GameStage::LoadingAssets,
      close_game: false,
      collision_map: HashMap::new(),
      frame: 0,
      tick: 0,

      fullscreen: true,
      paused: false,
      fast_forward: false,
      input_enabled: false,
      ticking: true,

      looking: false,

      world_seed: 0,

      area_offset: [0, 0],

      world_day: 0,
      world_time: 18.0,
      world_year: 0
    }
  }
}

impl GameState {
  /// 1.0 is noon, 0.0 is midnight
  pub fn world_time_relative(&self) -> f32 {
    ((self.world_time * 15.0 * (std::f32::consts::PI / 180.0)).sin() + 1.0) / 2.0
  }

  pub fn change_area(&mut self, x_change: i32, y_change: i32) {
    self.area_offset[0] += x_change * MAP_WIDTH;
    self.area_offset[0] += y_change * MAP_HEIGHT;
  }
}
