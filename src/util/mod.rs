use crate::component::Position;
pub mod colors;
mod connectable_char;
pub use self::connectable_char::ConnectableChars;

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
pub fn distance(p: Position, d: Position) -> f32 {
    ((d.x as f32 - p.x as f32).powf(2.0) + (d.y as f32 - p.y as f32).powf(2.0)).sqrt()
}

/// turns a -1.0..1.0 sample into a 0.0..1.0 sample
pub fn rand_up(v: f32) -> f32 {
    (v + 1.0) / 2.0
}
