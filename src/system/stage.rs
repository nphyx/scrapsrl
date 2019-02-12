use crate::resource::{AreaMapCollection, GameStage, GameState};
/// stub game stage management
use specs::{Read, System, Write};

pub struct Stage;

impl<'a> System<'a> for Stage {
    type SystemData = (Read<'a, AreaMapCollection>, Write<'a, GameState>);

    fn run(&mut self, (maps, mut state): Self::SystemData) {
        match state.stage {
            GameStage::LoadingAssets => {
                if state.frame > 30 {
                    // simulate loading :3
                    state.stage = GameStage::Initializing;
                }
            }
            GameStage::Initializing => {
                if state.frame > 45 && maps.ready(state.region) {
                    state.stage = GameStage::Playing;
                }
            }
            GameStage::Playing => {
                if !maps.ready(state.region) {
                    state.stage = GameStage::Initializing;
                }
            }
        }
    }
}
