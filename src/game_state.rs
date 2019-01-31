pub struct GameState {
    pub score: i32
}

impl GameState {
    pub fn new() -> GameState {
        GameState{score: 0}
    }
}
