use std::sync::Arc;

mod draw;
mod status;
mod sidebar;
mod util;
mod widget;

pub use self::widget::Widget;
pub use sidebar::*;
pub use status::*;

pub struct Menus {
  stack: Vec<Arc<widget::Widget>>
}

impl Menus {
  pub fn new() -> Menus {
    Menus{stack: Vec::new()}
  }

  pub fn open_menu(&mut self, menu: impl widget::Widget + 'static) {
    self.stack.push(Arc::new(menu));
  }

  pub fn in_menu(&mut self) -> bool {
    self.stack.len() > 0
  }
}
