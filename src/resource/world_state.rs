use rand::prelude::*;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Copy, Clone, Component, Debug, Deserialize, Serialize)]
#[storage(VecStorage)]
pub struct WorldState {
    /// global RNG seed
    pub seed: u32,

    pub day: u32,
    pub time: f32,
    pub year: u32,
}

impl Default for WorldState {
    fn default() -> WorldState {
        let mut rng = rand::thread_rng();
        WorldState {
            seed: rng.gen_range(0, std::u32::MAX),
            day: 0,
            time: 6.0,
            year: 0,
        }
    }
}

impl WorldState {
    /// 1.0 is noon, 0.0 is midnight
    pub fn time_relative(&self) -> f32 {
        ((self.time * 15.0 * (std::f32::consts::PI / 180.0)).sin() + 1.0) / 2.0
    }
}
