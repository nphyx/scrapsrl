use crate::component::Position;
use wfc::Size;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    /// top left corner
    pub t_l: Position,
    /// bottom right corner
    pub b_r: Position,
}

impl Rect {
    pub fn new(t_l: Position, b_r: Position) -> Rect {
        Rect { t_l, b_r }
    }

    pub fn width(&self) -> i32 {
        self.b_r.x - self.t_l.x
    }

    pub fn height(&self) -> i32 {
        self.b_r.y - self.t_l.y
    }

    /// checks whether the given pos is within the bounds of the rectangle
    pub fn includes(&self, pos: &Position) -> bool {
        pos.x >= self.t_l.x && pos.y >= self.t_l.y && pos.x <= self.b_r.x && pos.y <= self.b_r.y
    }

    #[allow(unused)]
    /// expands the perimeter by <n> on each side
    pub fn expand_perimeter(&mut self, n: u8) {
        self.t_l.x -= 1;
        self.t_l.y -= 1;
        self.b_r.x += 1;
        self.b_r.y += 1;
    }

    #[allow(unused)]
    /// expands the perimeter by <n> on each side
    pub fn shrink_perimeter(&mut self, n: u8) {
        self.t_l.x += 1;
        self.t_l.y += 1;
        self.b_r.x -= 1;
        self.b_r.y -= 1;
    }

    #[allow(unused)]
    pub fn is_corner(&self, pos: Position) -> bool {
        return pos == self.t_l
            || pos == self.b_r
            || (pos == Position::new(self.t_l.x, self.b_r.y))
            || (pos == Position::new(self.b_r.x, self.t_l.y));
    }

    pub fn to_wave_size(&self) -> Size {
        // wave size is exclusive of bottom/right bounds
        Size::new(self.width() as u32 + 1, self.height() as u32 + 1)
    }

    #[allow(unused)]
    /// iterates row-wise through all positions in the rectangle
    pub fn iter(&self) -> RectIter {
        RectIter {
            rect: self.clone(),
            next_pos: self.t_l.clone(),
        }
    }

    /// iterates through all the perimeter positions in the rectangle,
    /// inclusive, clockwise from top-left corner
    pub fn iter_perimeter(&self) -> RectIterPerimeter {
        RectIterPerimeter {
            rect: self.clone(),
            next_pos: self.t_l.clone(),
            final_item: false,
        }
    }

    #[allow(unused)]
    /// iterates through the rows in a rectangle, yielding each row as a vector
    /// of positions
    pub fn iter_rows(&self) -> RectRowsIter {
        RectRowsIter {
            rect: self.clone(),
            next_row: self.t_l.y,
        }
    }

    /// iterates through the columns in a rectangle, yielding each column as a vector
    /// of positions
    pub fn iter_columns(&self) -> RectColsIter {
        RectColsIter {
            rect: self.clone(),
            next_col: self.t_l.x,
        }
    }

    #[allow(unused)]
    /// iterates through a single row
    pub fn iter_row(&self, y: i32) -> RectRowIter {
        RectRowIter {
            rect: self.clone(),
            next_x: self.t_l.x,
            y,
        }
    }

    #[allow(unused)]
    /// iterates through a single row
    pub fn iter_column(&self, x: i32) -> RectColIter {
        RectColIter {
            rect: self.clone(),
            next_y: self.t_l.y,
            x,
        }
    }
}

use crate::resource::AreaMap;
impl From<&AreaMap> for Rect {
    fn from(map: &AreaMap) -> Rect {
        Rect {
            t_l: Position::new(0, 0),
            b_r: Position::new(map.width, map.height),
        }
    }
}

#[derive(Debug)]
/// iterates inclusively through a rectangle's points
pub struct RectIter {
    rect: Rect,
    next_pos: Position,
}

impl Iterator for RectIter {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        let res = self.next_pos.clone();
        if res.y > self.rect.b_r.y {
            return None;
        }
        self.next_pos.x += 1;
        if self.next_pos.x > self.rect.b_r.x {
            self.next_pos.x = self.rect.t_l.x;
            self.next_pos.y += 1;
        }
        Some(res)
    }
}

#[derive(Debug)]
/// iterates clockwise inclusively through a rectangle's perimeter, starting at the top
/// left corner and stopping at one tile below the top left corner
pub struct RectIterPerimeter {
    rect: Rect,
    next_pos: Position,
    final_item: bool,
}

impl Iterator for RectIterPerimeter {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        if self.final_item == true {
            return None;
        }
        let res = self.next_pos.clone();
        if self.next_pos.y == self.rect.t_l.y && self.next_pos.x < self.rect.b_r.x {
            self.next_pos.x += 1;
        } else if self.next_pos.x == self.rect.b_r.x && self.next_pos.y < self.rect.b_r.y {
            self.next_pos.y += 1;
        } else if self.next_pos.x > self.rect.t_l.x && self.next_pos.y == self.rect.b_r.y {
            self.next_pos.x -= 1;
        } else if self.next_pos.y > self.rect.t_l.y {
            self.next_pos.y -= 1;
        }
        if self.next_pos == self.rect.t_l {
            self.final_item = true;
        }
        Some(res)
    }
}

