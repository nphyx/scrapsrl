use super::iterators::AreaMapIter;
use super::{Tile, HEIGHT, WIDTH};
use crate::component::Position;
use crate::resource::GeographyTemplate;
use crate::util::Rect;

#[derive(Clone)]
pub struct AreaMap {
    pub tiles: Vec<Vec<Tile>>,
    pub width: i32,
    pub height: i32,
    /// mark true when mapgen is complete
    pub populated: bool,
    pub geography: GeographyTemplate,
}

impl Default for AreaMap {
    fn default() -> AreaMap {
        let tiles = vec![vec![Tile::default(); HEIGHT]; WIDTH];
        AreaMap {
            tiles,
            width: WIDTH as i32,
            height: HEIGHT as i32,
            populated: false,
            geography: GeographyTemplate::default(),
        }
    }
}

impl AreaMap {
    #[allow(unused)]
    fn with_dimensions(width: i32, height: i32) -> AreaMap {
        let tiles = vec![vec![Tile::default(); height as usize]; width as usize];
        AreaMap {
            tiles,
            width,
            height,
            populated: false,
            geography: GeographyTemplate::default(),
        }
    }

    pub fn wipe(&mut self) {
        let tile = Tile::default();
        for x in 0..self.width as usize {
            for y in 0..self.height as usize {
                self.tiles[x][y] = tile.clone();
            }
        }
    }

    pub fn get(&self, pos: Position) -> Option<Tile> {
        if 0 > pos.x || pos.x >= self.width || 0 > pos.y || pos.y >= self.height {
            return None;
        }
        Some(self.tiles[pos.x as usize][pos.y as usize].clone())
    }

    #[allow(unused)]
    pub fn get_mut(&mut self, pos: Position) -> Option<&mut Tile> {
        if 0 > pos.x || pos.x >= self.width || 0 > pos.y || pos.y >= self.height {
            return None;
        }
        Some(&mut self.tiles[pos.x as usize][pos.y as usize])
    }

    pub fn get_icon(&self, pos: Position) -> Option<char> {
        if 0 > pos.x || pos.x >= self.width || 0 > pos.y || pos.y >= self.height {
            return None;
        }
        Some(self.tiles[pos.x as usize][pos.y as usize].icon)
    }

    pub fn set(&mut self, pos: Position, tile: Tile) {
        if 0 > pos.x || pos.x >= self.width || 0 > pos.y || pos.y >= self.height {
            return;
        }
        self.tiles[pos.x as usize][pos.y as usize] = tile;
    }

    pub fn set_icon(&mut self, pos: Position, icon: char) {
        if 0 > pos.x || pos.x >= self.width || 0 > pos.y || pos.y >= self.height {
            return;
        }
        self.tiles[pos.x as usize][pos.y as usize].icon = icon;
    }

