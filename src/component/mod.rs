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
mod solid;
mod user_input; 

pub use self::ai_brain::AIBrain;
pub use self::character::Character;
pub use self::colors::Colors;
pub use self::cursor::Cursor;
pub use self::description::Description;
pub use self::icon::Icon;
pub use self::move_plan::MovePlan;
pub use self::opaque::Opaque;
pub use self::player::Player;
pub use self::position::Position;
pub use self::solid::Solid;
pub use self::user_input::UserInput;

/// initializes all components
pub fn init(world: &mut World) {
  world.register::<AIBrain>();
  world.register::<Character>();
  world.register::<Colors>();
  world.register::<Cursor>();
  world.register::<Description>();
  world.register::<Icon>();
  world.register::<MovePlan>();
  world.register::<Player>();
  world.register::<Position>();
  world.register::<Opaque>();
  world.register::<Solid>();
  world.register::<UserInput>();
}
