use tcod::console::Console;
use tcod::input::Key;
use crate::game_state::GameState;
pub mod draw;
pub mod notification;
pub mod widget;
use crate::ui::draw::{draw_status_bar, draw_sidebar};
pub use crate::constants::SIDEBAR_WIDTH;
pub use crate::ui::notification::Notification;

pub struct UI {
    stack: Vec<Box<widget::Widget>>
}

impl UI {
    pub fn new() -> UI {
        UI{stack: Vec::new()}
    }

    pub fn draw(&mut self, console: &Console, state: &GameState) {
        if self.in_menu() {
            draw_status_bar(&console, format!("-- PAUSED -- SCORE {:?}", state.score));
        } else {
            draw_status_bar(&console, format!("STATUS STATUS STATUS SCORE {:?}", state.score));
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
