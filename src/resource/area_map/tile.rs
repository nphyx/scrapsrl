use crate::component::{Color, Description};

#[derive(Clone)]
pub struct Tile {
    pub icon: char,
    pub fg: Color,
    pub bg: Color,
    pub transparent: bool,
    pub walkable: bool,
    /// indicates the tile has had a non-ground tile placed in it
    /// used to prevent overlapping structures during generation
    pub constructed: bool,
    pub description: Description,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            icon: ' ',
            fg: Color::new(255, 255, 255),
            bg: Color::new(0, 0, 0),
            transparent: true,
            walkable: true,
            constructed: false,
            description: Description::default(),
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
        constructed: bool,
        description: Description,
    ) -> Tile {
        Tile {
            icon,
            fg,
            bg,
            transparent,
            walkable,
            constructed,
            description,
        }
    }
}
