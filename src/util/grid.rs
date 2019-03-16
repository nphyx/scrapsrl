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
/// A generic grid employing a Rect to define its bounds, backed by an ndarray.
/// There is no general ::new() constructor for a grid, but if T is Default + Clone
/// there are with_bounds and with_dimensions constructors.
impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.bounds.width()
    }
    pub fn height(&self) -> usize {
        self.bounds.height()
    }

    /// Bounds-checked getter for a grid cell. Returns None when out of bounds.
    pub fn maybe_get(&self, pos: Pos) -> Option<&T> {
        if self.bounds.includes(pos) {
            return Some(&self.contents[(pos - self.bounds.t_l).as_tuple()]);
        }
        None
    }

    /// Unchecked getter for a grid cell.
    /// # Panics
    /// Panics if the requested position is out of bounds.
    pub fn unchecked_get(&self, pos: Pos) -> &T {
        &self.contents[(pos - self.bounds.t_l).as_tuple()]
    }

    /// Bounds-checked mutable get for a grid cell.
    pub fn maybe_get_mut(&mut self, pos: Pos) -> Option<&mut T> {
        if self.bounds.includes(pos) {
            return Some(&mut self.contents[(pos - self.bounds.t_l).as_tuple()]);
        }
        None
    }

    /// Unchecked mutable getter for a grid cell.
    /// # Panics
    /// Panics if the requested position is out of bounds.
    pub fn unchecked_get_mut(&mut self, pos: Pos) -> &mut T {
        &mut self.contents[(pos - self.bounds.t_l).as_tuple()]
    }

    /// Unchecked setter for a grid cell.
    /// # Panics
    /// Panics if the requested position is out of bounds.
    pub fn unchecked_set(&mut self, pos: Pos, entry: T) {
        self.contents[(pos - self.bounds.t_l).as_tuple()] = entry;
    }

    /// Bounds-checked setter for a grid cell.
    /// # Errors
    /// Returns an error result when out of bounds.
    pub fn try_set(&mut self, pos: Pos, entry: T) -> Result<bool, &'static str> {
        if self.bounds.includes(pos) {
            self.contents[(pos - self.bounds.t_l).as_tuple()] = entry;
            return Ok(true);
        }
        Err("out of bounds")
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
        if (!self.bounds.contains(bounds)) {
            // this shouldn't happen but it isn't fatal, let's log it so we can look into it
            // further
            println!("warning: tried to fit_rect on a map but the rect outside the map bounds");
            return Rect::new(Pos::new(0, 0), Pos::new(0, 0));
        }
        // this is our height histogram, we populate it from the map
        let mut cells: Grid<usize> = Grid::with_bounds(bounds);
        let mut height: usize = 0;
        // build the heightmap
        for col in bounds.iter_columns() {
            for pos in col.iter().cloned() {
                if bounds.includes(pos) {
                    if occupied(self.unchecked_get(pos)) {
                        height = 0
                    } else {
                        height += 1
                    }
                    cells.unchecked_set(pos, height);
                }
            }
            height = 0;
        }
        // dbg!(&cells); FIXME create a debug CLI option and then do this when it's enabled

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
                let height = *cells.unchecked_get(*pos);
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
/// Implements functions that are only available for Grids with cells that implement Default +
/// Clone.
impl<T: Default + Clone> Grid<T> {
    /// Constructor creating a grid with the given bounding rectangle
    pub fn with_bounds(bounds: Rect<usize>) -> Grid<T> {
        let mut contents: Array2<T> = Array2::default((bounds.height(), bounds.width()));
        Grid { contents, bounds }
    }

    /// Constructor creating a grid with the given dimensions
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

    /// Resets all the cells in the grid to defaults.
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

    /// Pastes the contents of another grid into the grid.
    /// # Error
    /// Returns an error result if the pasted grid would not fit in the grid's bounds.
    pub fn paste_into(&mut self, t_l: Pos, mut subgrid: Grid<T>) -> Result<bool, &'static str> {
        if self.bounds.contains(subgrid.bounds) {
            for pos in subgrid.bounds.iter() {
                self.unchecked_set(pos + t_l, subgrid.unchecked_get(pos).clone());
            }
            return Ok(true);
        }
        Err("pasted submap not contained in target map")
    }

    /// A row-wise iterator over the contents of the grid.
    pub fn iter(&self) -> ndarray::iter::Iter<'_, T, ndarray::Dim<[usize; 2]>> {
        self.contents.iter()
    }

    /// An iterator that returns the contents of the grid by row.
    pub fn iter_rows(&self) -> ndarray::iter::Lanes<'_, T, ndarray::Dim<[usize; 1]>> {
        self.contents.genrows()
    }
}

