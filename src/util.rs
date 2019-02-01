use crate::entity::Coord;
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};


pub fn clamp<T>(a: T, b: T, x: T) -> T  where T: std::cmp::PartialOrd {
    if x < a { a } else if x > b { b } else { x }
}

pub fn distance(px: f32, py: f32, dx: f32, dy: f32) -> f32 {
    return ((dx - px).powf(2.0) + (dy - py).powf(2.0)).sqrt()
}

pub fn plan(&to: &Coord, map: &tcod::map::Map) -> Option<Coord> {
    let planned = Coord{
        x: clamp(0, MAP_WIDTH - 1, to.x),
        y: clamp(0, MAP_HEIGHT - 1, to.y)
    };
    if map.is_walkable(planned.x, planned.y) {
        return Some(planned)
    }
    None
}
