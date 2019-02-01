use tcod::console::Console;
use tcod::input::Key;

pub trait Widget {
  /**
   * Returns true if still in menu, false if the menu is still in use.
   */
  fn handle_input(&mut self, keypress: Key) -> bool;
  fn draw(&self, console: &Console);
}
