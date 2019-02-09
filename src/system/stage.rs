/// stub game stage management

use specs::{System, Write};
use crate::resource::{GameState, GameStage};

pub struct Stage;

impl<'a> System<'a> for Stage {
  type SystemData = Write<'a, GameState>;

  fn run(&mut self, mut state: Self::SystemData) {
    match state.stage {
      GameStage::LoadingAssets => {
        if state.frame > 120 { // simulate loading :3
          state.stage = GameStage::Initializing;
        }
      }
      GameStage::Initializing => {
        if state.frame > 180 && state.map_gen_queued == false {
          state.stage = GameStage::Playing;
        }
      }
      GameStage::Playing => {
        if state.map_gen_queued {
          state.stage = GameStage::Initializing;
        }
      }
    }
  }
}
