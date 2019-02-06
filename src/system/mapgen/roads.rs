use crate::area_map::{AreaMap, Tile};
use tcod::noise::Noise;
use tcod::colors::{Color, lerp};
use crate::component::Position;
use super::util::*;
use crate::util::{clamp, icons::*};
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};

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
fn place_car(map: &mut AreaMap, pos:[i32; 2], offset: [i32; 2], noise: &Noise, scale: f32, damage_factor: f32, bg: Color) {
  let color_good = Color::new(68, 68, 68);
  let color_bad = Color::new(72, 40, 36);
  let v = rand_up(fbm_offset(noise, pos, offset, 1.0, 32));
  let ch = VEHICLES[(v * VEHICLES.len() as f32).floor() as usize];
  let i = turb_offset(noise, pos, offset, scale, 32);
  let fg = lerp(color_good, color_bad, i * damage_factor);
  map.set(Position{x: pos[0], y: pos[1]}, prep_tile(ch, fg, bg, false, true, "a ruined vehicle", "The rusted hulk of an old automobile."));
}

fn damaged_road<'a>(road_bg: Color, blend_factor: f32) -> Tile<'a> {
  let grass_bg = Color{r:48, g:44, b:26};
  let grass_fg = Color{r:102, g:161, b:94};
  let bg = lerp(grass_bg, road_bg, blend_factor * 0.25);
  prep_tile(
    ',',
    grass_fg,
    bg,
    false,
    false,
    "grass",
    "Some grass growing through a crack in the road.")
}

fn road_segment<'a>(icon: char, fg: Color, bg: Color) -> Tile<'a> {
  let road_short_desc = "road";
  let road_long_desc = "A crumbling old road.";
  prep_tile(
    icon,
    fg,
    bg,
    false,
    false,
    road_short_desc,
    road_long_desc)
}

/// determines the vertical offset of a horizontal road at a given x position 
fn road_lat(noise: &Noise, map: &AreaMap, x: i32, offset: [i32; 2]) -> f32 {
  let hh = map.height / 2;
  rand_up(fbm_offset(noise, [x, hh], offset, 0.01, 1)) * map.height as f32
}

/// generates a horizontal road on the map. Damage factor is a range from 0 = pristine
/// to +1 = completely wrecked.
pub fn place_horizontal_roads(noise: &Noise, map: &mut AreaMap, offset: [i32; 2],
    noise_scale: f32, damage_factor: f32, lanes: i32) {
  let road_line_fg = Color{r: 102, g: 92, b: 81};
  let road_line_center = Color{r: 104, g: 90, b: 61};
  let road_bg = Color{r: 22, g: 20, b: 16};
  let road_rubble_fg = Color{r: 26, g: 23, b: 20};
  for cx in 0..map.width {
    let y = road_lat(noise, map, cx, offset).floor() as i32;
    let y_min = y - (lanes);
    let y_max = y + (lanes);
    for cy in y_min..y_max+1 {
      let i = rand_up(turb_offset(noise, [cx, cy], offset, noise_scale, 32));
      let pos = Position{x: cx, y: cy};
      if y > 0 && y < map.height {
        if i < damage_factor {
          map.set(pos, damaged_road(road_bg, i)); 
        } else if cy == y_min || cy == y_max { // outer line
          map.set(pos, road_segment(LINE_HORIZ, road_line_fg, road_bg));
        } else if cy == y { // center line
          let icon = if lanes > 2 { LINE_DBL_HORIZ } else { '-' };
          map.set(pos, road_segment(icon, road_line_center, road_bg));
        } else if (cy - y) % 2 == 0 { // lane seperator
          map.set(pos, road_segment('-', road_line_fg, road_bg));
        } else {
          map.set(pos, road_segment('\u{e35d}', road_rubble_fg, road_bg));
        }
      }
      let car_chance = fbm_offset(noise, [cx, cy], offset, 1.0, 32);
      if car_chance > 0.95 {
        place_car(map, [cx, cy], offset, noise, noise_scale, damage_factor, road_bg);
      }
    }
  }
}
