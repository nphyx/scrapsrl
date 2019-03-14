use super::{rect::RectIter, Rect};
use crate::component::Position;

#[allow(unused)]
#[derive(Clone)]
pub struct Grid<T> {
    contents: Vec<Vec<T>>,
    bounds: Rect,
}

#[allow(unused)]
impl<T> Grid<T> {
    pub fn width(&self) -> i32 {
        self.bounds.width()
    }
    pub fn height(&self) -> i32 {
        self.bounds.height()
    }

    pub fn get(&self, pos: Position) -> Option<&T> {
        self.contents
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize))
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut T> {
        self.contents
            .get_mut(pos.y as usize)
            .and_then(|row| row.get_mut(pos.x as usize))
    }

    pub fn set(&mut self, pos: Position, entry: T) {
        self.contents[pos.y as usize][pos.x as usize] = entry;
    }

    pub fn bounds(&self) -> Rect {
        self.bounds.clone()
    }

    pub fn clear(&mut self) {
        self.contents.clear()
    }
}

#[allow(unused)]
impl<T: Default + Clone> Grid<T> {
    pub fn with_dimensions(width: usize, height: usize) -> Grid<T> {
        let bounds = Rect {
            t_l: Position { x: 0, y: 0 },
            b_r: Position {
                x: width as i32 - 1,
                y: height as i32 - 1,
            },
        };
        Grid::with_bounds(bounds)
    }

    /// creates a grid with the given bounding rectangle
    pub fn with_bounds(bounds: Rect) -> Grid<T> {
        let mut contents: Vec<Vec<T>> =
            vec![vec![Default::default(); bounds.width() as usize]; bounds.height() as usize];
        Grid { contents, bounds }
    }

    pub fn subgrid(&self, rect: Rect) -> Result<Grid<T>, &'static str> {
        if self.bounds.contains(rect) {
            let mut subgrid: Grid<T> =
                Grid::with_dimensions(rect.width() as usize, rect.height() as usize);
            for pos in rect.iter() {
                subgrid.set(pos - rect.t_l, self.get(pos).unwrap().clone());
            }
            return Ok(subgrid);
        }
        Err("rectangle out of grid bounds")
    }

    pub fn paste_into(
        &mut self,
        t_l: Position,
        mut subgrid: Grid<T>,
    ) -> Result<bool, &'static str> {
        if self.bounds.contains(subgrid.bounds()) {
            for (pos, entry) in subgrid.iter() {
                self.set(pos + t_l, entry);
            }
            return Ok(true);
        }
        return Err("pasted submap not contained in target map");
    }

    pub fn iter(&self) -> GridIter<'_, T> {
        GridIter {
            iterator: self.bounds.clone().iter(),
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
    iterator: RectIter,
    grid: &'a Grid<T>,
}

impl<'a, T: Clone> Iterator for GridIter<'a, T> {
    type Item = (Position, T);

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
    type Item = (Position, T);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(drain) = self.col_iter {
            if let Some((x, entry)) = drain.next() {
                return Some((Position::new(x, self.last_y), entry));
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
