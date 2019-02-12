use super::util::*;
use crate::component::{Color, Position};
use crate::resource::{tile_types::*, AreaMap, Tile};
use crate::util::colors::lerp;
use tcod::noise::Noise;

/// we'll want to get grass color in other functions maybe
pub fn grass_bg_color(
    noise: &Noise,
    pos: [i32; 2],
    offset: [i32; 2],
    noise_scale: f32,
    octaves: u32,
) -> Color {
    let color_sg_bg = Color {
        r: 42,
        g: 54,
        b: 28,
    };
    let color_tg_bg = Color {
        r: 38,
        g: 36,
        b: 21,
    };
    let i = rand_up(fbm_offset(noise, pos, offset, noise_scale, octaves));
    lerp(color_sg_bg, color_tg_bg, i)
}

pub fn grass(
    noise: &Noise,
    map: &mut AreaMap,
    width: i32,
    height: i32,
    offset: [i32; 2],
    noise_scale: f32,
) {
    let short_fg = Color {
        r: 112,
        g: 141,
        b: 64,
    };
    let tall_fg = Color {
        r: 118,
        g: 121,
        b: 72,
    };
    for x in 0..width {
        for y in 0..height {
            let i = rand_up(fbm_offset(noise, [x, y], offset, noise_scale, 32));
            let bg = grass_bg_color(noise, [x, y], offset, noise_scale, 32);
            let fg = lerp(short_fg, tall_fg, i);
            if i < 0.5 {
                map.set(
                    Position { x, y },
                    Tile::new(',', fg, bg, true, true, TYPE_GRASS),
                );
            } else {
                map.set(
                    Position { x, y },
                    Tile::new('"', fg, bg, true, true, TYPE_GRASS_LONG),
                );
            }
        }
    }
}
