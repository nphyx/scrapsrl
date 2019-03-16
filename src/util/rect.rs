use super::Coord;
use num::*;
use std::cmp::PartialOrd;
use std::ops::{AddAssign, SubAssign};
use wfc::Size;

#[derive(Copy, Clone)]
pub struct Rect<T> {
    /// top left corner
    pub t_l: Coord<T>,
    /// bottom right corner
    pub b_r: Coord<T>,
}

impl Rect<usize> {
    pub fn width(&self) -> usize {
        self.b_r.x - self.t_l.x + 1
    }

    pub fn height(&self) -> usize {
        self.b_r.y - self.t_l.y + 1
    }

    pub fn to_wave_size(&self) -> Size {
        // wave size is exclusive of bottom/right bounds
        Size::new(self.width() as u32, self.height() as u32)
    }
}

#[allow(unused)]
impl Rect<i32> {
    pub fn width(&self) -> usize {
        (self.b_r.x - self.t_l.x + 1) as usize
    }

    pub fn height(&self) -> usize {
        (self.b_r.y - self.t_l.y + 1) as usize
    }

    #[deprecated]
    pub fn to_wave_size(&self) -> Size {
        // wave size is exclusive of bottom/right bounds
        Size::new(self.width() as u32, self.height() as u32)
    }

    /// iterates row-wise through all positions in the rectangle
    pub fn iter(&self) -> RectIter<i32> {
        RectIter {
            rect: *self,
            next_coord: self.t_l,
        }
    }
}

#[allow(unused)]
impl Rect<u32> {
    pub fn width(&self) -> usize {
        (self.b_r.x - self.t_l.x + 1) as usize
    }

    pub fn height(&self) -> usize {
        (self.b_r.y - self.t_l.y + 1) as usize
    }

    pub fn to_wave_size(&self) -> Size {
        // wave size is exclusive of bottom/right bounds
        Size::new((self.width() + 1) as u32, (self.height() + 1) as u32)
    }
}

impl<T: Integer + PartialOrd + AddAssign + SubAssign + Into<usize> + Copy> Rect<T> {
    pub fn new(t_l: Coord<T>, b_r: Coord<T>) -> Rect<T> {
        Rect { t_l, b_r }
    }

    /// checks whether the given pos is within the bounds of the rectangle
    pub fn includes(&self, coord: Coord<T>) -> bool {
        coord.x >= self.t_l.x
            && coord.y >= self.t_l.y
            && coord.x <= self.b_r.x
            && coord.y <= self.b_r.y
    }

    pub fn contains(&self, rect: Rect<T>) -> bool {
        self.includes(rect.t_l) && self.includes(rect.b_r)
    }

    #[allow(unused)]
    /// expands the perimeter by <n> on each side
    pub fn expand_perimeter(&mut self, n: T) {
        self.t_l.x -= n;
        self.t_l.y -= n;
        self.b_r.x += n;
        self.b_r.y += n;
    }

    #[allow(unused)]
    /// expands the perimeter by <n> on each side
    pub fn shrink_perimeter(&mut self, n: T) {
        self.t_l.x += n;
        self.t_l.y += n;
        self.b_r.x -= n;
        self.b_r.y -= n;
    }

    #[allow(unused)]
    pub fn is_corner(&self, coord: Coord<T>) -> bool {
        coord == self.t_l
            || coord == self.b_r
            || (coord == Coord::new(self.t_l.x, self.b_r.y))
            || (coord == Coord::new(self.b_r.x, self.t_l.y))
    }
}

impl Rect<usize> {
    #[allow(unused)]
    /// iterates row-wise through all positions in the rectangle
    pub fn iter(&self) -> RectIter<usize> {
        RectIter {
            rect: *self,
            next_coord: self.t_l,
        }
    }

    /// iterates through all the perimeter positions in the rectangle,
    /// inclusive, clockwise from top-left corner
    pub fn iter_perimeter(&self) -> RectIterPerimeter<usize> {
        RectIterPerimeter {
            rect: *self,
            next_coord: self.t_l,
            final_item: false,
        }
    }

    #[allow(unused)]
    /// iterates through the rows in a rectangle, yielding each row as a vector
    /// of positions
    pub fn iter_rows(&self) -> RectRowsIter<usize> {
        RectRowsIter {
            rect: *self,
            next_row: self.t_l.y as usize,
        }
    }

    /// iterates through the columns in a rectangle, yielding each column as a vector
    /// of positions
    pub fn iter_columns(&self) -> RectColsIter<usize> {
        RectColsIter {
            rect: *self,
            next_col: self.t_l.x as usize,
        }
    }

    #[allow(unused)]
    /// iterates through a single row
    pub fn iter_row(&self, y: usize) -> RectRowIter<usize> {
        RectRowIter {
            rect: *self,
            next_x: self.t_l.x as usize,
            y,
        }
    }

