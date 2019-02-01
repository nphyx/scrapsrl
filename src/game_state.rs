use crate::entity::Character;
use crate::player::Player;
use crate::mapgen::Tiles;
use tcod::map::Map;

pub struct GameState {
    pub score: i32,
    pub player: Player,
    pub map: Map,
    pub tiles: Tiles
}

impl GameState {
    pub fn new(pc: Character, map: Map, tiles: Tiles) -> GameState {
        GameState{
            score: 0,
            player: Player::new(pc),
            map: map,
            tiles: tiles
        }
    }
}
