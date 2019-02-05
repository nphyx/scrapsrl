/*
use specs::{Builder, EntityBuilder};
use tcod::colors::Color;
// use crate::display::DrawSelf;
use crate::component::*;

pub fn build_object_entity<'a>(builder: &EntityBuilder<'a>, ch: char, desc: String) -> &'a EntityBuilder<'a> {
  let ret = &mut builder
    .with(Position{x:0, y:0})
    .with(Icon{ch})
    .with(Colors{
      fg: Color{r: 128, g: 128, b:128},
      bg: Color{r: 128, g: 128, b:128}
    })
    .with(Description{short: desc.clone(), long: desc.clone()});
}
*/

/*
impl Object {
  pub fn new() -> Object {
    return Object{
      pos: Coord{x: 0, y: 0},
      ch: '!',
      color: Color{r: 128, g: 128, b: 128},
      interact_notification: None,
      desc: "".to_string()
    }
  }

  pub fn set_ch(&mut self, ch: char) {
    self.ch = ch;
  }

  pub fn set_notification(&mut self, notif: Notification) {
    self.interact_notification = Some(notif);
  }

  pub fn set_desc(&mut self, desc: String) {
    self.desc = desc;
  }
}

impl Entity for Object {
  fn pos(&self) -> Coord { self.pos }
  fn set_pos(&mut self, pos: Coord) { self.pos = pos }
  fn tick(&mut self, _state: &GameState) {}
  fn player_interact(&mut self, player: &mut Player, state: &mut GameState) -> EntityInteraction {
    /*
    if distance(player.pos(), self.pos()) < 2.0 {
      match &self.interact_notification {
        Some(notif) => return EntityInteraction::Notification(notif.clone()),
        None => {}
      }
    }
    */
    EntityInteraction::None
  }
  fn desc(&self) -> String {
    self.desc.to_string()
  }
  fn entity_type(&self) -> EntityType {
    EntityType::Object
  }
}

impl DrawSelf for Object {
  fn draw(&self, console: &mut Console) {
    console.put_char(self.pos().x, self.pos().y, self.ch, BackgroundFlag::None);
    console.set_char_foreground(self.pos().x, self.pos().y, self.color);
  }
  fn draw_at(&self, console: &mut Console, x: i32, y:i32) {
    console.put_char(x, y, self.ch, BackgroundFlag::None);
    console.set_char_foreground(x, y, self.color);
  }
}
*/
