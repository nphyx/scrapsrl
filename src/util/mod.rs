use crate::component::Position;
pub mod colors;
mod connectable_char;
pub use self::connectable_char::ConnectableChars;

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

pub fn distance(p: Position, d: Position) -> f32 {
    ((d.x as f32 - p.x as f32).powf(2.0) + (d.y as f32 - p.y as f32).powf(2.0)).sqrt()
}
