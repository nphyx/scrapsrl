use tcod::input::Key;
use tcod::map::Map;
use tcod::input::KeyCode::*; // {NumPad7, NumPad8, NumPad9, NumPad4, NumPad6, NumPad1, NumPad2, NumPad3, NumPad5, NoKey, Shift};
use tcod::{Console};
use super::*;
use crate::game_state::GameState;
use crate::cursor::Cursor;
use crate::util::plan;
use crate::display::DrawSelf;

pub struct Player {
  pub character: Character,
  pub cursor: Cursor,
  pub wants_interact_at: Option<Coord>,
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
  fn player_interact(&mut self, _player: &mut Player, _state: &mut GameState) -> EntityInteraction {
    EntityInteraction::None
  }
  fn desc(&self) -> String {
    self.character.desc()
  }
  fn entity_type(&self) -> EntityType {
    return EntityType::Player
  }
}

impl DrawSelf for Player {
  fn draw(&self, console: &mut Console) {
    self.character.draw(console);
    if self.cursor.active {
      self.cursor.draw(console);
    }
  }
  fn draw_at(&self, console: &mut Console, x: i32, y:i32) {
    self.character.draw_at(console, x, y);
  }
}

impl Player {
  pub fn new(mut character: Character) -> Player {
    character.set_ch('\u{e213}');
    Player{
      character,
      score: 0,
      wants_interact_at: None,
      cursor: Cursor{
        pos: Coord{x: 0, y: 0},
        active: false
      },
    }
  }

  /// Called when input is delegated to the player character. Returns true if the
  /// game state should advance and/or system level keys should be permitted.
  pub fn handle_input(&mut self, keypress: &Key, map: &Map, entities: &EntityCollection) -> bool {
    self.wants_interact_at = None;
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
      Key { code: Escape, .. } => {
        if self.cursor.active {
          self.cursor.active = false; 
          return false;
        }
      },
      Key { code: Enter, .. } |
      Key { code: NumPadEnter, ..} => {
        if self.cursor.active {
          self.wants_interact_at = Some(self.cursor.pos.clone());
          self.cursor.active = false;
        }
        else {
          self.cursor.active = true;
        }
      }
      _ => {}
    }

    if self.cursor.active {
      self.cursor.pos += to;
      return false;
    } else if to.x != 0 || to.y != 0 {
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
