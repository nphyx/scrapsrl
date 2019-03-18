use crate::component::Pos;
pub mod colors;
mod connect;
mod coord;
mod grid;
mod rect;
pub use self::connect::connect_chars;
pub use self::coord::Coord;
pub use self::grid::Grid;
pub use self::rect::Rect;

/// clamps a number x between range a..b
pub fn clamp<T>(a: T, b: T, x: T) -> T
where
    T: std::cmp::PartialOrd,
{
    if x < a {
        a
    } else if x > b {
        b
    } else {
        x
    }
}

/// finds the absolute distance between two points
pub fn distance(p: Pos, d: Pos) -> f32 {
    ((d.x as f32 - p.x as f32).powf(2.0) + (d.y as f32 - p.y as f32).powf(2.0)).sqrt()
}

/// turns a -1.0..1.0 sample into a 0.0..1.0 sample
pub fn rand_up(v: f32) -> f32 {
    (v + 1.0) / 2.0
}

/// chooses an item from a vec of options given an f32 sample between 0.0 and 1.0
pub fn choose<T>(options: &[T], sample: f32) -> Option<T>
where
    T: Clone,
{
    let len = options.len();
    let i = (sample * len as f32).floor() as usize;
    match options.get(i) {
        Some(item) => Some(item.clone()),
        None => None,
    }
}

/// chooses an item from a vec of options given an f32 sample between 0.0 and 1.0
pub fn choose_ref<T>(options: &[T], sample: f32) -> Option<&T>
where
    T: Clone,
{
    let len = options.len();
    let i = (sample * len as f32).floor() as usize;
    match options.get(i) {
        Some(item) => Some(&item),
        None => None,
    }
}
