use tcod::{Console, BackgroundFlag};
// use rand::prelude::*;
use super::*;
use crate::display::DrawSelf;
use crate::game_state::GameState;
use crate::ui::Notification;
// use crate::util::distance;

pub struct NPC {
  pub character: Character,
  pub behavior: &'static behavior::Behavior,
  interact_notification: Option<Notification>,
  target: Option<&'static Entity>
}

impl NPC {
  pub fn new(character: Character) -> NPC {
    NPC{
      character,
      target: None,
      interact_notification: None,
      behavior: &behavior::MovementBehavior::BrownianWalk
    }
  }

  pub fn set_notification(&mut self, notif: Notification) {
    self.interact_notification = Some(notif);
  }
}

impl Entity for NPC {
  fn pos(&self) -> Coord {
    self.character.pos()
  }
  fn set_pos(&mut self, coord: Coord) {
    self.character.set_pos(coord)
  }
  fn tick(&mut self, state: &GameState) {
    /*
    self.behavior.execute(self, state);
    self.character.tick(state);
    */
  }
  fn player_interact(&mut self, player: &mut Player, state: &mut GameState) -> EntityInteraction {
    /*
    if distance(player.pos(), self.pos()) < 2.0 {
      let mut rng = rand::thread_rng();
      player.score += 1;
      self.set_pos(Coord{
        x: rng.gen_range(0, MAP_WIDTH),
        y: rng.gen_range(0, MAP_HEIGHT)
      });
      match &self.interact_notification {
        Some(notif) => return EntityInteraction::Notification(notif.clone()),
        None => {}
      }
    }
    */
    EntityInteraction::None
  }
  fn desc(&self) -> String {
    self.character.desc()
  }
  fn entity_type(&self) -> EntityType {
    EntityType::NPC
  }
}

impl DrawSelf for NPC {
  fn draw(&self, console: &mut Console) {
    self.character.draw(console);
  }
  fn draw_at(&self, console: &mut Console, x: i32, y: i32) {
    console.put_char(x, y, self.character.ch, BackgroundFlag::None);
    console.set_char_foreground(x, y, self.character.color);
  }
}
