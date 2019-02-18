use super::util::*;
use crate::component::{Color, Position};
use crate::resource::{tile_types::*, AreaMap, Tile};
use crate::util::colors::lerp;
use tcod::noise::Noise;

const VEHICLES: [char; 1] = ['F'];
// FIXME use icon assets here
// FIXME redo tile description system

/// places a car
fn place_car(
    map: &mut AreaMap,
    pos: [i32; 2],
    offset: [i32; 2],
    noise: &Noise,
    scale: f32,
    damage_factor: f32,
) {
    let color_good = Color::new(68, 68, 68);
    let color_bad = Color::new(72, 40, 36);
    let v = rand_up(fbm_offset(noise, pos, offset, 1.0, 1));
    let ch = VEHICLES[(v * VEHICLES.len() as f32).floor() as usize];
    let i = turb_offset(noise, pos, offset, scale, 32);
    let fg = lerp(color_good, color_bad, i * damage_factor);
    let pos = Position {
        x: pos[0],
        y: pos[1],
    };
    if let Some(tile) = map.get(pos) {
        map.set(pos, Tile::new(ch, fg, tile.bg, true, false, TYPE_VEHICLE))
    }
}

fn damaged_road(ground_bg: Color, road_bg: Color, blend_factor: f32) -> Tile {
    let grass_fg = Color {
        r: 102,
        g: 161,
        b: 94,
    };
    let bg = lerp(ground_bg, road_bg, blend_factor * 0.5);
    Tile::new(',', grass_fg, bg, true, true, TYPE_ROAD_CRACKED)
}

/// creates a single segment of road
fn road_segment(icon: char, fg: Color, bg: Color) -> Tile {
    Tile::new(icon, fg, bg, true, true, TYPE_ROAD)
}

/// determines the vertical offset of a horizontal road at a given x position
fn road_lat(noise: &Noise, map: &AreaMap, x: i32, offset: [i32; 2]) -> f32 {
    let hh = map.height / 2;
    rand_up(fbm_offset(noise, [x, hh], offset, 0.01, 1)) * map.height as f32
}

/// generates a horizontal road on the map. Damage factor is a range from 0 = pristine
/// to +1 = completely wrecked.
pub fn place_horizontal_roads(
    noise: &Noise,
    map: &mut AreaMap,
    offset: [i32; 2],
    noise_scale: f32,
    damage_factor: f32,
    lanes: i32,
) {
    let road_line_fg = Color {
        r: 102,
        g: 92,
        b: 81,
    };
    let road_line_center = Color {
        r: 104,
        g: 90,
        b: 61,
    };
    let road_bg = Color { r: 4, g: 4, b: 4 };
    let road_rubble_fg = Color { r: 5, g: 5, b: 5 };
    for cx in 0..map.width {
        let y = road_lat(noise, map, cx, offset).floor() as i32;
        let y_min = y - (lanes);
        let y_max = y + (lanes);
        for cy in y_min..=y_max {
            let i = rand_up(turb_offset(noise, [cx, cy], offset, noise_scale, 32));
            let pos = Position { x: cx, y: cy };
            let mut ground_bg = Color::new(0, 0, 0);
            if y > 0 && y < map.height {
                if i < damage_factor {
                    if let Some(tile) = map.get(pos) {
                        ground_bg = tile.bg;
                    }
                    map.set(pos, damaged_road(ground_bg, road_bg, i));
                } else if cy == y_min || cy == y_max {
                    // outer line
                    map.set(pos, road_segment('=', road_line_fg, road_bg));
                } else if cy == y {
                    // center line
                    let icon = if lanes > 2 { '=' } else { '-' };
                    map.set(pos, road_segment(icon, road_line_center, road_bg));
                } else if (cy - y) % 2 == 0 {
                    // lane seperator
                    map.set(pos, road_segment('-', road_line_fg, road_bg));
                } else {
                    map.set(pos, road_segment('\u{e35d}', road_rubble_fg, road_bg));
                }
            }
            let car_chance = fbm_offset(noise, [cx, cy], offset, 1.0, 32);
            if car_chance > 0.95 {
                place_car(map, [cx, cy], offset, noise, noise_scale, damage_factor);
            }
        }
    }
}
