use tcod::map::Map;
use tcod::random::{Rng, Algo};
use tcod::noise::*;
use tcod::colors::{Color, lerp};
use super::constants::{MAP_WIDTH, MAP_HEIGHT};
use super::entity::Coord;
use super::util::clamp;
use super::util::icons::*;

pub use self::tile::{Tile, ConnectedTile};
use crate::component::{Tile, Position, Color, Description, Icon};

pub fn make_tile_entity(&world: specs::World, tile: Tile, icon: Icon, colors: Colors, description: Description) -> specs::Entity {
  world.create_entity()
    .with(tile)
    .with(icon)
    .with(Position{x:0, y:0})
    .with(Colors{fg: Color{r:0, g:0, b:0}, bg: Color{r:0, g:0, b:0}})
    .with(description)
    .build()
}

const SEED: u32 = 2234567891;

pub fn rand_up(v: f32) -> f32 { (v + 1.0) / 2.0 }

const VEHICLES: [char; 9] = [
  ICON_BUS,
  ICON_DBL_BUS,
  ICON_SCHOOL_BUS,
  ICON_CONVERTIBLE,
  ICON_SUV,
  ICON_HATCHBACK,
  ICON_SEDAN,
  ICON_SPORTSCAR,
  ICON_TRUCK_PICKUP
];


/// places a car
pub fn place_car(tiles: &mut Tiles, x: i32, y:i32, noise: &Noise, noise_scale: f32, damage_factor: f32, bg: Color) {
  let color_good = Color::new(68, 68, 68);
  let color_bad = Color::new(72, 40, 36);
  let v = rand_up(noise.get_fbm([x as f32 * noise_scale, y as f32 * noise_scale], 32));
  let ch = VEHICLES[(v * VEHICLES.len() as f32).floor() as usize];
  let i = noise.get_turbulence([x as f32 * noise_scale, y as f32 * noise_scale], 32);
  let fg = lerp(color_good, color_bad, i * damage_factor);
  tiles.insert(Coord{x, y}, Tile{ch, fg, bg, transparent: true, walkable: false, desc: "A ruined vehicle."});
}

/// generates a horizontal road on the map. Damage factor is a range from 0 = pristine to +1 =
/// completely wrecked.
pub fn place_horizontal_road(tiles: &mut Tiles, width: i32, height: i32, noise: &Noise, noise_scale: f32, damage_factor: f32) {
  let mut y = height / 2;
  let y_mod = noise.get_fbm([0.0, y as f32 * noise_scale], 32);
  y = clamp(0, MAP_HEIGHT, y + (y as f32 * y_mod) as i32);
  let desc = "A crumbling old road.";
  let grass_desc = "Some grass growing through a crack in the road.";
  let transparent = true;
  let walkable = true;
  let road_line_fg = Color{r: 102, g: 92, b: 81};
  let road_bg = Color{r: 22, g: 20, b: 16};
  let road_rubble_fg = Color{r: 26, g: 23, b: 20};
  let color_grass_fg = Color{r:102, g:161, b:94};
  let color_grass_bg = Color{r:48, g:44, b:26};
  for cx in 0..width {
    let wander = noise.get_fbm([cx as f32 * noise_scale, y as f32 * noise_scale], 32);
    if wander > 0.8 {
      y += 1;
    }
    else if wander < -0.8 {
      y -= 1;
    }
    for cy in y-3..y+4 {
      let mut bg = road_bg;
      let i = rand_up(noise.get_turbulence([cx as f32 * noise_scale, cy as f32 * noise_scale], 32));
      if i < damage_factor {
        let ch = ',';
        bg = lerp(color_grass_bg, road_bg, i * 0.25);
        tiles.insert(
          Coord{x: cx, y: cy},
          Tile{ch, fg: color_grass_fg, bg, transparent, walkable, desc: grass_desc});
      } else {
        if y > 0 && y < height {
          if cy == y - 3 || cy == y + 3 {
            tiles.insert(
              Coord{x: cx, y: cy},
              Tile{ch: LINE_HORIZ, fg: road_line_fg, bg, transparent, walkable, desc});
          } else if cy == y {
            tiles.insert(
              Coord{x: cx, y: cy},
              Tile{ch: '-', fg: road_line_fg, bg, transparent, walkable, desc});
          } else {
            tiles.insert(
              Coord{x: cx, y: cy},
              Tile{ch: '\u{e35d}', fg: road_rubble_fg, bg, transparent, walkable, desc});
          }
        }
      }
      let car_chance = noise.get_fbm([cx as f32, cy as f32], 32);
      if car_chance > 0.95 {
        place_car(tiles, cx, cy, noise, noise_scale, damage_factor, bg);
      }
    }
  }
}


