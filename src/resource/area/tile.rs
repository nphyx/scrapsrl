use crate::component::Color;

#[derive(Copy, Clone)]
pub struct Tile {
    pub icon: char,
    pub fg: Color,
    pub bg: Color,
    pub transparent: bool,
    pub walkable: bool,
    /// arbitrary type flag used by map generator to mark tiles for checks
    pub type_id: u32,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            icon: ' ',
            fg: Color::new(255, 255, 255),
            bg: Color::new(0, 0, 0),
            transparent: true,
            walkable: true,
            type_id: 0,
        }
    }
}

impl Tile {
    pub fn new(
        icon: char,
        fg: Color,
        bg: Color,
        transparent: bool,
        walkable: bool,
        type_id: u32,
    ) -> Tile {
        Tile {
            icon,
            fg,
            bg,
            transparent,
            walkable,
            type_id,
        }
    }
}
