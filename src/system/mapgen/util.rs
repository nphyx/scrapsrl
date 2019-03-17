use super::MapGenBundle;
use crate::util::{clamp, rand_up};
use tcod::noise::Noise;

pub fn scale_axis(i: i32, offset: i32, scale: f32) -> f32 {
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

pub fn turb_offset(
    noise: &Noise,
    pos: [i32; 2],
    offset: [i32; 2],
    scale: f32,
    octaves: u32,
) -> f32 {
    noise.get_turbulence(place(pos, offset, scale), octaves)
}

/// determines the vertical offset of a horizontal road at a given x position
pub fn road_center_longitudinal(bundle: &MapGenBundle, x: usize) -> usize {
    let lanes = bundle.world.get_road(bundle.region).lanes_x;
    let pop = bundle.world.get_pop(bundle.region);
    let hh = bundle.map.height() as i32 / 2;
    let base = (rand_up(fbm_offset(
        bundle.noise,
        [x as i32, hh],
        bundle.region.to_offset(),
        0.01,
        1,
    )) * (1.0 - pop)
        * bundle.map.height() as f32) as i32;
    clamp(
        i32::from(lanes * 2),
        bundle.map.height() as i32 - i32::from(lanes * 2),
        base,
    ) as usize
}

/// determines the vertical offset of a horizontal road at a given x position
pub fn road_center_latitudinal(bundle: &MapGenBundle, y: usize) -> usize {
    let lanes = bundle.world.get_road(bundle.region).lanes_y;
    let pop = bundle.world.get_pop(bundle.region);
    let hw = bundle.map.width() as i32 / 2;
    let base = (rand_up(fbm_offset(
        bundle.noise,
        [hw, y as i32],
        bundle.region.to_offset(),
        0.01,
        1,
    )) * (1.0 - pop)
        * bundle.map.width() as f32) as i32;
    clamp(
        i32::from(lanes * 2),
        bundle.map.width() as i32 - i32::from(lanes * 2),
        base,
    ) as usize
}
