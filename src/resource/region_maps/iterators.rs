use super::{RegionMap, Tile};
use crate::component::Pos;

pub struct RegionMapIter<'a> {
    pub map: &'a RegionMap,
    pub cur: [usize; 2],
}

impl<'a> Iterator for RegionMapIter<'a> {
    type Item = (Pos, &'a Tile);

    fn next(&mut self) -> Option<(Pos, &'a Tile)> {
        let [mut x, mut y] = self.cur;
        let w = self.map.width() as usize;
        let h = self.map.height() as usize;
        if x >= w {
            x = 0;
            y += 1;
        }
        if y >= h {
            return None;
        }
        let pos = Pos::new(x, y);
        let res = (pos, self.map.get(pos).unwrap());
        self.cur = [x + 1, y];
        Some(res)
    }
}
