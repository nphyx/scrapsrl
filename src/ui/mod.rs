use tcod::console::Console;
use tcod::input::Key;
use crate::game_state::GameState;
pub mod draw;
pub mod notification;
pub mod widget;
use crate::ui::draw::{draw_status_bar, draw_sidebar};
pub use crate::constants::SIDEBAR_WIDTH;
pub use crate::ui::notification::Notification;

fn meter_bar(max: u8, cur: u8,  cap: u8) -> String {
  let gap: u8 = max - cur - cap;
  return format!("[{:\u{2588}<3$}{:\u{2592}<4$}{:\u{2593}<5$}]", "", "", "", cur as usize, gap as usize, cap as usize);
}

pub struct UI {
  stack: Vec<Box<widget::Widget>>
}

impl UI {
  pub fn new() -> UI {
    UI{stack: Vec::new()}
  }

  pub fn draw(&mut self, console: &Console, state: &GameState) {
    let pc = &state.player.character;
    if self.in_menu() {
      draw_status_bar(&console, format!("-- PAUSED -- SCORE {:?}", state.score));
    } else {
      let stamina: (u8, u8, u8) = pc.stamina();
      let focus: (u8, u8, u8) = pc.focus();
      let grit: (u8, u8, u8) = pc.grit();
      draw_status_bar(&console,
        format!(
          concat!(
            "stamina: {} ",
            "focus: {} ",
            "grit: {} ",
            "SCORE {}"),
            meter_bar(stamina.0, stamina.1, stamina.2),
            meter_bar(focus.0, focus.1, focus.2),
            meter_bar(grit.0, grit.1, grit.2),
            state.score));
    }
    draw_sidebar(&console, &state);
    let current_menu = self.stack.get(0);
    match current_menu {
      Some(menu) => menu.draw(&console),
      _ => {} 
    }
  }

  pub fn handle_input(&mut self, keypress: Key, _state: &mut GameState) -> bool {
    let current_menu = self.stack.get(0);
    match current_menu {
      Some(menu) => {
        if !menu.handle_input(keypress) {
          self.stack.pop();
        }
        return true
      }
      _ => return false
    }
  }

  pub fn open_menu(&mut self, menu: impl widget::Widget + 'static) {
    self.stack.push(Box::new(menu));
  }

  pub fn in_menu(&mut self) -> bool {
    self.stack.len() > 0
  }
}
