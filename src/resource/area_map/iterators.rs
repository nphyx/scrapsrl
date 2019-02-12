use crate::component::Position;
use super::{AreaMap, Tile, WIDTH, HEIGHT};

pub struct AreaMapIter<'a> {
  pub map: &'a AreaMap,
  pub cur: [usize; 2]
}

impl<'a> Iterator for AreaMapIter<'a> {
  type Item = (Position, &'a Tile);

  fn next(&mut self) -> Option<(Position, &'a Tile)> {
    let [x, y] = &mut self.cur;
    if *x >= WIDTH {
      *x = 0;
      *y += 1;
    }
    if *y >= HEIGHT {
      return None; 
    }
    let r = (Position{x:*x as i32, y:*y as i32}, &self.map.tiles[*x][*y]);
    *x += 1;
    Some(r)
  }
}