    pub fn iter(&self) -> AreaMapIter<'_> {
        AreaMapIter {
            map: self,
            cur: [0, 0],
        }
    }

    pub fn bounding_rect(&self) -> Rect {
        Rect {
            t_l: Position::new(0, 0),
            b_r: Position::new(self.width - 1, self.height - 1),
        }
    }

    /// makes a copy of a subset of the map within the requested rectangle
    pub fn submap(&self, rect: &Rect) -> Result<AreaMap, &'static str> {
        if self.bounding_rect().contains(rect) {
            let mut submap = AreaMap::with_dimensions(rect.width() + 1, rect.height() + 1);
            let t_l = rect.t_l.clone();
            for pos in rect.iter() {
                submap.set(pos - t_l, self.get(pos).unwrap().clone())
            }
            return Ok(submap);
        }
        return Err("rectangle out of map bounds");
    }

    /// paste a submap into a map, starting at <t_l> top-left corner position
    pub fn paste_at(&mut self, t_l: Position, submap: AreaMap) -> Result<bool, &'static str> {
        let occupied_rect = &Rect::new(
            t_l,
            Position::new(t_l.x + submap.width - 1, t_l.y + submap.height - 1),
        );
        if self.bounding_rect().contains(occupied_rect) {
            for (pos, tile) in submap.iter() {
                self.set(pos + t_l, tile.clone());
            }
            return Ok(true);
        }
        return Err("pasted submap not contained in target map");
    }

    /// finds the largest rectangle that is unoccupied within the bounds provided
    /// by <room>
    /// uses a stack-based solution for determining the largest rectangle in
    /// each column, then picks the overall largest
    pub fn fit_rect(&self, room: Rect) -> Rect {
        // this is our height histogram, we populate it from the map
        let zero: i32 = 0;
        let mut cells = vec![vec![zero; self.width as usize]; self.height as usize];
        let mut height: i32 = 0;
        let mut max_area: i32 = 0;
        // build the heightmap
        for col in room.iter_columns() {
            for pos in col.iter() {
                // don't bother populating areas outside bounds
                // it would be cheaper memory-wise to only build the part
                // we're examining, but it makes everything else more complicated
                // TODO reexamine this and maybe make it more complicated
                if room.includes(&pos) {
                    let maybe_tile = self.get(*pos);
                    if maybe_tile.is_none() || maybe_tile.unwrap().constructed {
                        height = 0
                    } else {
                        height += 1
                    }
                    // we're iterating column-wise so we need to flip the axes for a
                    // row-wise grid
                    cells[pos.y as usize][pos.x as usize] = height;
                }
            }
            height = 0;
        }
        // dbg!(debug_fit_rect(&cells));

        // solve largest rectangle in histogram for each column
        let mut stack: Vec<(i32, i32)> = Vec::new();
        // bottom-right corner
        let mut b_r: Position = Position::new(0, 0);
        let mut t_l: Position = Position::new(0, 0);
        let mut check = |x: i32, y: i32, (stack_x, stack_height): (i32, i32)| {
            let cur_width = x - stack_x;
            let temp_area = stack_height * cur_width;
            if temp_area > max_area {
                // check fires at X+1, so decrement it
                max_area = temp_area;
                t_l = Position::new(x - cur_width, y - (stack_height - 1));
                b_r = Position::new(x - 1, y);
            }
        };
        for (row_i, row) in room.iter_rows().enumerate() {
            for pos in row.iter() {
                let x = pos.x;
                let y = pos.y;
                let height = *(cells.get(y as usize).unwrap().get(x as usize).unwrap());
                let last: Option<(i32, i32)>;
                {
                    last = stack.iter().cloned().last();
                }
                if stack.len() == 0 {
                    stack.push((x, height));
                } else if let Some(entry) = last {
                    if height > entry.1 {
                        {
                            stack.push((x, height));
                        }
                    } else if height < entry.1 {
                        let mut consumed: usize = 0;
                        let mut temp_x: i32 = 0;
                        for entry in stack.iter().cloned().rev() {
                            if height > entry.1 {
                                break;
                            } else {
                                check(x, y, entry);
                                temp_x = entry.0;
                                consumed += 1;
                            }
                        }
                        if consumed > 0 {
                            while consumed > 0 {
                                stack.pop();
                                consumed -= 1;
                            }
                            stack.push((temp_x, height));
                        }
                    }
                } // end if let Some(entry) = last
            } // end pos in row.iter()
            for entry in stack.drain(0..).rev() {
                check(room.b_r.x + 1, room.t_l.y + row_i as i32, entry);
            }
        }
        /*
        for (y, row) in cells.iter().enumerate() {
            for (x, height) in row.iter().cloned().enumerate() {
        }
        */
        Rect { t_l, b_r }
    }
}

