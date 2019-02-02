use tcod::{Console, BackgroundFlag};
use tcod::colors::Color;
use super::{Entity, EntityInteraction, Coord, Player};
use crate::display::DrawSelf;
use crate::game_state::GameState;
use crate::ui::Notification;
use crate::util::distance;

pub struct Object {
  pos: Coord,
  ch: char,
  color: Color,
  interact_notification: Option<Notification>
}

impl Object {
  pub fn new() -> Object {
    return Object{
      pos: Coord{x: 0, y: 0},
      ch: '!',
      color: Color{r: 128, g: 128, b: 128},
      interact_notification: None
    }
  }

  pub fn set_ch(&mut self, ch: char) {
    self.ch = ch;
  }

  pub fn set_notification(&mut self, notif: Notification) {
    self.interact_notification = Some(notif);
  }

}

impl Entity for Object {
  fn pos(&self) -> Coord { self.pos }
  fn set_pos(&mut self, pos: Coord) { self.pos = pos }
  fn tick(&mut self, _state: &GameState) {}
  fn player_interact(&mut self, player: &mut Player, state: &mut GameState) -> EntityInteraction {
    if distance(player.pos(), self.pos()) < 2.0 {
      match &self.interact_notification {
        Some(notif) => return EntityInteraction::Notification(notif.clone()),
        None => {}
      }
    }
    EntityInteraction::None
  }
}

impl DrawSelf for Object {
  fn draw(&self, console: &mut Console) {
    console.put_char(self.pos().x, self.pos().y, self.ch, BackgroundFlag::None);
    console.set_char_foreground(self.pos().x, self.pos().y, self.color);
  }
}
