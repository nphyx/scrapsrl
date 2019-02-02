use tcod::input::Key;
use tcod::map::Map;
use tcod::input::KeyCode::*; // {NumPad7, NumPad8, NumPad9, NumPad4, NumPad6, NumPad1, NumPad2, NumPad3, NumPad5, NoKey, Shift};
use tcod::{Console};
use super::{Character, Coord, Entity, EntityCollection};
use crate::game_state::GameState;
use crate::cursor::Cursor;
use crate::util::plan;
use crate::display::DrawSelf;

pub struct Player {
  pub character: Character,
  pub cursor: Cursor,
  pub score: i32
}

impl Entity for Player {
  fn pos(&self) -> Coord {
    self.character.pos()
  }
  fn set_pos(&mut self, coord: Coord) {
    self.character.set_pos(coord);
    self.cursor.pos = self.character.pos().clone();
  }
  fn tick(&mut self, state: &GameState) {
    self.character.tick(state);
  }
}

impl DrawSelf for Player {
  fn draw(&self, console: &mut Console) {
    println!("drawing self");
    self.character.draw(console);
    if self.cursor.active {
      self.cursor.draw(console);
    }
  }
}

impl Player {
  pub fn new(mut character: Character) -> Player {
    character.set_ch('\u{e213}');
    Player{
      character,
      score: 0,
      cursor: Cursor{
        pos: Coord{x: 0, y: 0},
        active: false
      }
    }
  }

  pub fn interact(entities: &EntityCollection) {
  }

  pub fn handle_input(&mut self, keypress: &Key, map: &Map, entities: &EntityCollection) -> bool {
    let mut to = Coord{x: 0, y: 0}; // = self.character.pos();
    let mut speed = 1;
    if keypress.shift {
      speed = 2;
    }
    if keypress.code == NoKey || keypress.code == Shift {
      return false;
    } 
    match keypress {
      Key { code: NumPad7, .. } => { // up-left
        to.x = -speed;
        to.y = -speed;
      },
      Key { code: NumPad8, .. } => { // up
        to.y = -speed;
      },
      Key { code: NumPad9, .. } => { // up-right
        to.x = speed;
        to.y = -speed;
      },
      Key { code: NumPad1, .. } => { // down-left
        to.x = -speed;
        to.y = speed;
      },
      Key { code: NumPad2, .. } => { // down
        to.y = speed;
      },
      Key { code: NumPad3, .. } => { // down-right
        to.x = speed;
        to.y = speed;
      },
      Key { code: NumPad4, .. } => { // left
        to.x = -speed;
      },
      Key { code: NumPad6, .. } => { // right
        to.x = speed;
      },
      Key { code: NumPad5, .. } => { // interact
        self.cursor.active = !self.cursor.active;
      },
      _ => {}
    }

    if self.cursor.active {
      self.cursor.pos += to;
      return false;
    } else if (to.x != 0 || to.y != 0) {
      to += self.pos();
      match plan(&to, &map) {
        Some(coord) => {
          if self.character.spend_stamina(speed as u8) {
            self.character.set_pos(coord);
            self.cursor.move_to(coord);
          }
        },
        _ => {}
      }
    }
    return true;
  }
}
