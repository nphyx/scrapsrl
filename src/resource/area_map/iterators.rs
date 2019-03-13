use super::{AreaMap, Tile};
use crate::component::Position;

pub struct AreaMapIter<'a> {
    pub map: &'a AreaMap,
    pub cur: [usize; 2],
}

impl<'a> Iterator for AreaMapIter<'a> {
    type Item = (Position, &'a Tile);

    fn next(&mut self) -> Option<(Position, &'a Tile)> {
        let [mut x, mut y] = self.cur.clone();
        let w = self.map.width as usize;
        let h = self.map.height as usize;
        if x >= w {
            x = 0;
            y += 1;
        }
        if y >= h {
            return None;
        }
        let r = (
            Position {
                x: x as i32,
                y: y as i32,
            },
            self.map.tiles.get(x).unwrap().get(y).unwrap(),
        );
        self.cur = [x + 1, y];
        Some(r)
    }
}