#[allow(unused)]
fn debug_fit_rect(cells: &Vec<Vec<i32>>) {
    println!(
        "   {}",
        (0..cells.len())
            .enumerate()
            .map(|(i, _)| format!("{: >2}", i))
            .collect::<String>()
    );
    println!(
        "  +{}",
        (0..cells.len())
            .map(|_| "--".to_string())
            .collect::<String>()
    );
    for (x, row) in cells.iter().enumerate() {
        println!(
            "{: >2}|{}",
            x,
            row.iter()
                .map(|i: &i32| format!("{: >2}", *i))
                .collect::<String>()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::{Color, Description, Position};
    #[test]
    fn fit_rect() {
        let mut map = AreaMap::with_dimensions(8, 8);
        let occupied = Tile::new(
            '#',
            Color::default(),
            Color::default(),
            false,
            false,
            true,
            Description::default(),
        );
        /* 0 1 2 3 4 5 6 7
         0 . . # . . . . .
         1 . . # . # . . .
         2 . . # . . . . #
         3 . # . . . . . .
         4 . # . . # # # #
         5 # # # # . . . .
         6 . # . . . . . .
         7 . # . . . . . #
        */
        map.set(Position::new(0, 5), occupied.clone());
        map.set(Position::new(1, 3), occupied.clone());
        map.set(Position::new(1, 4), occupied.clone());
        map.set(Position::new(1, 5), occupied.clone());
        map.set(Position::new(1, 6), occupied.clone());
        map.set(Position::new(1, 7), occupied.clone());
        map.set(Position::new(2, 0), occupied.clone());
        map.set(Position::new(2, 1), occupied.clone());
        map.set(Position::new(2, 2), occupied.clone());
        map.set(Position::new(2, 5), occupied.clone());
        map.set(Position::new(3, 5), occupied.clone());
        map.set(Position::new(4, 1), occupied.clone());
        map.set(Position::new(4, 4), occupied.clone());
        map.set(Position::new(5, 4), occupied.clone());
        map.set(Position::new(6, 4), occupied.clone());
        map.set(Position::new(7, 2), occupied.clone());
        map.set(Position::new(7, 4), occupied.clone());
        map.set(Position::new(7, 7), occupied.clone());
        {
            /* 1 2 3 4 5
             0 . # . . .
             1 . # . # .
             2 . # . . .
             3 # . . . .
             4 # . # # #
            */
            let rect = Rect::new(Position::new(1, 0), Position::new(5, 4));
            let expect_t_l = dbg!(Position::new(3, 2));
            let expect_b_r = dbg!(Position::new(5, 3));
            let res = dbg!(map.fit_rect(rect));
            assert!(res.t_l == expect_t_l, "top left correct");
            assert!(res.b_r == expect_b_r, "bottom right correct");
        };
        {
            /* 0 1 2 3 4 5 6
             4 . # . . # # #
             5 # # # # . . .
             6 . # . . . . .
             7 . # . . . . .
            */
            let rect = Rect::new(Position::new(0, 4), Position::new(6, 7));
            let expect_t_l = dbg!(Position::new(2, 6));
            let expect_b_r = dbg!(Position::new(6, 7));
            let res = dbg!(map.fit_rect(rect));
            assert!(res.t_l == expect_t_l, "top left correct");
            assert!(res.b_r == expect_b_r, "bottom right correct");
        };
        {
            /* 3 4 5 6 7
             0 . . . . .
             1 . # . . .
             2 . . . . #
             3 . . . . .
             4 . # # # #
             5 # . . . .
             6 . . . . .
            */
            let rect = Rect::new(Position::new(3, 0), Position::new(7, 6));
            let expect_t_l = Position::new(5, 0);
            let expect_b_r = Position::new(6, 3);
            let res = dbg!(map.fit_rect(rect));
            assert!(res.t_l == expect_t_l, "top left correct");
            assert!(res.b_r == expect_b_r, "bottom right correct");
        };
    }

    #[test]
    fn area_map_submap() {
        let mut map = AreaMap::with_dimensions(8, 8);
        let occupied = Tile::new(
            '#',
            Color::default(),
            Color::default(),
            false,
            false,
            true,
            Description::default(),
        );
        /* 0 1 2 3 4 5 6 7
         0 . . # . . . . .
         1 . . # . # . . .
         2 . . # . . . . #
         3 . # . . . . . .
         4 . # . . # # # #
         5 # # # # . . . .
         6 . # . . . . . .
         7 . # . . . . . #
        */
        map.set(Position::new(0, 5), occupied.clone());
        map.set(Position::new(1, 3), occupied.clone());
        map.set(Position::new(1, 4), occupied.clone());
        map.set(Position::new(1, 5), occupied.clone());
        map.set(Position::new(1, 6), occupied.clone());
        map.set(Position::new(1, 7), occupied.clone());
        map.set(Position::new(2, 0), occupied.clone());
        map.set(Position::new(2, 1), occupied.clone());
        map.set(Position::new(2, 2), occupied.clone());
        map.set(Position::new(2, 5), occupied.clone());
        map.set(Position::new(3, 5), occupied.clone());
        map.set(Position::new(4, 1), occupied.clone());
        map.set(Position::new(4, 4), occupied.clone());
        map.set(Position::new(5, 4), occupied.clone());
        map.set(Position::new(6, 4), occupied.clone());
        map.set(Position::new(7, 2), occupied.clone());
        map.set(Position::new(7, 4), occupied.clone());
        map.set(Position::new(7, 7), occupied.clone());
        {
            let t_l = Position::new(0, 0);
            let b_r = Position::new(2, 2);
            if let Ok(submap) = map.submap(&Rect { t_l, b_r }) {
                assert_eq!(submap.get(Position::new(0, 0)).unwrap().icon, ' ');
                assert_eq!(submap.get(Position::new(1, 1)).unwrap().icon, ' ');
                assert_eq!(submap.get(Position::new(2, 0)).unwrap().icon, '#');
                assert_eq!(submap.get(Position::new(2, 1)).unwrap().icon, '#');
                assert_eq!(submap.get(Position::new(2, 2)).unwrap().icon, '#');
                assert_eq!(submap.get(Position::new(0, 2)).unwrap().icon, ' ');
            } else {
                assert!(false, "submap creation failed with error");
            }
        }
    }
    #[test]
    fn area_map_paste_at() {
        let mut map = AreaMap::with_dimensions(5, 5);
        let mut submap = AreaMap::with_dimensions(3, 3);
        let occupied = Tile::new(
            '#',
            Color::default(),
            Color::default(),
            false,
            false,
            true,
            Description::default(),
        );
        /* 0 1 2
         0 # . .
         1 # . #
         2 . # .
        */
        submap.set(Position::new(0, 0), occupied.clone());
        submap.set(Position::new(0, 1), occupied.clone());
        submap.set(Position::new(2, 1), occupied.clone());
        submap.set(Position::new(1, 2), occupied.clone());
        {
            let t_l = Position::new(0, 0);
            if let Ok(_) = map.paste_at(t_l, submap.clone()) {
                assert_eq!(map.get(Position::new(0, 0)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(0, 1)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(2, 1)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(1, 2)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(1, 1)).unwrap().icon, ' ');
                map.wipe();
            } else {
                assert!(false, "submap paste failed with error")
            }
        }
        {
            let t_l = Position::new(2, 2);
            if let Ok(_) = map.paste_at(t_l, submap.clone()) {
                assert_eq!(map.get(Position::new(2, 2)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(2, 3)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(4, 3)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(3, 4)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(3, 3)).unwrap().icon, ' ');
                map.wipe();
            } else {
                assert!(false, "submap paste failed with error")
            }
        }
    }
}
