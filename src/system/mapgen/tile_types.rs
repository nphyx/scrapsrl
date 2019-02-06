/// A collection of general tile types and their descriptions.
/// Not very elegant doing it this way, as this file will have
/// to be updated every time we add a new type... but it'll do for now.

pub struct TileType {
  id: u32,
  short_desc: &'static str,
  long_desc: &'static str
}

impl TileType {
  pub fn new(id: u32, short_desc: &'static str, long_desc: &'static str) -> TileType {
    TileType{id, short_desc, long_desc}
  }
}

pub const TYPE_TREE: u32 = 0;
pub const TYPE_ROAD: u32 = 1;
pub const TYPE_ROAD_CRACKED: u32 = 2;
pub const TYPE_VEHICLE: u32 = 3;
pub const TYPE_GRASS: u32 = 4;
pub const TYPE_GRASS_LONG: u32 = 5;

pub fn get_tile_descriptors() -> Vec<TileType> {
  vec![
    TileType::new(0, "", ""),
    TileType::new(TYPE_TREE, "tree", "A big tree. It seems to be thriving."),
    TileType::new(TYPE_ROAD, "road", "A crumbling old road."),
    TileType::new(TYPE_ROAD_CRACKED, "cracked road", "Some grass growing through a crack in the road."),
    TileType::new(TYPE_VEHICLE, "vehicle", "The rusted hulk of an old automobile."),
    TileType::new(TYPE_GRASS, "grass", "Just some ordinary grass."),
    TileType::new(TYPE_GRASS_LONG, "tall grass", "Some tall grass.")
  ]
}
