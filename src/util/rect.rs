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

    pub fn to_wave_size(&self) -> Size {
        Size::new(self.width() as u32, self.height() as u32)
    }

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

    /// iterates through the rows in a rectangle, yielding each row as a vector
    /// of positions
    pub fn iter_rows(&self) -> RectRowIter {
        RectRowIter {
            rect: self.clone(),
            next_row: self.t_l.y,
        }
    }

    /// iterates through the columns in a rectangle, yielding each column as a vector
    /// of positions
    pub fn iter_columns(&self) -> RectColIter {
        RectColIter {
            rect: self.clone(),
            next_col: self.t_l.x,
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
            println!("incrementing x");
            self.next_pos.x += 1;
        } else if self.next_pos.x == self.rect.b_r.x && self.next_pos.y < self.rect.b_r.y {
            println!("incrementing y");
            self.next_pos.y += 1;
        } else if self.next_pos.x > self.rect.t_l.x && self.next_pos.y == self.rect.b_r.y {
            println!("decrementing x");
            self.next_pos.x -= 1;
        } else if self.next_pos.y > self.rect.t_l.y {
            println!("decrementing y");
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
/// iterates through horizontal slices of the rectangle as vecs of positions
pub struct RectRowIter {
    rect: Rect,
    next_row: i32,
}

impl Iterator for RectRowIter {
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

#[derive(Debug)]
/// iterates through vertical slices of the rectangle as vecs of positions
pub struct RectColIter {
    rect: Rect,
    next_col: i32,
}

impl Iterator for RectColIter {
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
            for pos in rect.iter() {
                count += 1;
            }
            assert_eq!(count, 25);
        }
        {
            let rect = Rect::new(Position::new(0, 0), Position::new(2, 0));
            let mut count: u8 = 0;
            for pos in rect.iter() {
                count += 1;
            }
            assert_eq!(count, 3);
        }
        {
            let rect = Rect::new(Position::new(0, 0), Position::new(0, 4));
            let mut count: u8 = 0;
            for pos in rect.iter() {
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
}
