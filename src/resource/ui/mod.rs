use std::sync::{Arc, Mutex};
use tcod::input::{Key}; //, KeyCode::*};
use specs::{Component, VecStorage};

mod notification;
mod types;
pub use notification::*;
use types::*;

pub enum UIResponse {
  /* Consumed, FIXME UNUSED */
  Completed,
  Unrecognized
}

pub struct UIQueue {
  stack: Vec<Arc<Mutex<dyn Widget>>>
}

impl Component for UIQueue {
  type Storage = VecStorage<UIQueue>;
}

impl Default for UIQueue {
  fn default() -> UIQueue {
    UIQueue{stack: Vec::new()}
  }
}

impl UIQueue {
  /* FIXME unused 
  pub fn new() -> UIQueue {
    UIQueue{stack: Vec::new()}
  }
  */

  pub fn add(&mut self, widget: impl Widget + 'static) {
    self.stack.push(Arc::new(Mutex::new(widget)));
  }

  pub fn len(&self) -> usize {
    self.stack.len()
  }

  pub fn get(&self) -> Option<&Arc<Mutex<dyn Widget>>> {
    self.stack.get(0)
  }

  pub fn next(&mut self, input: Key) -> UIResponse {
    let response: UIResponse;
    {
      let top = &mut self.stack[0].lock().unwrap();
      response = top.next(input);
    }
    if let UIResponse::Completed = response {
      self.stack.remove(0);
    }
    return response
  }
}

pub trait Widget: Send + Sync { 
  /// gets the UIElementType for the widget
  fn get_type(&self) -> UIElementType;
  /// gets the title string for the widget
  fn get_title(&self) -> String;
  /// gets the body string for the widget
  fn get_body(&self) -> String;
  /// gets the footer string for the widget
  fn get_footer(&self) -> String;
  /// asks whether the widget is done with its task
  fn done(&self) -> bool;
  /// Passes user input to the widget for consumption and returns true if it still has
  /// stuff to do
  fn next(&mut self, input: Key) -> UIResponse;
}