    #[allow(unused)]
    /// iterates through a single row
    pub fn iter_column(&self, x: usize) -> RectColIter<usize> {
        RectColIter {
            rect: *self,
            next_y: self.t_l.y as usize,
            x,
        }
    }
}

#[derive(Debug)]
/// iterates inclusively through a rectangle's points
pub struct RectIter<T> {
    rect: Rect<T>,
    next_coord: Coord<T>,
}

impl Iterator for RectIter<usize> {
    type Item = Coord<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.next_coord;
        if res.y > self.rect.b_r.y {
            return None;
        }
        self.next_coord.x += 1;
        if self.next_coord.x > self.rect.b_r.x {
            self.next_coord.x = self.rect.t_l.x;
            self.next_coord.y += 1;
        }
        Some(res)
    }
}

impl Iterator for RectIter<i32> {
    type Item = Coord<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.next_coord;
        if res.y > self.rect.b_r.y {
            return None;
        }
        self.next_coord.x += 1;
        if self.next_coord.x > self.rect.b_r.x {
            self.next_coord.x = self.rect.t_l.x;
            self.next_coord.y += 1;
        }
        Some(res)
    }
}

#[derive(Debug)]
/// iterates clockwise inclusively through a rectangle's perimeter, starting at the top
/// left corner and stopping at one tile below the top left corner
pub struct RectIterPerimeter<T> {
    rect: Rect<T>,
    next_coord: Coord<T>,
    final_item: bool,
}

impl Iterator for RectIterPerimeter<usize> {
    type Item = Coord<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.final_item {
            return None;
        }
        let res = self.next_coord;
        if self.next_coord.y == self.rect.t_l.y && self.next_coord.x < self.rect.b_r.x {
            self.next_coord.x += 1;
        } else if self.next_coord.x == self.rect.b_r.x && self.next_coord.y < self.rect.b_r.y {
            self.next_coord.y += 1;
        } else if self.next_coord.x > self.rect.t_l.x && self.next_coord.y == self.rect.b_r.y {
            self.next_coord.x -= 1;
        } else if self.next_coord.y > self.rect.t_l.y {
            self.next_coord.y -= 1;
        }
        if self.next_coord == self.rect.t_l {
            self.final_item = true;
        }
        Some(res)
    }
}

impl IntoIterator for Rect<usize> {
    type Item = Coord<usize>;
    type IntoIter = RectIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            next_coord: Self::Item::new(self.t_l.x, self.t_l.y),
            rect: self,
        }
    }
}

impl IntoIterator for Rect<i32> {
    type Item = Coord<i32>;
    type IntoIter = RectIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            next_coord: Self::Item::new(self.t_l.x, self.t_l.y),
            rect: self,
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
/// iterates through horizontal slices of the rectangle as vecs of positions
pub struct RectRowsIter<T> {
    rect: Rect<T>,
    next_row: usize,
}

impl Iterator for RectRowsIter<usize> {
    type Item = Vec<Coord<usize>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_row > self.rect.b_r.y {
            return None;
        }
        let res = (self.rect.t_l.x..=self.rect.b_r.x)
            .map(|x| Coord {
                x,
                y: self.next_row,
            })
            .collect();
        self.next_row += 1;
        Some(res)
    }
}

#[allow(unused)]
pub struct RectRowIter<T> {
    rect: Rect<T>,
    y: usize,
    next_x: usize,
}

impl Iterator for RectRowIter<usize> {
    type Item = Coord<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_x <= self.rect.b_r.x {
            self.next_x += 1;
            return Some(Coord::new(self.next_x - 1, self.y));
        }
        None
    }
}

#[derive(Debug)]
/// iterates through vertical slices of the rectangle as vecs of positions
pub struct RectColsIter<T> {
    rect: Rect<T>,
    next_col: usize,
}

impl Iterator for RectColsIter<usize> {
    type Item = Vec<Coord<usize>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_col > self.rect.b_r.x {
            return None;
        }
        let res = (self.rect.t_l.y..=self.rect.b_r.y)
            .map(|y| Coord {
                x: self.next_col,
                y,
            })
            .collect();
        self.next_col += 1;
        Some(res)
    }
}

#[allow(unused)]
pub struct RectColIter<T> {
    rect: Rect<T>,
    x: T,
    next_y: T,
}

impl Iterator for RectColIter<usize> {
    type Item = Coord<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_y <= self.rect.b_r.y {
            self.next_y += 1;
            return Some(Coord::new(self.x, self.next_y - 1));
        }
        None
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Rect<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Rect({:?}, {:?})", self.t_l, self.b_r)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Rect<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Rect({}, {})", self.t_l, self.b_r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::Pos;

