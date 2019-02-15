use crate::resource::{AreaMapCollection, GameStage, GameState, Templates};
/// stub game stage management
use specs::{Read, System, Write};

pub struct Stage;

impl<'a> System<'a> for Stage {
    type SystemData = (
        Read<'a, Templates>,
        Read<'a, AreaMapCollection>,
        Write<'a, GameState>,
    );

    fn run(&mut self, (templates, maps, mut state): Self::SystemData) {
        match state.stage {
            GameStage::LoadingAssets => {
                if templates.ready {
                    state.stage = GameStage::Initializing;
                }
            }
            GameStage::Initializing => {
                if maps.ready(state.region) {
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
