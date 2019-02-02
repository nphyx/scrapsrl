mod character;
mod object;
mod entity_part;
mod behavior;
mod player;
mod npc;
pub mod body_layout;
pub use self::player::Player;
pub use self::npc::NPC;
pub use self::entity_part::EntityComponent;
pub use self::character::Character;
pub use self::object::Object;
use crate::display::DrawSelf;
use crate::game_state::GameState;
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::util::clamp;
use crate::ui::Notification;

/**
 * A coordinate.
 */

#[derive(Copy,Clone,Debug)]
pub struct Coord {
  pub x: i32,
  pub y: i32
}

impl std::ops::AddAssign<Coord> for Coord {
  fn add_assign(&mut self, coord: Coord) {
    println!("adding {:?}, {:?}", self, coord);
    self.x = clamp(0, MAP_WIDTH, self.x + coord.x);
    self.y = clamp(0, MAP_HEIGHT, self.y + coord.y);
  }
}

impl std::ops::Add<Coord> for Coord {
  type Output = Coord;
  fn add(self, coord: Coord) -> Coord {
    Coord{x: self.x + coord.x, y: self.y + coord.y}
  }
}

impl std::cmp::PartialEq for Coord {
  fn eq(&self, &cmp: &Coord) -> bool {
    return self.x == cmp.x && self.y == cmp.y;
  }
}

pub trait Entity: DrawSelf {
  fn pos(&self) -> Coord;
  fn set_pos(&mut self, pos: Coord);
  fn tick(&mut self, state: &GameState);
  fn player_interact(&mut self, player: &mut Player, state: &mut GameState) -> EntityInteraction;
}

pub type EntityCollection = Vec<Box<Entity>>;

pub enum EntityInteraction {
  Notification(Notification),
  None
}
