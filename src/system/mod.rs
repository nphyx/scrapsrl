use specs::{System, ReadStorage};
use crate::component::{Icon, Description};
use crate::mapgen::Tiles;

mod describe;
mod connect_tiles;
mod draw_icon;
pub use describe::Describe;
pub use connect_tiles::ConnectTiles;
pub use draw_icon::DrawIcon;
