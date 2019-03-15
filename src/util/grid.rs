use super::{rect::RectIter, Rect};
use crate::component::Pos;

#[allow(unused)]
#[derive(Clone)]
pub struct Grid<T> {
    contents: Vec<Vec<T>>,
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

    pub fn get(&self, pos: Pos) -> Option<&T> {
        self.contents.get(pos.y).and_then(|row| row.get(pos.x))
    }

    pub fn get_mut(&mut self, pos: Pos) -> Option<&mut T> {
        self.contents
            .get_mut(pos.y)
            .and_then(|row| row.get_mut(pos.x))
    }

    pub fn set(&mut self, pos: Pos, entry: T) {
        self.contents[pos.y][pos.x] = entry;
    }

    #[deprecated]
    pub fn bounds(&self) -> Rect<usize> {
        self.bounds
    }

    pub fn clear(&mut self) {
        self.contents.clear()
    }
}

#[allow(unused)]
impl<T: Default + Clone> Grid<T> {
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

    /// creates a grid with the given bounding rectangle
    pub fn with_bounds(bounds: Rect<usize>) -> Grid<T> {
        let mut contents: Vec<Vec<T>> =
            vec![vec![Default::default(); bounds.width()]; bounds.height()];
        Grid { contents, bounds }
    }

    pub fn subgrid(&self, rect: Rect<usize>) -> Result<Grid<T>, &'static str> {
        if self.bounds.contains(rect) {
            let mut subgrid: Grid<T> = Grid::with_dimensions(rect.width(), rect.height());
            for pos in rect.iter() {
                subgrid.set(pos - rect.t_l, self.get(pos).unwrap().clone());
            }
            return Ok(subgrid);
        }
        Err("rectangle out of grid bounds")
    }

    pub fn paste_into(&mut self, t_l: Pos, mut subgrid: Grid<T>) -> Result<bool, &'static str> {
        if self.bounds.contains(subgrid.bounds) {
            for (pos, entry) in subgrid.iter() {
                self.set(pos + t_l, entry);
            }
            return Ok(true);
        }
        Err("pasted submap not contained in target map")
    }

    pub fn iter(&self) -> GridIter<'_, T> {
        GridIter {
            iterator: self.bounds.iter(),
            grid: self,
        }
    }

    /*
    pub fn drain(self) -> GridDrain<T> {
        GridDrain::from(self)
    }
    */
}

pub struct GridIter<'a, T> {
    iterator: RectIter<usize>,
    grid: &'a Grid<T>,
}

impl<'a, T: Clone> Iterator for GridIter<'a, T> {
    type Item = (Pos, T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pos) = self.iterator.next() {
            if let Some(item) = self.grid.get(pos) {
                return Some((pos, item.clone()));
            }
        }
        None
    }
}

/*
use super::rect::RectRowsIter;
use std::ops::Range;
use std::vec::Iter;
pub struct GridDrain<T> {
    grid: Grid<T>,
    row_iter: Range<i32>,
    col_iter: Iter<i32, T>,
    last_y: i32,
}

impl<'a, T> Iterator for GridDrain<'a, T> {
    type Item = (Pos, T);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(drain) = self.col_iter {
            if let Some((x, entry)) = drain.next() {
                return Some((Pos::new(x, self.last_y), entry));
            }
        }
        if let Some(y) = self.row_iter.next() {
            self.last_y = y as i32;
            if let Some(col) = self.grid.contents.get(y as usize) {
                self.col_iter = col.drain(0..).into_iter().enumerate();
        }
        if let Some(y) = self.row_iter.next() {
            if let Some(entry) = self.grid.get(pos) {
                return Some((pos, entry));
            }
            return None;
        }
        None
    }
}

impl<'a, T> From<Grid<T>> for GridDrain<'a, T> {
    fn from(grid: Grid<T>) -> GridDrain<'a, T> {
        let row_iter = 0..grid.bounds().height().iter();
        GridDrain { grid, row_iter }
    }
}
*/
