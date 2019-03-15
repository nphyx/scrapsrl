use crate::component::Description;
use crate::resource::{Assets, Tile};
use crate::util::Rect;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StructureConnectionType {
    Road,              // place connection facing the nearest road, if on the structure perimeter
    Structure(String), // place connection to an adjacent structure
    Outside,           // place connection on the outer structure perimeter
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StructureConnectionMethod {
    Driveway, // draw a driveway (this is fixed functionality determined by geography, TODO)
    Walkway,  // draw a walkway (this is fixed functionality determined by geography, TODO)
    Door,     // place a door for this connection
    Open,     // place an opening in the wall for this connection
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum StructureTilePos {
    /// can be placed in any position
    Any,
    /// acts as a floor
    Floor,
    /// may only be adjacent to the room perimeter
    Perimeter,
    /// treat as free-standing furniture
    Fixture,
    /// does not have any automatic placement (only specified rules)
    NoAuto,
}

fn default_tile_position() -> StructureTilePos {
    StructureTilePos::Any
}

fn default_weight() -> u32 {
    1
}

fn default_empty_string() -> String {
    "".to_string()
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
    #[serde(default = "default_empty_string")]
    short_desc: String,
    #[serde(default = "default_empty_string")]
    long_desc: String,
    #[serde(default = "default_true")]
    pub transparent: bool,
    #[serde(default = "default_true")]
    pub walkable: bool,
    #[serde(default = "default_tile_position")]
    pub position: StructureTilePos,
    #[serde(default = "default_weight")]
    weight: u32,
    #[serde(default)]
    allowed_neighbors: (HashSet<char>, HashSet<char>, HashSet<char>, HashSet<char>),
}

use std::collections::HashSet;
impl Default for StructureTile {
    fn default() -> StructureTile {
        StructureTile {
            icon: "?".to_string(),
            fg: (255, 255, 255),
            bg: (0, 0, 0),
            transparent: true,
            walkable: true,
            position: StructureTilePos::Any,
            short_desc: "".to_string(),
            long_desc: "".to_string(),
            weight: 1,
            allowed_neighbors: (
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
                HashSet::new(),
            ),
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

    pub fn to_tile(&self, assets: &Assets) -> Tile {
        Tile {
            icon: assets.get_icon(&self.icon).base_ch(),
            fg: self.fg(),
            bg: self.bg(),
            transparent: self.transparent,
            walkable: self.walkable,
            constructed: true,
            description: Description::new(&self.short_desc, &self.long_desc),
        }
    }
}

use std::collections::HashMap;
use wfc::{PatternDescription, PatternTable};
fn default_building_slots() -> u8 {
    1
}

fn default_empty_string_vec() -> Vec<String> {
    Vec::new()
}

fn default_empty_connection_vec() -> Vec<StructureConnection> {
    Vec::new()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StructureTemplate {
    pub min_width: usize,
    pub max_width: usize,
    pub min_height: usize,
    pub max_height: usize,
    /// perimeter is *inside* the bounds, so account for it in min/max properties
    pub perimeter: usize,
    #[serde(default = "default_building_slots")]
    /// maps have a cap on the number of structures they can make;
    /// this is number of slots this structure should count for
    pub building_slots: u8,
    #[serde(default = "StructureTile::default")]
    /// tile used for outer walls
    pub perimeter_tile: StructureTile,
    #[serde(default = "default_empty_connection_vec")]
    /// a special instruction for connecting to roads, other structures, etc
    pub connect_to: Vec<StructureConnection>,
    #[serde(default = "default_empty_string_vec")]
    /// nested structures that may be placed inside this structure
    pub interior_structures: Vec<String>,
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
            building_slots: 1,
            perimeter_tile: StructureTile::default(),
            connect_to: Vec::new(),
            interior_structures: Vec::new(),
            tiles: HashMap::new(),
        }
    }
}

use direction::*;
impl StructureTemplate {
    /// does cleanup and reconciliation of allowed neighbors rules
    pub fn init(&mut self) {
        let mut floors: HashSet<char> = HashSet::new();

        // gather up all the floors, which can go next to most other tiles
        // and we don't want to have to write them over and over again
        // in templates
        for (ch, tile) in &mut self.tiles {
            if tile.position == StructureTilePos::Floor {
                floors.insert(*ch);
            };
        }
        // add floors as neighbors to Any and Fixture types
        for tile in self.tiles.values_mut() {
            if tile.position == StructureTilePos::Fixture || tile.position == StructureTilePos::Any
            {
                for ch in &floors {
                    tile.allowed_neighbors.0.insert(*ch);
                    tile.allowed_neighbors.1.insert(*ch);
                    tile.allowed_neighbors.2.insert(*ch);
                    tile.allowed_neighbors.3.insert(*ch);
                }
            }
        }

        // now let's make sure all the values are properly mirrored
        // order of directions is north_of, east_of, south_of, west_of
        let mut north_of: HashMap<char, HashSet<char>> = HashMap::new();
        let mut west_of: HashMap<char, HashSet<char>> = HashMap::new();
        let mut east_of: HashMap<char, HashSet<char>> = HashMap::new();
        let mut south_of: HashMap<char, HashSet<char>> = HashMap::new();
        for (ch, tile) in self.tiles.clone() {
            for i in tile.allowed_neighbors.0 {
                let entry = south_of.entry(ch).or_insert_with(HashSet::new);
                entry.insert(i);
                let entry = north_of.entry(i).or_insert_with(HashSet::new);
                entry.insert(ch);
            }
            for i in tile.allowed_neighbors.2 {
                let entry = north_of.entry(ch).or_insert_with(HashSet::new);
                entry.insert(i);
                let entry = south_of.entry(i).or_insert_with(HashSet::new);
                entry.insert(ch);
            }
            for i in tile.allowed_neighbors.1 {
                let entry = west_of.entry(ch).or_insert_with(HashSet::new);
                entry.insert(i);
                let entry = east_of.entry(i).or_insert_with(HashSet::new);
                entry.insert(ch);
            }
            for i in tile.allowed_neighbors.3 {
                let entry = east_of.entry(ch).or_insert_with(HashSet::new);
                entry.insert(i);
                let entry = west_of.entry(i).or_insert_with(HashSet::new);
                entry.insert(ch);
            }
        }

        for (ch, set) in north_of {
            let tile = self.tiles.get_mut(&ch).unwrap();
            for i in set {
                tile.allowed_neighbors.2.insert(i);
            }
        }
        for (ch, set) in south_of {
            let tile = self.tiles.get_mut(&ch).unwrap();
            for i in set {
                tile.allowed_neighbors.0.insert(i);
            }
        }
        for (ch, set) in west_of {
            let tile = self.tiles.get_mut(&ch).unwrap();
            for i in set {
                tile.allowed_neighbors.1.insert(i);
            }
        }
        for (ch, set) in east_of {
            let tile = self.tiles.get_mut(&ch).unwrap();
            for i in set {
                tile.allowed_neighbors.3.insert(i);
            }
        }
        /*
        let complete: Vec<(
            char,
            (HashSet<char>, HashSet<char>, HashSet<char>, HashSet<char>),
        )> = self
            .tiles
            .iter()
            .map(|(c, t)| (*c, t.allowed_neighbors.clone()))
            .collect();
        for item in complete {
            println!("CH --- NORTH -- EAST -- SOUTH -- WEST");
            println!("{:?}", item);
        }
        */
    }

    /// builds a charmap of the structure's tiles
    pub fn get_charmap(&self) -> CharMap {
        let mut charmap: CharMap = HashMap::new();
        self.tiles.keys().enumerate().for_each(|(i, key)| {
            charmap.insert(*key, i as u32);
        });
        charmap
    }
    pub fn get_mapchar(&self) -> MapChar {
        let mut mapchar: MapChar = HashMap::new();
        self.tiles.keys().enumerate().for_each(|(i, key)| {
            mapchar.insert(i as u32, *key);
        });
        mapchar
    }

    pub fn get_pattern_table(&self) -> PatternTable<PatternDescription> {
        let charmap = self.get_charmap();
        let descs = self
            .tiles
            .values()
            .map(|tile| tile.pattern_description(&charmap))
            .collect();
        PatternTable::from_vec(descs)
    }

    pub fn get_tile(&self, label: char) -> StructureTile {
        let default = StructureTile::default();
        let tile = self.tiles.get(&label).unwrap_or(&default);
        // FIXME this sucks?
        tile.clone()
    }

    pub fn fits_in(&self, rect: Rect<usize>) -> bool {
        rect.width() >= self.min_width && rect.height() >= self.min_height
    }
}