/// For usize Grids, debug output that prints a nice ASCII grid with axis labels.
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

/// For bool Grids, debug output that prints a nice ASCII grid with axis labels. False prints as
/// '.', and true prints as '#'.
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
                            .map(|i| if *i { "#" } else { "." })
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

    #[test]
    /// test bool debug output
    fn grid_debug_bool() {
        let grid: Grid<bool> = Grid::with_dimensions(2, 2);
        grid.unchecked_set(Position::new(0, 0), true);
        grid.unchecked_set(Position::new(1, 1), true);
        assert_eq!(
            format!("{:?}", grid),
            "\n\n   | 0 1\n---+--\n 0 | # .\n 1 | . #\n"
        );
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
        grid.unchecked_set(Pos::new(0, 5), true);
        grid.unchecked_set(Pos::new(1, 3), true);
        grid.unchecked_set(Pos::new(1, 4), true);
        grid.unchecked_set(Pos::new(1, 5), true);
        grid.unchecked_set(Pos::new(1, 6), true);
        grid.unchecked_set(Pos::new(1, 7), true);
        grid.unchecked_set(Pos::new(2, 0), true);
        grid.unchecked_set(Pos::new(2, 1), true);
        grid.unchecked_set(Pos::new(2, 2), true);
        grid.unchecked_set(Pos::new(2, 5), true);
        grid.unchecked_set(Pos::new(3, 5), true);
        grid.unchecked_set(Pos::new(4, 1), true);
        grid.unchecked_set(Pos::new(4, 4), true);
        grid.unchecked_set(Pos::new(5, 4), true);
        grid.unchecked_set(Pos::new(6, 4), true);
        grid.unchecked_set(Pos::new(7, 2), true);
        grid.unchecked_set(Pos::new(7, 4), true);
        grid.unchecked_set(Pos::new(7, 7), true);

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
                assert_eq!(*subgrid.unchecked_get(Pos::new(0, 0)), false);
                assert_eq!(*subgrid.unchecked_get(Pos::new(1, 1)), false);
                assert_eq!(*subgrid.unchecked_get(Pos::new(2, 0)), true);
                assert_eq!(*subgrid.unchecked_get(Pos::new(2, 1)), true);
                assert_eq!(*subgrid.unchecked_get(Pos::new(2, 2)), true);
                assert_eq!(*subgrid.unchecked_get(Pos::new(0, 2)), false);
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
        subgrid.unchecked_set(Pos::new(0, 0), true);
        subgrid.unchecked_set(Pos::new(0, 1), true);
        subgrid.unchecked_set(Pos::new(2, 1), true);
        subgrid.unchecked_set(Pos::new(1, 2), true);
        {
            let t_l = Pos::new(0, 0);
            if let Ok(_) = grid.paste_into(t_l, subgrid.clone()) {
                assert_eq!(*grid.unchecked_get(Pos::new(0, 0)), true);
                assert_eq!(*grid.unchecked_get(Pos::new(0, 1)), true);
                assert_eq!(*grid.unchecked_get(Pos::new(2, 1)), true);
                assert_eq!(*grid.unchecked_get(Pos::new(1, 2)), true);
                assert_eq!(*grid.unchecked_get(Pos::new(1, 1)), false);
                grid.clear();
            } else {
                assert!(false, "subgrid paste failed with error")
            }
        }
        {
            let t_l = Pos::new(2, 2);
            if let Ok(_) = grid.paste_into(t_l, subgrid.clone()) {
                assert_eq!(*grid.unchecked_get(Pos::new(2, 2)), true);
                assert_eq!(*grid.unchecked_get(Pos::new(2, 3)), true);
                assert_eq!(*grid.unchecked_get(Pos::new(4, 3)), true);
                assert_eq!(*grid.unchecked_get(Pos::new(3, 4)), true);
                assert_eq!(*grid.unchecked_get(Pos::new(3, 3)), false);
                grid.clear();
            } else {
                assert!(false, "subgrid paste failed with error")
            }
        }
    }
}
