use tcod::console::Console;
use tcod::input::Key;
use crate::game_state::GameState;
pub mod draw;
pub mod notification;
pub mod widget;
pub mod chain;
use crate::entity::{Player, EntityCollection};
use crate::ui::draw::{draw_status_bar, draw_sidebar};
pub use crate::constants::SIDEBAR_WIDTH;
pub use self::widget::Widget;
pub use self::notification::Notification;
pub use self::chain::Chain;

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

  pub fn draw(&mut self, console: &Console, player: &Player, state: &GameState, entities: &EntityCollection) {
    let pc = &player.character;
    if self.in_menu() {
      draw_status_bar(console, format!("-- PAUSED -- SCORE {:?}", player.score));
    } else if player.cursor.active {
      draw_status_bar(console, "-- LOOKING --   [enter] interact   [Num5] cancel".to_string());
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
            player.score));
    }
    draw_sidebar(console, player, state, entities);
    let current_menu = self.stack.get(0);
    match current_menu {
      Some(menu) => menu.draw(&console),
      _ => {} 
    }
  }

  pub fn handle_input(&mut self, keypress: Key, _state: &mut GameState) -> bool {
    let current_menu = self.stack.get_mut(0);
    match current_menu {
      Some(menu) => {
        if !menu.handle_input(keypress) {
          self.stack.remove(0);
        }
      }
      _ => return false
    }
    return true
  }

  pub fn open_menu(&mut self, menu: impl widget::Widget + 'static) {
    self.stack.push(Box::new(menu));
  }

  pub fn in_menu(&mut self) -> bool {
    self.stack.len() > 0
  }
}
