/* TODO reimplement parts & behaviors
// mod entity_part;
// mod behavior;
// mod player;
mod npc;
mod object;
mod tile;
// pub mod body_layout;
// pub use self::player::make_player_entity;
//pub use self::npc::NPC;
pub use self::object::build_object_entity;
// pub use self::entity_part::EntityComponent;
// pub use self::npc::make_npc_entity;
*/

/*
use crate::display::DrawSelf;
use crate::game_state::GameState;
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::util::clamp;
use crate::ui::Notification;

pub trait Entity: DrawSelf {
  fn pos(&self) -> Coord;
  fn set_pos(&mut self, pos: Coord);
  fn tick(&mut self, state: &GameState);
  fn player_interact(&mut self, player: &mut Player, state: &mut GameState) -> EntityInteraction;
  fn desc(&self) -> String;
  fn entity_type(&self) -> EntityType;
}

pub type EntityCollection = Vec<Box<Entity>>;

pub enum EntityInteraction {
  Notification(Notification),
  None
}
*/
