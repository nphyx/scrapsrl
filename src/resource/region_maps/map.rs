use super::iterators::RegionMapIter;
use super::{Tile, HEIGHT, WIDTH};
use crate::component::Pos;
use crate::resource::GeographyTemplate;
use crate::util::{Grid, Rect};

#[derive(Clone)]
pub struct RegionMap {
    /// don't really like this being public FIXME
    pub grid: Grid<Tile>,
    // grid needs to be public for mapgen ...
    /// mark true when mapgen is complete
    pub populated: bool,
    pub geography: GeographyTemplate,
}

impl Default for RegionMap {
    fn default() -> RegionMap {
        let grid = Grid::with_dimensions(WIDTH, HEIGHT);
        RegionMap {
            grid,
            populated: false,
            geography: GeographyTemplate::default(),
        }
    }
}

impl RegionMap {
    #[allow(unused)]
    fn with_dimensions(width: usize, height: usize) -> RegionMap {
        let grid = Grid::with_dimensions(width, height);
        RegionMap {
            grid,
            populated: false,
            geography: GeographyTemplate::default(),
        }
    }

    pub fn height(&self) -> usize {
        self.grid.height()
    }

    pub fn width(&self) -> usize {
        self.grid.width()
    }

    #[allow(unused)]
    pub fn wipe(&mut self) {
        self.grid.clear();
        self.populated = false;
    }

    pub fn bounds(&self) -> Rect<usize> {
        self.grid.bounds
    }

    pub fn get(&self, pos: Pos) -> Option<&Tile> {
        self.grid.maybe_get(pos)
    }

    #[allow(unused)]
    pub fn get_mut(&mut self, pos: Pos) -> Option<&mut Tile> {
        self.grid.maybe_get_mut(pos)
    }

    /// Shortcut function for getting the icon for a tile.
    pub fn get_icon(&self, pos: Pos) -> char {
        self.grid.maybe_get(pos).map_or('?', |t| t.icon)
    }

    pub fn unchecked_set(&mut self, pos: Pos, tile: Tile) {
        self.grid.unchecked_set(pos, tile)
    }

    pub fn try_set(&mut self, pos: Pos, tile: Tile) -> Result<bool, &'static str> {
        self.grid.try_set(pos, tile)
    }

    /// Shortcut function for setting the icon for a tile. If the tile doesn't
    /// exist for some reason (e.g. out of bounds) it fails quietly. This shouldn't
    /// be a problem, typically means something went midly haywire during map
    /// generation, which we don't want to panic on.
    pub fn set_icon(&mut self, pos: Pos, icon: char) {
        if let Some(tile) = self.grid.maybe_get_mut(pos) {
            tile.icon = icon
        };
    }

    pub fn iter(&self) -> RegionMapIter<'_> {
        RegionMapIter {
            map: self,
            cur: [0, 0],
        }
    }

    #[allow(unused)]
    pub fn bounding_rect(&self) -> Rect<usize> {
        self.grid.bounds
    }

    /// paste a subgrid into a map, starting at <t_l> top-left corner position
    /// consumes the subgrid in the process
    pub fn paste_into(&mut self, t_l: Pos, subgrid: Grid<Tile>) -> Result<bool, &'static str> {
        self.grid.paste_into(t_l, subgrid)
    }
}

impl From<&RegionMap> for Rect<usize> {
    fn from(map: &RegionMap) -> Rect<usize> {
        Rect {
            t_l: Pos::new(0, 0),
            b_r: Pos::new(map.width(), map.height()),
        }
    }
}

impl std::fmt::Debug for RegionMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\ngeography: {:?}\npopulated: {}\n\n{:?}",
            self.geography
                .description
                .as_ref()
                .map_or("unknown".to_string(), |d| d.short.clone()),
            self.populated,
            self.grid
        )
    }
}

/// Prints a nicely formated map with axis labels using the UTF-8 codepoint corresponding to the
/// tile index. Most of the tile output will be nonsensical since the bitmap font used as a tileset
/// does not attempt to match up to sensible characters, but this will at least show what is going
/// on with the map.
impl std::fmt::Debug for Grid<Tile> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let width = self.width();
        let mut x = 0;
        write!(
            f,
            "{}\n{}\n{}",
            format!(
                "   |{}",
                (0..width)
                    .enumerate()
                    .map(|(i, _)| format!("{: >2}", i))
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
                        x - 1,
                        row.iter()
                            .map(|tile| format!("{: >2}", tile.icon))
                            .collect::<String>(),
                    )
                })
                .collect::<String>()
        )
    }
}
