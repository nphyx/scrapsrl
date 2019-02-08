use tcod::console::Console;
use tcod::input::Key;
use std::sync::Arc;
use crate::game_state::GameState;

mod draw;
mod status;
mod sidebar;
mod util;

/*
pub mod draw;
pub mod notification;
pub mod widget;
pub mod chain;
// use crate::entity::{Player, EntityCollection};
pub use crate::constants::SIDEBAR_WIDTH;
pub use self::notification::Notification;
pub use self::chain::Chain;
*/

use crate::component::Character;

mod widget;
pub use self::widget::Widget;

// TODO redo UI

pub struct UI {
  stack: Vec<Arc<widget::Widget>>
}

impl UI {
  pub fn new() -> UI {
    UI{stack: Vec::new()}
  }

  pub fn draw(&mut self, console: &Console, pc: &Character, state: &GameState) {
    sidebar::draw(console, pc, state);
    status::draw(console, pc, state);
    let current_menu = self.stack.get(0);
    match current_menu {
      Some(menu) => menu.draw(&console),
      _ => {} 
    }
  }

  pub fn open_menu(&mut self, menu: impl widget::Widget + 'static) {
    self.stack.push(Arc::new(menu));
  }

  pub fn in_menu(&mut self) -> bool {
    self.stack.len() > 0
  }
}