pub fn place_tree(tiles: &mut Tiles, cx: i32, cy: i32) {
  let min_x = clamp(0, MAP_WIDTH, cx - 1);
  let min_y = clamp(0, MAP_HEIGHT, cy - 1);
  let max_x = clamp(0, MAP_WIDTH, cx + 2);
  let max_y = clamp(0, MAP_HEIGHT, cy + 2);
  let fg = Color{r:86, g:50, b:32};
  let bg = Color{r:32, g:24, b:12};
  // let color_tg_bg = Color{r:38, g:36, b:21};
  let tree_bark: Tile = Tile{fg, bg, 
    ch: LINE, walkable: false, transparent: false,
    desc: "The bark of a tree."};
  let tree_trunk: Tile = Tile{fg, bg,
    ch: '0', walkable: false, transparent: false,
    desc: "The trunk of a tree."};

  for x in min_x..max_x {
    for y in min_y..max_y {
      tiles.insert(Coord{x, y}, tree_bark.clone());
    }
  }
  tiles.insert(Coord{x:cx, y:cy}, tree_trunk.clone());
}

fn check_tree_placement(tree_places: &Vec<(i32, i32)>, cx: i32, cy: i32) -> bool {
  for x in cx-3..cx+3 {
    for y in cy-3..cy+3 {
      if tree_places.contains(&(x, y)) { return false; }
    }
  }
  return true;
}

fn place_trees(tiles: &mut Tiles, width: i32, height: i32, noise: &Noise, noise_scale: f32) {
  let mut tree_places: Vec<(i32, i32)> = vec![];
  for x in 0..width {
    for y in 0..height {
      if check_tree_placement(&tree_places, x, y) {
        let i = rand_up(noise.get_fbm([x as f32 * noise_scale, y as f32 * noise_scale], 32));
        if i > 0.99 {
          place_tree(tiles, x, y);
          tree_places.push((x, y));
        }
      }
    }
  }
}

pub fn lay_grass(tiles: &mut Tiles, width: i32, height: i32, noise: &Noise, noise_scale: f32) {
  let desc_sg = "Just some ordinary grass.";
  let desc_tg = "Some tall grass.";
  let transparent = true;
  let walkable = true;
  let color_sg_fg = Color{r:112, g:141, b:64};
  let color_sg_bg = Color{r:42, g:54, b:28};
  let color_tg_fg = Color{r:118, g:121, b:72};
  let color_tg_bg = Color{r:38, g:36, b:21};
  for x in 0..width {
    for y in 0..height {
      let i = rand_up(noise.get_fbm([x as f32 * noise_scale, y as f32 * noise_scale], 32));
      let bg = lerp(color_sg_bg, color_tg_bg, i);
      let fg = lerp(color_sg_fg, color_tg_fg, i);
      if i < 0.5 {
        tiles.insert(
          Coord{x, y},
          Tile{ch: ',', fg, bg, transparent, walkable, desc: desc_sg});
      } else {
        tiles.insert(
          Coord{x, y},
          Tile{ch: '"', fg, bg, transparent, walkable, desc: desc_tg});
      }
    }
  }
}

pub fn generate<'a>(width: i32, height: i32) -> (Map, Tiles<'a>) {
  let mut map = Map::new(width, height);
  let mut tiles: Tiles = Tiles::new();
  let rng = Rng::new_with_seed(Algo::CMWC, SEED);
  let noise = Noise::init_with_dimensions(2)
    .noise_type(NoiseType::Simplex)
    .random(rng)
    .init();

  // lay down a basic grass layer
  lay_grass(&mut tiles, width, height, &noise, 0.2);

  // draw a road
  place_horizontal_road(&mut tiles, width, height, &noise, 0.1, 0.8);

  // place trees (ok for them to grow through the road, it's been a long time)
  place_trees(&mut tiles, width, height, &noise, 0.2);

  // connect connectable tiles
  tiles.connect_tiles();

  // assign passability based on final tile layout
  for x in 0..width {
    for y in 0..height {
      match tiles.get(Coord{x, y}) {
        Some(tile) => map.set(x, y, tile.transparent, tile.walkable),
        None => {}
      }
    }
  }

  return (map, tiles);
}
