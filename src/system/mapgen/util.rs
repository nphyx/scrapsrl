use tcod::colors::{Color};
use crate::area_map::Tile;
use tcod::noise::Noise;

pub fn rand_up(v: f32) -> f32 { (v + 1.0) / 2.0 }

pub fn scale_axis(i:i32, offset:i32, scale:f32) -> f32 {
  ((i + offset) as f32 * scale) as f32
}

pub fn place(pos: [i32; 2], offset: [i32; 2], scale: f32) -> [f32; 2] {
  [
    scale_axis(pos[0], offset[0], scale),
    scale_axis(pos[1], offset[1], scale),
  ]
}

pub fn fbm_offset(noise: &Noise, pos: [i32; 2], offset: [i32; 2], scale: f32, octaves: u32) -> f32 {
  noise.get_fbm(place(pos, offset, scale), octaves)
}

pub fn turb_offset(noise: &Noise, pos: [i32; 2], offset: [i32; 2], scale: f32, octaves: u32) -> f32 {
  noise.get_turbulence(place(pos, offset, scale), octaves)
}

/// build a Tile, which will eventually be added to the map if it ends up in the
/// final tile set.
pub fn prep_tile<'a>(icon: char, fg: Color, bg: Color, opaque: bool, solid: bool,
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

