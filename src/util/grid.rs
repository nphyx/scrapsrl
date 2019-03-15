use super::rect::*;
use crate::component::Pos;
use ndarray::Array2;
#[allow(unused)]
#[derive(Clone)]
pub struct Grid<T> {
    contents: Array2<T>,
    pub bounds: Rect<usize>,
}

#[allow(unused)]
impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.bounds.width()
    }
    pub fn height(&self) -> usize {
        self.bounds.height()
    }

    pub fn get(&self, pos: Pos) -> &T {
        &self.contents[(pos - self.bounds.t_l).as_tuple()]
    }

    pub fn get_mut(&mut self, pos: Pos) -> &mut T {
        &mut self.contents[(pos - self.bounds.t_l).as_tuple()]
    }

    pub fn set(&mut self, pos: Pos, entry: T) {
        self.contents[(pos - self.bounds.t_l).as_tuple()] = entry;
    }

    #[deprecated]
    pub fn bounds(&self) -> Rect<usize> {
        self.bounds
    }

    /// finds the largest rectangle that is unoccupied within the bounds provided
    /// by <room>
    /// uses a stack-based solution for determining the largest rectangle in
    /// each column, then picks the overall largest
    pub fn fit_rect(&self, bounds: Rect<usize>, occupied: &Fn(&T) -> bool) -> Rect<usize> {
        // this is our height histogram, we populate it from the map
        let mut cells: Grid<usize> = Grid::with_bounds(bounds);
        let mut height: usize = 0;
        // build the heightmap
        for col in bounds.iter_columns() {
            for pos in col.iter().cloned() {
                if bounds.includes(pos) {
                    if occupied(self.get(pos)) {
                        height = 0
                    } else {
                        height += 1
                    }
                    cells.set(pos, height);
                }
            }
            height = 0;
        }
        dbg!(&cells);

        // solve largest rectangle in histogram for each column
        let mut stack: Vec<(usize, usize)> = Vec::new();
        // bottom-right corner
        let mut b_r: Pos = Pos::new(0, 0);
        let mut t_l: Pos = Pos::new(0, 0);
        let mut max_area: usize = 0;
        let mut check = |pos: Pos, (stack_x, stack_height): (usize, usize)| {
            let cur_width = pos.x - stack_x;
            let temp_area = stack_height * cur_width;
            if temp_area > max_area {
                // check fires at X+1, so decrement it
                max_area = temp_area;
                t_l = Pos::new(pos.x - cur_width, pos.y - (stack_height - 1));
                b_r = pos - Pos::new(1, 0);
            }
        };
        for (y, row) in bounds.iter_rows().enumerate() {
            for pos in row.iter() {
                let last: Option<(usize, usize)>;
                let height = *cells.get(*pos);
                {
                    last = stack.iter().cloned().last();
                }
                if stack.is_empty() {
                    stack.push((pos.x, height));
                } else if let Some(entry) = last {
                    if height > entry.1 {
                        {
                            stack.push((pos.x, height));
                        }
                    } else if height < entry.1 {
                        let mut consumed: usize = 0;
                        let mut temp_x: usize = 0;
                        for entry in stack.iter().cloned().rev() {
                            if height > entry.1 {
                                break;
                            } else {
                                check(*pos, entry);
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
                check(Pos::new(bounds.b_r.x + 1, bounds.t_l.y + y), entry);
            }
        }
        Rect { t_l, b_r }
    }
}

#[allow(unused)]
impl<T: Default + Clone> Grid<T> {
    /// creates a grid with the given bounding rectangle
    pub fn with_bounds(bounds: Rect<usize>) -> Grid<T> {
        let mut contents: Array2<T> = Array2::default((bounds.height(), bounds.width()));
        Grid { contents, bounds }
    }

    /// creates a grid with the given dimensions
    pub fn with_dimensions(width: usize, height: usize) -> Grid<T> {
        let bounds = Rect {
            t_l: Pos { x: 0, y: 0 },
            b_r: Pos {
                x: width - 1,
                y: height - 1,
            },
        };
        Grid::with_bounds(bounds)
    }

    pub fn clear(&mut self) {
        self.contents.fill(Default::default())
    }

    /// extracts a copy of a slice of the grid's contents
    pub fn subgrid(&self, rect: Rect<usize>) -> Result<Grid<T>, &'static str> {
        if self.bounds.contains(rect) {
            let width = rect.width();
            let height = rect.height();
            let mut subgrid: Grid<T> = Grid {
                bounds: rect,
                contents: self
                    .contents
                    .slice(s![rect.t_l.x..=rect.b_r.x, rect.t_l.y..=rect.b_r.y])
                    .into_owned(),
            };
            return Ok(subgrid);
        }
        Err("rectangle out of grid bounds")
    }

    pub fn paste_into(&mut self, t_l: Pos, mut subgrid: Grid<T>) -> Result<bool, &'static str> {
        if self.bounds.contains(subgrid.bounds) {
            for pos in subgrid.bounds.iter() {
                self.set(pos + t_l, subgrid.get(pos).clone());
            }
            return Ok(true);
        }
        Err("pasted submap not contained in target map")
    }

    pub fn iter(&self) -> ndarray::iter::Iter<'_, T, ndarray::Dim<[usize; 2]>> {
        self.contents.iter()
    }

    pub fn iter_rows(&self) -> ndarray::iter::Lanes<'_, T, ndarray::Dim<[usize; 1]>> {
        self.contents.genrows()
    }
}

