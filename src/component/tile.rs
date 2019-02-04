use std::collections::HashMap;
use tcod::colors::Color;
use specs::{Component, VecStorage};
// use crate::entity::Coord;
use crate::util::ConnectableChars;
use crate::component::{Tile, Position, Color, Description};

// We'll use a basic structure to define our tiles.
#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Tile {
  pub transparent: bool,
  pub walkable: bool,
}

pub struct ConnectedTile;

/*
#[derive(Default)]
pub struct Tiles {
  pub map: TileMap
}

impl<'a> Tiles<'a> {
  pub fn new() -> Tiles<'a> { Tiles{map: TileMap::new()} }
  pub fn insert(&mut self, key: Coord, tile: Tile<'a>) {
    self.map.insert(key, tile);
  }
  pub fn get(&self, key: Coord) -> Option<&Tile> {
    self.map.get(&key)
  }
  pub fn get_ch(&self, key: Coord) -> Option<char> {
    match self.map.get(&key) {
      Some(tile) => Some(tile.ch.clone()),
      None => None
    }
  }

  pub fn connect_tiles(&mut self) {
}
*/

