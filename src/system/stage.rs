/// stub game stage management

use specs::{System, Write, Read};
use crate::resource::{GameState, GameStage, AreaMapCollection};

pub struct Stage;

impl<'a> System<'a> for Stage {
  type SystemData = (
    Read<'a, AreaMapCollection>,
    Write<'a, GameState>
  );

  fn run(&mut self, (maps, mut state): Self::SystemData) {
    match state.stage {
      GameStage::LoadingAssets => {
        if state.frame > 120 { // simulate loading :3
          state.stage = GameStage::Initializing;
        }
      }
      GameStage::Initializing => {
        if state.frame > 180 && maps.populated() {
          state.stage = GameStage::Playing;
        }
      }
      GameStage::Playing => {
        if !maps.populated() {
          state.stage = GameStage::Initializing;
        }
      }
    }
  }
}
