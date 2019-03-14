use super::iterators::AreaMapIter;
use super::{Tile, HEIGHT, WIDTH};
use crate::component::Position;
use crate::resource::GeographyTemplate;
use crate::util::{Grid, Rect};

#[derive(Clone)]
pub struct AreaMap {
    grid: Grid<Tile>,
    /// mark true when mapgen is complete
    pub populated: bool,
    pub geography: GeographyTemplate,
}

impl Default for AreaMap {
    fn default() -> AreaMap {
        let grid = Grid::with_dimensions(WIDTH, HEIGHT);
        AreaMap {
            grid,
            populated: false,
            geography: GeographyTemplate::default(),
        }
    }
}

impl AreaMap {
    #[allow(unused)]
    fn with_dimensions(width: usize, height: usize) -> AreaMap {
        let grid = Grid::with_dimensions(width as usize, height as usize);
        AreaMap {
            grid,
            populated: false,
            geography: GeographyTemplate::default(),
        }
    }

    pub fn height(&self) -> i32 {
        self.grid.height()
    }

    pub fn width(&self) -> i32 {
        self.grid.width()
    }

    #[allow(unused)]
    pub fn wipe(&mut self) {
        self.grid.clear();
        self.populated = false;
    }

    pub fn bounds(&self) -> Rect {
        self.grid.bounds()
    }

    pub fn get(&self, pos: Position) -> Option<&Tile> {
        self.grid.get(pos)
    }

    #[allow(unused)]
    pub fn get_mut(&mut self, pos: Position) -> Option<&mut Tile> {
        self.grid.get_mut(pos)
    }

    pub fn get_icon(&self, pos: Position) -> Option<char> {
        self.grid.get(pos).map(|t| t.icon)
    }

    pub fn set(&mut self, pos: Position, tile: Tile) {
        self.grid.set(pos, tile)
    }

    pub fn set_icon(&mut self, pos: Position, icon: char) {
        self.grid.get_mut(pos).unwrap().icon = icon;
    }

    pub fn iter(&self) -> AreaMapIter<'_> {
        AreaMapIter {
            map: self,
            cur: [0, 0],
        }
    }

    pub fn bounding_rect(&self) -> Rect {
        self.grid.bounds()
    }

    pub fn subgrid(&self, rect: Rect) -> Result<Grid<Tile>, &'static str> {
        self.grid.subgrid(rect)
    }

    /// paste a subgrid into a map, starting at <t_l> top-left corner position
    /// consumes the subgrid in the process
    pub fn paste_into(&mut self, t_l: Position, subgrid: Grid<Tile>) -> Result<bool, &'static str> {
        self.grid.paste_into(t_l, subgrid)
    }

    /// finds the largest rectangle that is unoccupied within the bounds provided
    /// by <room>
    /// uses a stack-based solution for determining the largest rectangle in
    /// each column, then picks the overall largest
    pub fn fit_rect(&self, room: Rect) -> Rect {
        // this is our height histogram, we populate it from the map
        let mut cells: Grid<i32> =
            Grid::with_dimensions(self.width() as usize, self.height() as usize);
        let mut height: i32 = 0;
        let mut max_area: i32 = 0;
        // build the heightmap
        for col in room.iter_columns() {
            for pos in col.iter() {
                // don't bother populating areas outside bounds
                // it would be cheaper memory-wise to only build the part
                // we're examining, but it makes everything else more complicated
                // TODO reexamine this and maybe make it more complicated
                if room.includes(*pos) {
                    let maybe_tile = self.get(*pos);
                    if maybe_tile.is_none() || maybe_tile.unwrap().constructed {
                        height = 0
                    } else {
                        height += 1
                    }
                    // we're iterating column-wise so we need to flip the axes for a
                    // row-wise grid
                    cells.set(*pos, height);
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
                let height: i32 = *cells.get(*pos).unwrap_or(&0);
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
        Rect { t_l, b_r }
    }
}

impl From<&AreaMap> for Rect {
    fn from(map: &AreaMap) -> Rect {
        Rect {
            t_l: Position::new(0, 0),
            b_r: Position::new(map.width(), map.height()),
        }
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
    fn area_map_subgrid() {
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
            if let Ok(subgrid) = map.subgrid(Rect { t_l, b_r }) {
                assert_eq!(subgrid.get(Position::new(0, 0)).unwrap().icon, ' ');
                assert_eq!(subgrid.get(Position::new(1, 1)).unwrap().icon, ' ');
                assert_eq!(subgrid.get(Position::new(2, 0)).unwrap().icon, '#');
                assert_eq!(subgrid.get(Position::new(2, 1)).unwrap().icon, '#');
                assert_eq!(subgrid.get(Position::new(2, 2)).unwrap().icon, '#');
                assert_eq!(subgrid.get(Position::new(0, 2)).unwrap().icon, ' ');
            } else {
                assert!(false, "subgrid creation failed with error");
            }
        }
    }
    #[test]
    fn area_map_paste_into() {
        let mut map = AreaMap::with_dimensions(5, 5);
        let mut subgrid: Grid<Tile> = Grid::with_dimensions(3, 3);
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
        subgrid.set(Position::new(0, 0), occupied.clone());
        subgrid.set(Position::new(0, 1), occupied.clone());
        subgrid.set(Position::new(2, 1), occupied.clone());
        subgrid.set(Position::new(1, 2), occupied.clone());
        {
            let t_l = Position::new(0, 0);
            if let Ok(_) = map.paste_into(t_l, subgrid.clone()) {
                assert_eq!(map.get(Position::new(0, 0)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(0, 1)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(2, 1)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(1, 2)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(1, 1)).unwrap().icon, ' ');
                map.wipe();
            } else {
                assert!(false, "subgrid paste failed with error")
            }
        }
        {
            let t_l = Position::new(2, 2);
            if let Ok(_) = map.paste_into(t_l, subgrid.clone()) {
                assert_eq!(map.get(Position::new(2, 2)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(2, 3)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(4, 3)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(3, 4)).unwrap().icon, '#');
                assert_eq!(map.get(Position::new(3, 3)).unwrap().icon, ' ');
                map.wipe();
            } else {
                assert!(false, "subgrid paste failed with error")
            }
        }
    }
}
