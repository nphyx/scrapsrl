use tcod::random::{Rng, Algo};
use tcod::noise::*;
use tcod::colors::{Color, lerp};
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::util::{clamp, icons::*};
use crate::area_map::{AreaMap, Tile};
use crate::game_state::GameState;
use crate::component::*;

mod connect_tiles;

use connect_tiles::connect;

// const SEED: u32 = 2234567891;

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

/// build a TilePrep, which will eventually be made into a tile if it ends up in the final tile
/// set.
fn prep_tile<'a>(icon: char, fg: Color, bg: Color, opaque: bool, solid: bool,
    desc_short: &'a str, desc_long: &'a str) -> Tile<'a> {
  Tile{
    icon,
    fg,
    bg,
    transparent: !opaque,
    walkable: !solid,
    desc_short,
    desc_long
  }
}


/// places a car
fn place_car(map: &mut AreaMap, x: i32, y:i32, noise: &Noise, noise_scale: f32, damage_factor: f32, bg: Color) {
  let color_good = Color::new(68, 68, 68);
  let color_bad = Color::new(72, 40, 36);
  let v = rand_up(noise.get_fbm([x as f32 * noise_scale, y as f32 * noise_scale], 32));
  let ch = VEHICLES[(v * VEHICLES.len() as f32).floor() as usize];
  let i = noise.get_turbulence([x as f32 * noise_scale, y as f32 * noise_scale], 32);
  let fg = lerp(color_good, color_bad, i * damage_factor);
  map.set(Position{x, y}, prep_tile(ch, fg, bg, false, true, "a ruined vehicle", "The rusted hulk of an old automobile."));
}

/// generates a horizontal road on the map. Damage factor is a range from 0 = pristine to +1 =
/// completely wrecked.
fn place_horizontal_road(map: &mut AreaMap, width: i32, height: i32, noise: &Noise, noise_scale: f32, damage_factor: f32) {
  let mut y = height / 2;
  let y_mod = noise.get_fbm([0.0, y as f32 * noise_scale], 32);
  y = clamp(0, MAP_HEIGHT, y + (y as f32 * y_mod) as i32);
  let road_short_desc = "road";
  let road_long_desc = "A crumbling old road.";
  let grass_short_desc = "grass";
  let grass_long_desc = "Some grass growing through a crack in the road.";
  let opaque = false;
  let solid = false;
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
        map.set(
          Position{x: cx, y: cy},
          prep_tile(ch, color_grass_fg, bg, opaque, solid, grass_short_desc, grass_long_desc));
      } else {
        if y > 0 && y < height {
          if cy == y - 3 || cy == y + 3 {
          map.set(
            Position{x: cx, y: cy},
            prep_tile(LINE_HORIZ, road_line_fg, bg, opaque, solid, road_short_desc, road_long_desc));
          } else if cy == y {
            map.set(
              Position{x: cx, y: cy},
              prep_tile('-', road_line_fg, bg, opaque, solid, road_short_desc, road_long_desc));
          } else {
            map.set(
              Position{x: cx, y: cy},
              prep_tile('\u{e35d}', road_rubble_fg, bg, opaque, solid, road_short_desc, road_long_desc));
          }
        }
      }
      let car_chance = noise.get_fbm([cx as f32, cy as f32], 32);
      if car_chance > 0.95 {
        place_car(map, cx, cy, noise, noise_scale, damage_factor, bg);
      }
    }
  }
}


fn place_tree(map: &mut AreaMap, cx: i32, cy: i32) {
  let min_x = clamp(0, MAP_WIDTH, cx - 1);
  let min_y = clamp(0, MAP_HEIGHT, cy - 1);
  let max_x = clamp(0, MAP_WIDTH, cx + 2);
  let max_y = clamp(0, MAP_HEIGHT, cy + 2);
  let fg = Color{r:86, g:50, b:32};
  let bg = Color{r:32, g:24, b:12};
  let tree_bark = prep_tile(LINE, fg, bg, true, true, "tree bark", "The bark of a tree.");
  let tree_trunk = prep_tile('o', fg, bg, true, true, "tree trunk", "The trunk of a tree.");

  for x in min_x..max_x {
    for y in min_y..max_y {
      map.set(Position{x, y}, tree_bark.clone());
    }
  }
  map.set(Position{x:cx, y:cy}, tree_trunk.clone());
}

fn check_tree_placement(tree_places: &Vec<(i32, i32)>, cx: i32, cy: i32) -> bool {
  for x in cx-3..cx+3 {
    for y in cy-3..cy+3 {
      if tree_places.contains(&(x, y)) { return false; }
    }
  }
  return true;
}

fn place_trees(tiles: &mut AreaMap, width: i32, height: i32, noise: &Noise, noise_scale: f32) {
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

fn lay_grass(map: &mut AreaMap, width: i32, height: i32, noise: &Noise, noise_scale: f32) {
  let desc_sg_short = "grass";
  let desc_sg_long = "Just some ordinary grass.";
  let desc_tg_short = "tall grass";
  let desc_tg_long = "Some tall grass.";
  let opaque = false;
  let solid = false;
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
        map.set(
          Position{x, y},
          prep_tile(',', fg, bg, opaque, solid, desc_sg_short, desc_sg_long));
      } else {
        map.set(
          Position{x, y},
          prep_tile('"', fg, bg, opaque, solid, desc_tg_short, desc_tg_long));
      }
    }
  }
}

pub struct MapGenerator {
  width: i32,
  height: i32
}

impl MapGenerator {
  pub fn new(width: i32, height: i32) -> MapGenerator {
    MapGenerator{width, height}
  }
}

use specs::{System, Write};
impl<'a> System<'a> for MapGenerator {
  type SystemData = (
    Write<'a, AreaMap<'static>>,
    Write<'a, GameState>
  );

  fn run(&mut self, (mut map, mut state): Self::SystemData) {
    if !(state.map_gen_queued) { return; }
    map.wipe();
    let rng = Rng::new_with_seed(Algo::CMWC, state.world_seed);
    let width  = self.width;
    let height = self.height;
    // let (width, height) = (&map_width, &map_height).join();
    let noise = Noise::init_with_dimensions(2)
      .noise_type(NoiseType::Simplex)
      .random(rng)
      .init();

    // lay down a basic grass layer
    lay_grass(&mut map, width, height, &noise, 0.2);

    // draw a road
    place_horizontal_road(&mut map, width, height, &noise, 0.1, 0.8);

    // place trees (ok for them to grow through the road, it's been a long time)
    place_trees(&mut map, width, height, &noise, 0.2);

    // connect connectable tiles
    connect(&mut map);


    state.map_gen_queued = false;
  }
}
