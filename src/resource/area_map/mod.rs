use crate::component::{Position, Region};
use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::resource::GeographyTemplate;
use crate::util::Rect;

mod iterators;
mod tile;
use iterators::AreaMapIter;
pub use tile::Tile;

pub const WIDTH: usize = MAP_WIDTH as usize;
pub const HEIGHT: usize = MAP_HEIGHT as usize;

#[derive(Clone)]
pub struct AreaMap {
    tiles: Vec<Vec<Tile>>,
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
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
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

    /// finds the largest rectangle that is unoccupied within the bounds provided
    /// by <room>
    /// uses a stack-based solution for determining the largest rectangle in
    /// each column, then picks the overall largest
    pub fn fit_rect(&self, room: Rect) -> Rect {
        // this is our height histogram, we populate it from the map
        let zero: u32 = 0;
        let mut cells = vec![vec![zero; self.width as usize]; self.height as usize];
        let mut height: u32 = 0;
        let mut max_area: u32 = 0;
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

        // solve largest rectangle in histogram for each column
        let mut stack: Vec<(usize, u32)> = Vec::new();
        // bottom-right corner
        let mut b_r: Position = Position::new(0, 0);
        let mut t_l: Position = Position::new(0, 0);
        let mut check = |x: usize, y: usize, (stack_x, stack_height): (usize, u32)| {
            let cur_width = (x - stack_x) as u32;
            let temp_area = stack_height * cur_width;
            if temp_area > max_area {
                max_area = temp_area;
                t_l = Position::new(stack_x as i32, y as i32 - (stack_height as i32 - 1));
                b_r = Position::new(t_l.x + (cur_width as i32 - 1), y as i32);
            }
        };
        for (y, row) in cells.iter().enumerate() {
            for (x, height) in row.iter().cloned().enumerate() {
                let last: Option<(usize, u32)>;
                {
                    last = stack.iter().cloned().last();
                }
                if stack.len() == 0 {
                    stack.push((x, height));
                }
                if let Some(entry) = last {
                    if height > entry.1 {
                        {
                            stack.push((x, height));
                        }
                    } else if height < entry.1 {
                        let mut consumed: usize = 0;
                        for entry in stack.iter().cloned().rev() {
                            if height > entry.1 {
                                break;
                            } else {
                                check(x, y, entry);
                                consumed += 1;
                            }
                        }
                        if consumed > 0 {
                            while consumed > 0 {
                                stack.pop();
                                consumed -= 1;
                            }
                            let len = stack.len();
                            stack.push((len, height));
                        }
                    }
                }
            }
            for entry in stack.drain(0..).rev() {
                check(row.len(), y, entry);
            }
        }
        Rect { t_l, b_r }
    }
}

use specs::{Component, VecStorage};
use std::collections::hash_map::IterMut;
use std::collections::HashMap;
#[derive(Clone, Default, Component)]
#[storage(VecStorage)]
pub struct AreaMaps {
    maps: HashMap<Region, AreaMap>,
}

impl AreaMaps {
    /// initialize new maps for a given <center> and <radius> radius
    /// Note that radius extends from the edge of the center, so a "size 2" map is 5x5
    pub fn init(&mut self, center: Region, size: u8) {
        let s = i32::from(size); // size is only u8 to enforce an unsigned parameter
        let mut count: i32 = 0;
        let min_x = center.x - s;
        let max_x = center.x + s + 1;
        let min_y = center.y - s;
        let max_y = center.y + s + 1;
        for x in min_x..max_x {
            for y in min_y..max_y {
                let region = Region::new(x, y);
                self.maps.entry(region).or_insert_with(|| {
                    count += 1;
                    AreaMap::default()
                });
            }
        }
        if count > 0 {
            println!(
                "initialized {} new maps at center {:?}, size {}",
                count, center, size
            );
        }
    }

    /// get the map at the given location. Will probably die if the map doesn't exist,
    /// but we want that because it shouldn't have happened.
    pub fn get(&self, region: Region) -> &AreaMap {
        match self.maps.get(&region) {
            Some(map) => map,
            None => {
                panic!(format!("no map for region {:?}", region));
            }
        }
    }

    /// checks whether a map is in play
    pub fn has(&self, region: Region) -> bool {
        self.maps.get(&region).is_some()
    }

    /// check if the map for the given region is ready for play.
    pub fn ready(&self, region: Region) -> bool {
        match self.maps.get(&region) {
            Some(map) => map.populated,
            None => false,
        }
    }

    pub fn populated(&self) -> bool {
        let mut result = true;
        for (_, map) in self.maps.iter() {
            result = result && map.populated
        }
        result
    }

    /// prunes maps in collection farther than <size> maps from <center> in a square
    pub fn prune(&mut self, center: Region, size: u8) {
        let s = i32::from(size);
        let mut count: u32 = 0;
        let mut marked: Vec<Region> = Vec::new();
        for (region, _) in self.maps.iter() {
            if (center.x - region.x).abs() > s || (center.y - region.y).abs() > s {
                marked.push(region.clone());
            }
        }
        for mark in marked {
            println!("pruning {:?}", mark);
            self.maps.remove(&mark);
            count += 1;
        }
        if count > 0 {
            println!("pruned {} maps", count);
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Region, AreaMap> {
        self.maps.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::{Color, Description, Position};
    #[test]
    fn fit_rect() {
        let mut map = AreaMap::with_dimensions(5, 5);
        let occupied = Tile::new(
            '#',
            Color::default(),
            Color::default(),
            false,
            false,
            true,
            Description::default(),
        );
        map.set(Position::new(0, 0), occupied.clone());
        map.set(Position::new(3, 0), occupied.clone());
        map.set(Position::new(0, 3), occupied.clone());
        map.set(Position::new(3, 4), occupied.clone());
        map.set(Position::new(4, 4), occupied.clone());
        {
            let rect = Rect::new(Position::new(0, 0), Position::new(4, 4));
            let expect_t_l = Position::new(1, 1);
            let expect_b_r = Position::new(4, 3);
            let res = dbg!(map.fit_rect(rect));
            assert!(res.t_l == expect_t_l, "top left correct");
            assert!(res.b_r == expect_b_r, "bottom right correct");
        };
        {
            let rect = Rect::new(Position::new(1, 1), Position::new(3, 3));
            let expect_t_l = Position::new(1, 1);
            let expect_b_r = Position::new(3, 3);
            let res = dbg!(map.fit_rect(rect));
            assert!(res.t_l == expect_t_l, "top left correct");
            assert!(res.b_r == expect_b_r, "bottom right correct");
        };
        {
            let rect = Rect::new(Position::new(0, 0), Position::new(1, 3));
            let expect_t_l = Position::new(0, 1);
            let expect_b_r = Position::new(1, 2);
            let res = dbg!(map.fit_rect(rect));
            assert!(res.t_l == expect_t_l, "top left correct");
            assert!(res.b_r == expect_b_r, "bottom right correct");
        };
    }
}
