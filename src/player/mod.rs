use tcod::input::Key;
use tcod::map::Map;
use tcod::input::KeyCode::{NumPad7, NumPad8, NumPad9, NumPad4, NumPad6, NumPad1, NumPad2, NumPad3, NoKey, Shift};
use super::entity::{Character, Coord, Entity};
use crate::util::plan;

pub struct Player {
  pub character: Character
}

impl Player {
  pub fn new(mut character: Character) -> Player {
    character.set_ch('\u{e213}');
    Player{character}
  }

  // TEMP passthrough for character.pos
  pub fn pos(&self) -> Coord {
    self.character.pos()
  }

  // TEMP passthrough for character.set_pos
  pub fn set_pos(&mut self, coord: Coord) {
    self.character.set_pos(coord)
  }

  // passthrough for character tick
  pub fn tick(&mut self) {
    self.character.tick()
  }

  pub fn handle_input(&mut self, keypress: &Key, map: &Map) -> bool {
    let mut to: Coord = self.character.pos();
    let mut speed = 1;
    if keypress.shift {
      speed = 2;
    }
    if keypress.code == NoKey || keypress.code == Shift {
      return false;
    } 
    match keypress {
      Key { code: NumPad7, .. } => { // up-left
        to.x = self.character.pos().x - speed;
        to.y = self.character.pos().y - speed;
      },
      Key { code: NumPad8, .. } => { // up
        to.y = self.character.pos().y - speed;
      },
      Key { code: NumPad9, .. } => { // up-right
        to.x = self.character.pos().x + speed;
        to.y = self.character.pos().y - speed;
      },
      Key { code: NumPad1, .. } => { // down-left
        to.x = self.character.pos().x - speed;
        to.y = self.character.pos().y + speed;
      },
      Key { code: NumPad2, .. } => { // down
        to.y = self.character.pos().y + speed;
      },
      Key { code: NumPad3, .. } => { // down-right
        to.x = self.character.pos().x + speed;
        to.y = self.character.pos().y + speed;
      },
      Key { code: NumPad4, .. } => { // left
        to.x = self.character.pos().x - speed;
      },
      Key { code: NumPad6, .. } => { // right
        to.x = self.character.pos().x + speed; 
      },
      _ => {}
    }

    if to != self.pos() {
      match plan(&to, &map) {
        Some(coord) => {
          if self.character.spend_stamina(speed as u8) {
            self.character.set_pos(coord);
          }
        },
        _ => {}
      }
    }
    return true;
  }
}