    #[test]
    fn rect_contains() {
        let rect = Rect::new(Pos::new(1, 1), Pos::new(4, 4));
        assert!(rect.contains(Rect::new(Pos::new(1, 1), Pos::new(2, 2))));
        assert!(rect.contains(Rect::new(Pos::new(1, 1), Pos::new(4, 4))));
        assert!(!rect.contains(Rect::new(Pos::new(1, 1), Pos::new(5, 5))));
        assert!(!rect.contains(Rect::new(Pos::new(0, 0), Pos::new(2, 2))));
        assert!(!rect.contains(Rect::new(Pos::new(4, 4), Pos::new(7, 7))));
    }

    #[test]
    fn rect_iter() {
        {
            let mut iter = Rect::new(Pos::new(0, 0), Pos::new(1, 1)).into_iter();
            assert_eq!(iter.next().unwrap(), Pos { x: 0, y: 0 });
            assert_eq!(iter.next().unwrap(), Pos { x: 1, y: 0 });
            assert_eq!(iter.next().unwrap(), Pos { x: 0, y: 1 });
            assert_eq!(iter.next().unwrap(), Pos { x: 1, y: 1 });
            assert_eq!(iter.next(), None);
        }
        {
            let rect = Rect::new(Pos::new(0, 0), Pos::new(4, 4));
            let mut count: u8 = 0;
            for _pos in rect.iter() {
                count += 1;
            }
            assert_eq!(count, 25);
        }
        {
            let rect = Rect::new(Pos::new(0, 0), Pos::new(2, 0));
            let mut count: u8 = 0;
            for _pos in rect.iter() {
                count += 1;
            }
            assert_eq!(count, 3);
        }
        {
            let rect = Rect::new(Pos::new(0, 0), Pos::new(0, 4));
            let mut count: u8 = 0;
            for _pos in rect.iter() {
                count += 1;
            }
            assert_eq!(count, 5);
        }
    }
    #[test]
    fn rect_iter_perimeter() {
        {
            let mut iter = Rect::new(Pos::new(0, 0), Pos::new(2, 2)).iter_perimeter();
            assert_eq!(iter.next().unwrap(), Pos { x: 0, y: 0 });
            assert_eq!(iter.next().unwrap(), Pos { x: 1, y: 0 });
            assert_eq!(iter.next().unwrap(), Pos { x: 2, y: 0 });
            assert_eq!(iter.next().unwrap(), Pos { x: 2, y: 1 });
            assert_eq!(iter.next().unwrap(), Pos { x: 2, y: 2 });
            assert_eq!(iter.next().unwrap(), Pos { x: 1, y: 2 });
            assert_eq!(iter.next().unwrap(), Pos { x: 0, y: 2 });
            assert_eq!(iter.next().unwrap(), Pos { x: 0, y: 1 });
            assert_eq!(iter.next(), None);
        }
        {
            let mut iter = Rect::new(Pos::new(1, 1), Pos::new(2, 2)).iter_perimeter();
            assert_eq!(iter.next().unwrap(), Pos { x: 1, y: 1 });
            assert_eq!(iter.next().unwrap(), Pos { x: 2, y: 1 });
            assert_eq!(iter.next().unwrap(), Pos { x: 2, y: 2 });
            assert_eq!(iter.next().unwrap(), Pos { x: 1, y: 2 });
            assert_eq!(iter.next(), None);
        }
    }

    #[test]
    fn rect_iter_column() {
        let mut iter = Rect::new(Pos::new(0, 0), Pos::new(2, 2)).iter_column(1);
        assert_eq!(iter.next().unwrap(), Pos { x: 1, y: 0 });
        assert_eq!(iter.next().unwrap(), Pos { x: 1, y: 1 });
        assert_eq!(iter.next().unwrap(), Pos { x: 1, y: 2 });
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn rect_iter_row() {
        let mut iter = Rect::new(Pos::new(0, 0), Pos::new(2, 2)).iter_row(1);
        assert_eq!(iter.next().unwrap(), Pos { x: 0, y: 1 });
        assert_eq!(iter.next().unwrap(), Pos { x: 1, y: 1 });
        assert_eq!(iter.next().unwrap(), Pos { x: 2, y: 1 });
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn rect_is_corner() {
        let tl = Pos::new(0, 0);
        let tr = Pos::new(0, 4);
        let bl = Pos::new(4, 0);
        let br = Pos::new(4, 4);
        let rect = Rect::new(tl, br);
        assert!(rect.is_corner(tl));
        assert!(rect.is_corner(tr));
        assert!(rect.is_corner(bl));
        assert!(rect.is_corner(br));
        assert!(!rect.is_corner(Pos::new(0, 1)));
        assert!(!rect.is_corner(Pos::new(0, 3)));
        assert!(!rect.is_corner(Pos::new(1, 0)));
        assert!(!rect.is_corner(Pos::new(3, 0)));
        assert!(!rect.is_corner(Pos::new(2, 2)));
    }
}
