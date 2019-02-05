/// component module
use specs::World;

mod character;
mod colors;
mod connected_tile;
mod description;
mod icon;
mod move_plan;
mod notification;
mod opaque;
mod player;
mod position;
mod solid;
mod tile;

pub use self::character::Character;
pub use self::colors::Colors;
pub use self::connected_tile::ConnectedTile;
pub use self::description::Description;
pub use self::icon::Icon;
pub use self::move_plan::MovePlan;
pub use self::opaque::Opaque;
pub use self::player::Player;
pub use self::position::Position;
pub use self::solid::Solid;
pub use self::tile::Tile;

/// initializes all components
pub fn init(world: &mut World) {
  world.register::<Character>();
  world.register::<Colors>();
  world.register::<ConnectedTile>();
  world.register::<Description>();
  world.register::<Icon>();
  world.register::<MovePlan>();
  world.register::<Player>();
  world.register::<Position>();
  world.register::<Opaque>();
  world.register::<Solid>();
  world.register::<Tile>();
}
