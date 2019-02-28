use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StructureConnectionType {
    Road,
    Structure(StructureTemplate),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StructureConnectionMethod {
    Driveway,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StructureConnection {
    connection_type: StructureConnectionType,
    connection_method: StructureConnectionMethod,
}

/// A map of chars to tile IDs. Chars are used to label tiles in templates.
type CharMap = HashMap<char, u32>;

/// a map of ids and their corresponding chars (reverse of CharMap)
type MapChar = HashMap<u32, char>;

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StructureTile {
    /// identify a tile by a single char, this is arbitrary
    /// and defined in the template
    // label: char,
    /// the character that will be displayed
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    fg: (u8, u8, u8),
    #[serde(default)]
    bg: (u8, u8, u8),
    #[serde(default = "default_true")]
    pub transparent: bool,
    #[serde(default = "default_true")]
    pub walkable: bool,
    #[serde(default)]
    weight: u32,
    #[serde(default)]
    allowed_neighbors: (Vec<char>, Vec<char>, Vec<char>, Vec<char>),
}

impl Default for StructureTile {
    fn default() -> StructureTile {
        StructureTile {
            icon: "?".to_string(),
            fg: (255, 255, 255),
            bg: (0, 0, 0),
            transparent: true,
            walkable: true,
            weight: 1,
            allowed_neighbors: (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
        }
    }
}

use crate::component::Color;
impl StructureTile {
    pub fn fg(&self) -> Color {
        Color::from(self.fg)
    }

    pub fn bg(&self) -> Color {
        Color::from(self.bg)
    }
    pub fn pattern_description(&self, charmap: &CharMap) -> PatternDescription {
        use core::num::NonZeroU32;
        PatternDescription {
            weight: NonZeroU32::new(self.weight),
            allowed_neighbours: CardinalDirectionTable::new_array([
                self.allowed_neighbors
                    .0
                    .iter()
                    .map(|ch| *charmap.get(ch).unwrap_or(&0))
                    .collect(),
                self.allowed_neighbors
                    .1
                    .iter()
                    .map(|ch| *charmap.get(ch).unwrap_or(&0))
                    .collect(),
                self.allowed_neighbors
                    .2
                    .iter()
                    .map(|ch| *charmap.get(ch).unwrap_or(&0))
                    .collect(),
                self.allowed_neighbors
                    .3
                    .iter()
                    .map(|ch| *charmap.get(ch).unwrap_or(&0))
                    .collect(),
            ]),
        }
    }
}

use std::collections::HashMap;
use wfc::{PatternDescription, PatternTable};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StructureTemplate {
    pub min_width: i32,
    pub max_width: i32,
    pub min_height: i32,
    pub max_height: i32,
    /// perimeter is *inside* the bounds, so account for it in min/max properties
    pub perimeter: i32,
    /// a special instruction for connecting to roads, other structures, etc
    pub connect_to: Option<Vec<StructureConnection>>,
    /// this contains all the tiles and rules for the structure to pass to the wfc system
    /// 'char' is an arbitrary character for convenient use in the template
    pub tiles: HashMap<char, StructureTile>,
}

impl Default for StructureTemplate {
    fn default() -> StructureTemplate {
        StructureTemplate {
            min_width: 3,
            max_width: 3,
            min_height: 3,
            max_height: 3,
            perimeter: 1,
            connect_to: None,
            tiles: HashMap::new(),
        }
    }
}

use direction::*;
impl StructureTemplate {
    /// builds a charmap of the structure's tiles
    pub fn get_charmap(&self) -> CharMap {
        let mut charmap: CharMap = HashMap::new();
        self.tiles.keys().enumerate().for_each(|(i, key)| {
            charmap.insert(*key, i as u32);
        });
        return charmap;
    }
    pub fn get_mapchar(&self) -> MapChar {
        let mut mapchar: MapChar = HashMap::new();
        self.tiles.keys().enumerate().for_each(|(i, key)| {
            mapchar.insert(i as u32, *key);
        });
        return mapchar;
    }

    pub fn get_pattern_table(&self) -> PatternTable<PatternDescription> {
        let charmap = self.get_charmap();
        let descs = self
            .tiles
            .values()
            .map(|tile| tile.pattern_description(&charmap))
            .collect();
        return PatternTable::from_vec(descs);
    }

    pub fn get_tile(&self, label: char) -> StructureTile {
        let default = StructureTile::default();
        let tile = self.tiles.get(&label).unwrap_or(&default);
        // FIXME this sucks?
        tile.clone()
    }
}
