use tcod::colors::Color;
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::component::Position;

const WIDTH: usize = MAP_WIDTH as usize;
const HEIGHT: usize = MAP_HEIGHT as usize;

#[derive(Copy,Clone)]
pub struct Tile<'a> {
  pub icon: char,
  pub fg: Color,
  pub bg: Color,
  pub transparent: bool,
  pub walkable: bool,
  pub desc_short: &'a str,
  pub desc_long: &'a str
}

impl<'a> Default for Tile<'a> {
  fn default() -> Tile<'a> {
    Tile{
      icon: '?',
      fg: Color::new(255, 255, 255),
      bg: Color::new(0, 0, 0),
      transparent: true,
      walkable: true,
      desc_short: "",
      desc_long: ""
    }
  }
}

pub struct AreaMap<'a> {
  tiles: [[Tile<'a>; HEIGHT]; WIDTH]
}

pub struct AreaMapIter<'a> {
  map: &'a AreaMap<'a>,
  cur: [usize; 2]
}

impl <'a>Iterator for AreaMapIter<'a> {
  type Item = (Position, Tile<'a>);

  fn next(&mut self) -> Option<(Position, Tile<'a>)> {
    let [x, y] = &mut self.cur;
    if *x >= WIDTH {
      *x = 0;
      *y += 1;
    }
    if *y >= HEIGHT {
      return None; 
    }
    let r = (Position{x:*x as i32, y:*y as i32}, self.map.tiles[*x][*y]);
    *x += 1;
    Some(r)
  }
}

impl<'a> Default for AreaMap<'a> {
  fn default() -> AreaMap<'a> {
    let tiles = [[Tile::default(); HEIGHT]; WIDTH];
    AreaMap{tiles}
  }
}

impl<'a> AreaMap<'a> {
  pub fn wipe(&mut self) {
    let tile = Tile::default();
    for x in 0..WIDTH {
      for y in 0..HEIGHT {
        self.tiles[x][y] = tile;
      }
    }
  }

  pub fn get(&self, pos: Position) -> &Tile {
    &(self.tiles[pos.x as usize][pos.y as usize])
  }

  pub fn get_mut(&mut self, pos: Position) -> &mut Tile<'a> {
    &mut(self.tiles[pos.x as usize][pos.y as usize])
  }

  pub fn set(&mut self, pos: Position, tile: Tile<'a>) {
    self.tiles[pos.x as usize][pos.y as usize] = tile;
  }

  pub fn iter(&'a self) -> AreaMapIter<'a> {
    AreaMapIter{
      map: self,
      cur: [0, 0]
    }
  }
}
