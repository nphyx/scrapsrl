use tcod::input::KeyCode::{NumPadEnter, Enter, Escape};
use tcod::input::Key;
use tcod::console::Console;
use crate::ui::widget::Widget;
use crate::ui::draw::{draw_centered_dialog};

#[derive(Clone)]
pub struct Notification {
  title: String,
  body: String
}

impl Notification {
  pub fn new(title: String, body: String) -> Notification {
    Notification{title, body}
  }
}

impl Widget for Notification {
  fn handle_input(&mut self, keypress: Key) -> bool {
    match keypress {
      Key { code: Enter, .. } |
        Key { code: NumPadEnter, .. } => {
          return false;
        },
        Key { code: Escape, .. } => {
          return false;
        },
        _ => { return true; }
    }
  }

  fn draw(&self, console: &Console) {
    draw_centered_dialog(
      console,
      &self.title,
      &self.body,
      &format!("[Enter] Ok")
    );
  }
}