impl std::fmt::Debug for Grid<usize> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let width = self.width();
        let mut x = 0;
        write!(
            f,
            "\n\n{}\n{}\n{}",
            format!(
                "   |{}",
                (0..width)
                    .enumerate()
                    .map(|(i, _)| format!("{: >2}", i + self.bounds.t_l.x))
                    .collect::<String>()
            ),
            format!(
                "---+{}",
                (0..width).map(|_| "--".to_string()).collect::<String>()
            ),
            self.iter_rows()
                .into_iter()
                .map(|row| {
                    x += 1;
                    format!(
                        "{: >2} |{}\n",
                        x - 1 + self.bounds.t_l.y,
                        row.iter()
                            .map(|i| format!("{: >2}", *i))
                            .collect::<String>(),
                    )
                })
                .collect::<String>()
        )
    }
}

impl std::fmt::Debug for Grid<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let width = self.width();
        let mut x = 0;
        write!(
            f,
            "\n\n{}\n{}\n{}",
            format!(
                "   |{}",
                (0..width)
                    .enumerate()
                    .map(|(i, _)| format!("{: >2}", i + self.bounds.t_l.x))
                    .collect::<String>()
            ),
            format!(
                "---+{}",
                (0..width).map(|_| "--".to_string()).collect::<String>()
            ),
            self.iter_rows()
                .into_iter()
                .map(|row| {
                    x += 1;
                    format!(
                        "{: >2} |{}\n",
                        x - 1 + self.bounds.t_l.y,
                        row.iter()
                            .map(|i| format!("{: >2}", if *i { 1 } else { 0 }))
                            .collect::<String>(),
                    )
                })
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Axis;

    #[test]
    fn grid_with_bounds() {
        let bounds = Rect::new(Pos::new(0, 4), Pos::new(6, 7));
        let grid: Grid<usize> = Grid::with_bounds(bounds);
        assert_eq!(grid.contents.len(), 28);
        assert_eq!(grid.contents.len_of(Axis(0)), 4);
        assert_eq!(grid.contents.len_of(Axis(1)), 7);
        assert_eq!(grid.bounds.t_l, Pos::new(0, 4));
        assert_eq!(grid.bounds.b_r, Pos::new(6, 7));
        assert_eq!(grid.width(), 7);
        assert_eq!(grid.height(), 4);
    }

    #[test]
    fn grid_with_dimensions() {
        let grid: Grid<usize> = Grid::with_dimensions(3, 7);
        assert_eq!(grid.contents.len(), 21);
        assert_eq!(grid.contents.len_of(Axis(0)), 7);
        assert_eq!(grid.contents.len_of(Axis(1)), 3);
        assert_eq!(grid.bounds.t_l, Pos::new(0, 0));
        assert_eq!(grid.bounds.b_r, Pos::new(2, 6));
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 7);
    }

    #[test]
    /// test usize debug output
    fn grid_debug_usize() {
        let grid: Grid<usize> = Grid::with_dimensions(1, 1);
        assert_eq!(format!("{:?}", grid), "\n\n   | 0\n---+--\n 0 | 0\n");
    }

    fn build_test_grid() -> Grid<bool> {
        let mut grid: Grid<bool> = Grid::with_dimensions(8, 8);
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
        grid.set(Pos::new(0, 5), true);
        grid.set(Pos::new(1, 3), true);
        grid.set(Pos::new(1, 4), true);
        grid.set(Pos::new(1, 5), true);
        grid.set(Pos::new(1, 6), true);
        grid.set(Pos::new(1, 7), true);
        grid.set(Pos::new(2, 0), true);
        grid.set(Pos::new(2, 1), true);
        grid.set(Pos::new(2, 2), true);
        grid.set(Pos::new(2, 5), true);
        grid.set(Pos::new(3, 5), true);
        grid.set(Pos::new(4, 1), true);
        grid.set(Pos::new(4, 4), true);
        grid.set(Pos::new(5, 4), true);
        grid.set(Pos::new(6, 4), true);
        grid.set(Pos::new(7, 2), true);
        grid.set(Pos::new(7, 4), true);
        grid.set(Pos::new(7, 7), true);

        grid
    }

    #[test]
    fn fit_rect() {
        let grid = build_test_grid();
        let occupied = &|item: &bool| -> bool { *item };
        {
            /* 1 2 3 4 5
             0 . # . . .
             1 . # . # .
             2 . # . . .
             3 # . . . .
             4 # . # # #
            */
            dbg!(&grid);
            let rect = Rect::new(Pos::new(1, 0), Pos::new(5, 4));
            let expect_t_l = Pos::new(3, 2);
            let expect_b_r = Pos::new(5, 3);
            let res = grid.fit_rect(rect, &occupied);
            println!("expecting ({}, {})", expect_t_l, expect_b_r);
            println!("got   {}\n", res);
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
            dbg!(&grid);
            let rect = Rect::new(Pos::new(0, 4), Pos::new(6, 7));
            let expect_t_l = Pos::new(2, 6);
            let expect_b_r = Pos::new(6, 7);
            let res = grid.fit_rect(rect, &occupied);
            println!("expecting ({}, {})", expect_t_l, expect_b_r);
            println!("got   {}\n", res);
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
            let rect = Rect::new(Pos::new(3, 0), Pos::new(7, 6));
            let expect_t_l = Pos::new(5, 0);
            let expect_b_r = Pos::new(6, 3);
            let res = dbg!(grid.fit_rect(rect, &occupied));
            assert!(res.t_l == expect_t_l, "top left correct");
            assert!(res.b_r == expect_b_r, "bottom right correct");
        };
    }

    #[test]
    fn area_grid_subgrid() {
        let grid = build_test_grid();
        {
            let t_l = Pos::new(0, 0);
            let b_r = Pos::new(2, 2);
            if let Ok(subgrid) = grid.subgrid(Rect { t_l, b_r }) {
                assert_eq!(*subgrid.get(Pos::new(0, 0)), false);
                assert_eq!(*subgrid.get(Pos::new(1, 1)), false);
                assert_eq!(*subgrid.get(Pos::new(2, 0)), true);
                assert_eq!(*subgrid.get(Pos::new(2, 1)), true);
                assert_eq!(*subgrid.get(Pos::new(2, 2)), true);
                assert_eq!(*subgrid.get(Pos::new(0, 2)), false);
            } else {
                assert!(false, "subgrid creation failed with error");
            }
        }
    }
    #[test]
    fn area_grid_paste_into() {
        let mut grid: Grid<bool> = Grid::with_dimensions(5, 5);
        let mut subgrid: Grid<bool> = Grid::with_dimensions(3, 3);
        /* 0 1 2
         0 # . .
         1 # . #
         2 . # .
        */
        subgrid.set(Pos::new(0, 0), true);
        subgrid.set(Pos::new(0, 1), true);
        subgrid.set(Pos::new(2, 1), true);
        subgrid.set(Pos::new(1, 2), true);
        {
            let t_l = Pos::new(0, 0);
            if let Ok(_) = grid.paste_into(t_l, subgrid.clone()) {
                assert_eq!(*grid.get(Pos::new(0, 0)), true);
                assert_eq!(*grid.get(Pos::new(0, 1)), true);
                assert_eq!(*grid.get(Pos::new(2, 1)), true);
                assert_eq!(*grid.get(Pos::new(1, 2)), true);
                assert_eq!(*grid.get(Pos::new(1, 1)), false);
                grid.clear();
            } else {
                assert!(false, "subgrid paste failed with error")
            }
        }
        {
            let t_l = Pos::new(2, 2);
            if let Ok(_) = grid.paste_into(t_l, subgrid.clone()) {
                assert_eq!(*grid.get(Pos::new(2, 2)), true);
                assert_eq!(*grid.get(Pos::new(2, 3)), true);
                assert_eq!(*grid.get(Pos::new(4, 3)), true);
                assert_eq!(*grid.get(Pos::new(3, 4)), true);
                assert_eq!(*grid.get(Pos::new(3, 3)), false);
                grid.clear();
            } else {
                assert!(false, "subgrid paste failed with error")
            }
        }
    }
}
