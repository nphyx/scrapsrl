use super::{util::*, MapGenBundle};
use crate::component::{Color, Description, Pos};
use crate::resource::Tile;
use crate::util::colors::lerp;
use crate::util::*;

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
    bundle: &mut MapGenBundle,
    pos: Pos,
    offset: [i32; 2],
    scale: f32,
    damage_factor: f32,
) {
    let car_chance = fbm_offset(bundle.noise, pos.to_array(), offset, 1.0, 32);
    if car_chance < 0.95 {
        return;
    }
    let default_bg = Color::new(4, 4, 4);
    let color_good = Color::new(68, 68, 68);
    let color_bad = Color::new(38, 10, 8);
    let v = rand_up(fbm_offset(bundle.noise, pos.to_array(), offset, 10.0, 1));
    let icon = &VEHICLES[(v * VEHICLES.len() as f32).floor() as usize].to_string();
    let i = turb_offset(bundle.noise, pos.to_array(), offset, scale, 32);
    let fg = lerp(color_good, color_bad, i * damage_factor);
    bundle
        .map
        .try_set(
            pos,
            Tile::new(
                bundle.assets.get_icon(icon).ch(),
                fg,
                bundle.map.get(pos).map_or(default_bg, |t| t.bg),
                true,
                false,
                true,
                Description::new("vehicle", "the rusted hulk of an old automobile"),
            ),
        )
        .ok();
}

fn damaged_road(icon: char, road_bg: Color, ground_bg: Color, blend_factor: f32) -> Tile {
    let fg = lerp(ground_bg, road_bg, blend_factor * 0.5);
    Tile::new(
        icon,
        fg,
        road_bg,
        true,
        true,
        true,
        Description::new("road", "Little remains of this crumbled asphalt."),
    )
}

/// creates a single segment of road
fn road_segment(icon: char, fg: Color, bg: Color) -> Tile {
    Tile::new(
        icon,
        fg,
        bg,
        true,
        true,
        true,
        Description::new("road", "A crumbling old road."),
    )
}

/// generates a horizontal road on the map. Damage factor is a range from 0 = pristine
/// to +1 = completely wrecked.
pub fn place_horizontal_roads(bundle: &mut MapGenBundle, noise_scale: f32, damage_factor: f32) {
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
    let default_bg = Color { r: 4, g: 4, b: 4 };
    let bg = default_bg;
    let road_rubble_fg = Color { r: 5, g: 5, b: 5 };

    let dashed = bundle.assets.get_icon("road_line_dashed").ch();
    let line = bundle.assets.get_icon("road_line_single").ch();
    let dbl = bundle.assets.get_icon("road_line_double").ch();
    let lanes = bundle.world.get_road(bundle.region).lanes_x as usize;
    let offset = bundle.region.to_offset();

    let road_rubble = bundle.assets.get_icon("floor_racked").ch();
    let mut segment_icon: char;
    let mut fg: Color;
    let mut ground_bg: Color;
    let damage_icon = bundle.assets.get_icon("road_cracked").ch();

    for cx in 0..bundle.map.width() {
        let y = road_center_longitudinal(bundle, cx);
        let y_min = y - (lanes * 2); // *2 because two tiles per lane
        let y_max = y + (lanes * 2);

        for cy in y_min..=y_max {
            let i = rand_up(turb_offset(
                bundle.noise,
                [cx as i32, cy as i32],
                offset,
                noise_scale,
                32,
            ));
            let pos = Pos { x: cx, y: cy };

            if i < damage_factor {
                ground_bg = bundle.map.get(pos).map_or(default_bg, |t| t.bg);
                bundle
                    .map
                    .try_set(pos, damaged_road(damage_icon, bg, ground_bg, i))
                    .ok();
                continue;
            }

            if cy == y_min || cy == y_max {
                // outer line
                segment_icon = line;
                fg = road_line_fg;
            } else if cy == y {
                // center line
                if lanes > 1 {
                    // larger roads don't have pass lanes
                    segment_icon = dbl;
                } else {
                    // smaller roads do
                    segment_icon = dashed;
                }
                fg = road_line_center;
            } else if (cy as i32 - y as i32) % 2 == 0 {
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
            if bundle.map.get(pos).map_or(false, |t| t.constructed) {
                segment_icon = road_rubble;
                fg = road_rubble_fg;
            }

            bundle
                .map
                .try_set(pos, road_segment(segment_icon, fg, bg))
                .ok();

            place_car(bundle, pos, offset, noise_scale, damage_factor);
        }
    }
}

/// generates a vertical road on the map. Damage factor is a range from 0 = pristine
/// to +1 = completely wrecked.
pub fn place_vertical_roads(bundle: &mut MapGenBundle, noise_scale: f32, damage_factor: f32) {
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
    let default_bg = Color { r: 4, g: 4, b: 4 };
    let bg = default_bg;
    let road_rubble_fg = Color { r: 5, g: 5, b: 5 };

    let dashed = bundle.assets.get_icon("road_line_dashed").ch();
    let line = bundle.assets.get_icon("road_line_single").ch();
    let dbl = bundle.assets.get_icon("road_line_double").ch();
    let damage_icon = bundle.assets.get_icon("road_cracked").ch();
    let lanes = bundle.world.get_road(bundle.region).lanes_y as usize;
    let offset = bundle.region.to_offset();

    let road_rubble = bundle.assets.get_icon("floor_racked").ch();
    let mut ground_bg: Color;
    let mut segment_icon: char;
    let mut fg: Color;

    for cy in 0..bundle.map.height() {
        let x = road_center_latitudinal(bundle, cy);
        let x_min = x - (lanes * 2);
        let x_max = x + (lanes * 2);
        for cx in x_min..=x_max {
            let pos: Pos = Coord { x: cx, y: cy };
            let i = rand_up(turb_offset(
                bundle.noise,
                pos.to_array(),
                offset,
                noise_scale,
                32,
            ));

            if i < damage_factor {
                ground_bg = bundle.map.get(pos).map_or(default_bg, |t| t.bg);
                bundle
                    .map
                    .try_set(pos, damaged_road(damage_icon, bg, ground_bg, i))
                    .ok();
                continue;
            }

            if cx == x_min || cx == x_max {
                // outer line
                segment_icon = line;
                fg = road_line_fg;
            } else if cx == x {
                // center line
                if lanes > 1 {
                    // larger roads don't have pass lanes
                    segment_icon = dbl;
                } else {
                    // smaller roads do
                    segment_icon = dashed;
                }
                fg = road_line_center;
            } else if (cx as i32 - x as i32) % 2 == 0 {
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
            if bundle.map.get(pos).map_or(false, |t| t.constructed) {
                segment_icon = road_rubble;
                fg = road_rubble_fg;
            }

            bundle
                .map
                .try_set(pos, road_segment(segment_icon, fg, bg))
                .ok();

            place_car(bundle, pos, offset, noise_scale, damage_factor);
        }
    }
}
