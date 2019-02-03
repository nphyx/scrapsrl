use std::collections::HashMap;
use tcod::colors::Color;
use crate::entity::Coord;
use crate::util::ConnectableChars;

// We'll use a basic structure to define our tiles.
#[derive(Clone)]
pub struct Tile<'a> {
  pub ch: char,
  pub color: Color,
  pub solid: bool,
  pub desc: &'a str
}

pub type TileMap<'a> = HashMap<Coord, Tile<'a>>;

pub struct Tiles<'a> {
  pub map: TileMap<'a>
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
    let connectables = ConnectableChars::new();

    let mut queue: Vec<(Coord, Tile)> = Vec::new();

    for (coord, tile) in self.map.iter() {
      let orig = tile.ch;
      match connectables.connect(
          &orig,
          self.get_ch(Coord{x:coord.x, y:coord.y - 1}),
          self.get_ch(Coord{x:coord.x, y:coord.y + 1}),
          self.get_ch(Coord{x:coord.x - 1, y:coord.y}),
          self.get_ch(Coord{x:coord.x + 1, y:coord.y})) {
        Some(chosen) => {
          queue.push((*coord, Tile{
            color: tile.color,
            ch: chosen,
            solid: tile.solid,
            desc: tile.desc
          }));
        },
        None => {}
      }
    }

    while  queue.len() > 0  {
      let item = queue.pop();
      match item {
        Some(item) => {
          let (coord, tile) = item;
          self.map.insert(coord, tile);
        },
        None => break
      }
    }
  }
}

