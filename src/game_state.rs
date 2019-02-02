use crate::mapgen::Tiles;
use tcod::map::Map;

pub struct GameState<'a> {
  pub map: Map,
  pub tiles: Tiles<'a>
}

impl <'a>GameState<'a> {
  pub fn new(map: Map, tiles: Tiles) -> GameState {
    GameState{
      map: map,
      tiles: tiles
    }
  }
}
