use tcod::console::Console;
use tcod::input::Key;
use crate::ui::widget::Widget;

pub struct Chain {
  stack: Vec<Box<Widget>>
}

impl Chain {
  pub fn new(stack: Vec<Box<Widget>>) -> Chain {
    return Chain{stack};
  }
}

impl Widget for Chain {
  fn handle_input(&mut self, keypress: Key) -> bool {
    let current_item = self.stack.get_mut(0);
    match current_item {
      Some(item) => {
        if !item.handle_input(keypress) {
          self.stack.remove(0);
        }
      }
      _ => return false
    }
    if self.stack.len() > 0 { return true }
    return false
  }

  fn draw(&self, console: &Console) {
    let current_item = self.stack.get(0);
    match current_item {
      Some(item) => item.draw(&console),
      _ => {} 
    }
  }
}
