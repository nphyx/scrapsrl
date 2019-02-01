use crate::entity::Character;

pub struct GameState {
    pub score: i32,
    pub player_acted: bool,
    pub player: Character
}

impl GameState {
    pub fn new(player: Character) -> GameState {
        GameState{score: 0, player_acted: false, player: player}
    }
}
