use crate::component::Region;
use crate::resource::{AreaMap, WorldState};
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

/*
 * DEPRECATED replace with Rect::iter when needed
pub fn fill_rect(
    map: &mut AreaMap,
    start_x: i32,
    start_y: i32,
    width: i32,
    height: i32,
    tile: Tile,
) {
    let min_x = clamp(0, map.width, start_x);
    let min_y = clamp(0, map.height, start_y);
    let max_x = clamp(0, map.width, start_x + width);
    let max_y = clamp(0, map.height, start_y + height);

    for x in min_x..max_x {
        for y in min_y..max_y {
            map.set(Pos { x, y }, tile.clone());
        }
    }
}
*/

/// determines the vertical offset of a horizontal road at a given x position
pub fn road_center_longitudinal(
    noise: &Noise,
    world: &WorldState,
    map: &AreaMap,
    region: Region,
    x: usize,
) -> usize {
    let lanes = world.get_road(region).lanes_x;
    let pop = world.get_pop(region);
    let hh = map.height() as i32 / 2;
    let base = (rand_up(fbm_offset(
        noise,
        [x as i32, hh],
        region.to_offset(),
        0.01,
        1,
    )) * (1.0 - pop)
        * map.height() as f32) as i32;
    clamp(
        i32::from(lanes * 2),
        map.height() as i32 - i32::from(lanes * 2),
        base,
    ) as usize
}

/// determines the vertical offset of a horizontal road at a given x position
pub fn road_center_latitudinal(
    noise: &Noise,
    world: &WorldState,
    map: &AreaMap,
    region: Region,
    y: usize,
) -> usize {
    let lanes = world.get_road(region).lanes_y;
    let pop = world.get_pop(region);
    let hw = map.width() as i32 / 2;
    let base = (rand_up(fbm_offset(
        noise,
        [hw, y as i32],
        region.to_offset(),
        0.01,
        1,
    )) * (1.0 - pop)
        * map.width() as f32) as i32;
    clamp(
        i32::from(lanes * 2),
        map.width() as i32 - i32::from(lanes * 2),
        base,
    ) as usize
}
