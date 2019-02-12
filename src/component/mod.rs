/// component module
use specs::World;

pub mod ai_brain;
mod character;
mod colors;
mod cursor;
mod description;
mod icon;
mod move_plan;
mod notification;
mod opaque;
mod player;
mod position;
mod region;
mod solid;

pub use self::ai_brain::AIBrain;
pub use self::character::Character;
pub use self::colors::*;
pub use self::cursor::Cursor;
pub use self::description::Description;
pub use self::icon::Icon;
pub use self::move_plan::MovePlan;
pub use self::notification::NotificationInteraction;
pub use self::opaque::Opaque;
pub use self::player::Player;
pub use self::position::Position;
pub use self::region::Region;
pub use self::solid::Solid;

/// initializes all components
pub fn init(world: &mut World) {
  world.register::<AIBrain>();
  world.register::<Character>();
  world.register::<Colors>();
  world.register::<Cursor>();
  world.register::<Description>();
  world.register::<Icon>();
  world.register::<MovePlan>();
  world.register::<NotificationInteraction>();
  world.register::<Player>();
  world.register::<Position>();
  world.register::<Opaque>();
  world.register::<Region>();
  world.register::<Solid>();
}
