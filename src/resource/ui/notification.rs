use super::Widget;
use super::{UIElementType, UIResponse};
use tcod::input::{Key, KeyCode::*};

#[derive(Clone)]
pub struct Notification {
    title: String,
    body: String,
    done: bool,
}

impl Notification {
    pub fn new(title: String, body: String) -> Notification {
        Notification {
            title,
            body,
            done: false,
        }
    }
}

impl Widget for Notification {
    fn get_type(&self) -> UIElementType {
        UIElementType::Notification
    }
    fn get_title(&self) -> String {
        self.title.clone()
    }
    fn get_body(&self) -> String {
        self.body.clone()
    }
    fn get_footer(&self) -> String {
        "[Esc] Close".to_string()
    }
    fn next(&mut self, input: Key) -> UIResponse {
        match input {
            Key { code: Escape, .. } => UIResponse::Completed,
            _ => UIResponse::Unrecognized,
        }
    }
    fn done(&self) -> bool {
        self.done
    }
}
