use tcod::noise::Noise;
use crate::area_map::{AreaMap, Tile};
use crate::component::Position;
use crate::util::clamp;

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

pub fn fill_rect(map: &mut AreaMap, start_x: i32, start_y: i32, width: i32, height: i32, tile: Tile) {
  let min_x = clamp(0, map.width, start_x);
  let min_y = clamp(0, map.height, start_y);
  let max_x = clamp(0, map.width, start_x + width);
  let max_y = clamp(0, map.height, start_y + height);

  for x in min_x..max_x {
    for y in min_y..max_y {
      map.set(Position{x, y}, tile);
    }
  }
}