impl IntoIterator for Rect {
    type Item = Position;
    type IntoIter = RectIter;

    fn into_iter(self) -> RectIter {
        let iter = RectIter {
            next_pos: Position::new(self.t_l.x, self.t_l.y),
            rect: self,
        };
        return iter;
    }
}

#[derive(Debug)]
#[allow(unused)]
/// iterates through horizontal slices of the rectangle as vecs of positions
pub struct RectRowsIter {
    rect: Rect,
    next_row: i32,
}

impl Iterator for RectRowsIter {
    type Item = Vec<Position>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_row > self.rect.b_r.y {
            return None;
        }
        let res = (self.rect.t_l.x..=self.rect.b_r.x)
            .map(|x| Position {
                x,
                y: self.next_row,
            })
            .collect();
        self.next_row += 1;
        Some(res)
    }
}

#[allow(unused)]
pub struct RectRowIter {
    rect: Rect,
    y: i32,
    next_x: i32,
}

impl Iterator for RectRowIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_x <= self.rect.b_r.x {
            self.next_x += 1;
            return Some(Position::new(self.next_x - 1, self.y));
        }
        return None;
    }
}

#[derive(Debug)]
/// iterates through vertical slices of the rectangle as vecs of positions
pub struct RectColsIter {
    rect: Rect,
    next_col: i32,
}

impl Iterator for RectColsIter {
    type Item = Vec<Position>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_col > self.rect.b_r.x {
            return None;
        }
        let res = (self.rect.t_l.y..=self.rect.b_r.y)
            .map(|y| Position {
                x: self.next_col,
                y,
            })
            .collect();
        self.next_col += 1;
        Some(res)
    }
}

#[allow(unused)]
pub struct RectColIter {
    rect: Rect,
    x: i32,
    next_y: i32,
}

impl Iterator for RectColIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_y <= self.rect.b_r.y {
            self.next_y += 1;
            return Some(Position::new(self.x, self.next_y - 1));
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rect_iter() {
        {
            let mut iter = Rect::new(Position::new(0, 0), Position::new(1, 1)).into_iter();
            assert_eq!(iter.next().unwrap(), Position { x: 0, y: 0 });
            assert_eq!(iter.next().unwrap(), Position { x: 1, y: 0 });
            assert_eq!(iter.next().unwrap(), Position { x: 0, y: 1 });
            assert_eq!(iter.next().unwrap(), Position { x: 1, y: 1 });
            assert_eq!(iter.next(), None);
        }
        {
            let rect = Rect::new(Position::new(0, 0), Position::new(4, 4));
            let mut count: u8 = 0;
            for _pos in rect.iter() {
                count += 1;
            }
            assert_eq!(count, 25);
        }
        {
            let rect = Rect::new(Position::new(0, 0), Position::new(2, 0));
            let mut count: u8 = 0;
            for _pos in rect.iter() {
                count += 1;
            }
            assert_eq!(count, 3);
        }
        {
            let rect = Rect::new(Position::new(0, 0), Position::new(0, 4));
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
            let mut iter = Rect::new(Position::new(0, 0), Position::new(2, 2)).iter_perimeter();
            assert_eq!(iter.next().unwrap(), Position { x: 0, y: 0 });
            assert_eq!(iter.next().unwrap(), Position { x: 1, y: 0 });
            assert_eq!(iter.next().unwrap(), Position { x: 2, y: 0 });
            assert_eq!(iter.next().unwrap(), Position { x: 2, y: 1 });
            assert_eq!(iter.next().unwrap(), Position { x: 2, y: 2 });
            assert_eq!(iter.next().unwrap(), Position { x: 1, y: 2 });
            assert_eq!(iter.next().unwrap(), Position { x: 0, y: 2 });
            assert_eq!(iter.next().unwrap(), Position { x: 0, y: 1 });
            assert_eq!(iter.next(), None);
        }
        {
            let mut iter = Rect::new(Position::new(1, 1), Position::new(2, 2)).iter_perimeter();
            assert_eq!(iter.next().unwrap(), Position { x: 1, y: 1 });
            assert_eq!(iter.next().unwrap(), Position { x: 2, y: 1 });
            assert_eq!(iter.next().unwrap(), Position { x: 2, y: 2 });
            assert_eq!(iter.next().unwrap(), Position { x: 1, y: 2 });
            assert_eq!(iter.next(), None);
        }
    }

    #[test]
    fn rect_iter_column() {
        let mut iter = Rect::new(Position::new(0, 0), Position::new(2, 2)).iter_column(1);
        assert_eq!(iter.next().unwrap(), Position { x: 1, y: 0 });
        assert_eq!(iter.next().unwrap(), Position { x: 1, y: 1 });
        assert_eq!(iter.next().unwrap(), Position { x: 1, y: 2 });
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn rect_iter_row() {
        let mut iter = Rect::new(Position::new(0, 0), Position::new(2, 2)).iter_row(1);
        assert_eq!(iter.next().unwrap(), Position { x: 0, y: 1 });
        assert_eq!(iter.next().unwrap(), Position { x: 1, y: 1 });
        assert_eq!(iter.next().unwrap(), Position { x: 2, y: 1 });
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn rect_is_corner() {
        type Pos = Position;
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
