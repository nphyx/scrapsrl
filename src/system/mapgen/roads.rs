use super::util::*;
use crate::component::{Color, Position};
use crate::resource::{tile_types::*, AreaMap, Assets, Tile};
use crate::util::colors::lerp;
use tcod::noise::Noise;

/// FIXME don't hardcode this
const VEHICLES: [&str; 8] = [
    "vehicle_sedan",
    "vehicle_convertible",
    "vehicle_hatchback",
    "vehicle_bus_school",
    "vehicle_truck_pickup",
    "vehicle_suv",
    "vehicle_bus",
    "vehicle_double_bus",
];

/// places a car
fn place_car(
    assets: &Assets,
    noise: &Noise,
    map: &mut AreaMap,
    pos: [i32; 2],
    offset: [i32; 2],
    scale: f32,
    damage_factor: f32,
) {
    let car_chance = fbm_offset(noise, pos, offset, 1.0, 32);
    if car_chance < 0.95 {
        return;
    }
    let color_good = Color::new(68, 68, 68);
    let color_bad = Color::new(38, 10, 8);
    let v = rand_up(fbm_offset(noise, pos, offset, 10.0, 1));
    let ref icon = VEHICLES[(v * VEHICLES.len() as f32).floor() as usize].to_string();
    let i = turb_offset(noise, pos, offset, scale, 32);
    let fg = lerp(color_good, color_bad, i * damage_factor);
    let pos = Position {
        x: pos[0],
        y: pos[1],
    };
    if let Some(tile) = map.get(pos) {
        map.set(
            pos,
            Tile::new(
                assets.get_icon(icon).base_ch(),
                fg,
                tile.bg,
                true,
                false,
                TYPE_VEHICLE,
            ),
        )
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
    assets: &Assets,
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
    let bg = Color { r: 4, g: 4, b: 4 };
    let road_rubble_fg = Color { r: 5, g: 5, b: 5 };

    let dashed = assets.get_icon("line_emdash").base_ch();
    let line = assets.get_icon("line_single").ch(false, false, true, true);
    let dbl = assets.get_icon("line_double").ch(false, false, true, true);
    let road_rubble: char = '\u{e35d}';
    let mut segment_icon: char;
    let mut fg: Color;
    let mut ground_bg = Color::new(0, 0, 0);

    for cx in 0..map.width {
        let y = road_lat(noise, map, cx, offset).floor() as i32;
        let y_min = (y - (lanes * 2)).max(0);
        let y_max = y + (lanes * 2).min(map.height);

        for cy in y_min..=y_max {
            let i = rand_up(turb_offset(noise, [cx, cy], offset, noise_scale, 32));
            let pos = Position { x: cx, y: cy };

            if i < damage_factor {
                if let Some(tile) = map.get(pos) {
                    ground_bg = tile.bg;
                }
                map.set(pos, damaged_road(ground_bg, bg, i));
                continue;
            }

            if cy == y_min || cy == y_max {
                // outer line
                segment_icon = line;
                fg = road_line_fg;
            } else if cy == y {
                // center line
                if lanes > 2 {
                    // larger roads don't have pass lanes
                    segment_icon = dbl;
                } else {
                    // smaller roads do
                    segment_icon = dashed;
                }
                fg = road_line_center;
            } else if (cy - y) % 2 == 0 {
                // place a lane divider
                segment_icon = dashed;
                fg = road_line_fg;
            } else {
                // normal road tile
                segment_icon = road_rubble;
                fg = road_rubble_fg;
            }

            // check if overlapping a horizontal road and draw correct tile
            // if so
            if let Some(tile) = map.get(pos) {
                if tile.type_id == TYPE_ROAD {
                    segment_icon = road_rubble;
                    fg = road_rubble_fg;
                }
            }

            map.set(pos, road_segment(segment_icon, fg, bg));

            place_car(
                assets,
                noise,
                map,
                [cx, cy],
                offset,
                noise_scale,
                damage_factor,
            );
        }
    }
}

/// determines the vertical offset of a horizontal road at a given x position
fn road_long(noise: &Noise, map: &AreaMap, y: i32, offset: [i32; 2]) -> f32 {
    let hw = map.width / 2;
    rand_up(fbm_offset(noise, [hw, y], offset, 0.01, 1)) * map.width as f32
}

/// generates a vertical road on the map. Damage factor is a range from 0 = pristine
/// to +1 = completely wrecked.
pub fn place_vertical_roads(
    assets: &Assets,
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
    let bg = Color { r: 4, g: 4, b: 4 };
    let road_rubble_fg = Color { r: 5, g: 5, b: 5 };

    let dashed = '|';
    let line = assets.get_icon("line_single").ch(true, true, false, false);
    let dbl = assets.get_icon("line_double").ch(true, true, false, false);

    for cy in 0..map.height {
        let x = road_long(noise, map, cy, offset).floor() as i32;
        let x_min = (x - (lanes * 2)).max(0);
        let x_max = (x + (lanes * 2)).min(map.width);
        for cx in x_min..=x_max {
            let i = rand_up(turb_offset(noise, [cx, cy], offset, noise_scale, 32));
            let pos = Position { x: cx, y: cy };
            let mut ground_bg = Color::new(0, 0, 0);
            let road_rubble: char = '\u{e35d}';
            let mut segment_icon: char;
            let mut fg: Color;

            if i < damage_factor {
                if let Some(tile) = map.get(pos) {
                    ground_bg = tile.bg;
                }
                map.set(pos, damaged_road(ground_bg, bg, i));
                continue;
            }

            if cx == x_min || cx == x_max {
                // outer line
                segment_icon = line;
                fg = road_line_fg;
            } else if cx == x {
                // center line
                if lanes > 2 {
                    // larger roads don't have pass lanes
                    segment_icon = dbl;
                } else {
                    // smaller roads do
                    segment_icon = dashed;
                }
                fg = road_line_center;
            } else if (cx - x) % 2 == 0 {
                // place a lane divider
                segment_icon = dashed;
                fg = road_line_fg;
            } else {
                // normal road tile
                segment_icon = road_rubble;
                fg = road_rubble_fg;
            }

            // check if overlapping a horizontal road and draw correct tile
            // if so
            if let Some(tile) = map.get(pos) {
                if tile.type_id == TYPE_ROAD {
                    segment_icon = road_rubble;
                    fg = road_rubble_fg;
                }
            }

            map.set(pos, road_segment(segment_icon, fg, bg));

            place_car(
                assets,
                noise,
                map,
                [cx, cy],
                offset,
                noise_scale,
                damage_factor,
            );
        }
    }
}
