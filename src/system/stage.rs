use crate::resource::{RegionMaps, Assets, GameStage, GameState};
/// stub game stage management
use specs::{Read, System, Write};

pub struct Stage;

impl<'a> System<'a> for Stage {
    type SystemData = (
        Read<'a, Assets>,
        Read<'a, RegionMaps>,
        Write<'a, GameState>,
    );

    fn run(&mut self, (assets, maps, mut state): Self::SystemData) {
        match state.stage {
            GameStage::LoadingAssets => {
                if assets.ready {
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
