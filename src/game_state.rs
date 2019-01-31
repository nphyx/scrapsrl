use crate::entity::Character;

pub struct GameState {
    pub score: i32,
    pub player: Character
}

impl GameState {
    pub fn new(player: Character) -> GameState {
        GameState{score: 0, player: player}
    }
}
