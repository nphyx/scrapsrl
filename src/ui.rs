use tcod::console::{Console, TextAlignment, BackgroundFlag};
use tcod::input::Key;
use tcod::colors::Color;
use tcod::input::KeyCode::{Escape};
use crate::game_state::GameState;

pub const SIDEBAR_WIDTH: i32 = 20;

#[derive(Clone)]
pub enum MenuType {
    Dialog,
    CenteredDialog,
    Sidebar,
    Status
}

const TEXT_COLOR: Color = Color{r: 255, g: 255, b: 255};
const DIALOG_BG: Color = Color{r: 0, g: 0, b: 0};

fn reset_colors(mut console: &Console) {
    console.set_default_foreground(TEXT_COLOR);
    console.set_default_background(DIALOG_BG);
}

fn draw_dialog(mut console: &Console, cx: i32, cy:i32, text: &String, title: &String) {
    reset_colors(&console);
    console.set_alignment(TextAlignment::Center);
    console.rect(cx - 12, cy - 1, 24, 4, true, BackgroundFlag::Set);
    console.print(cx, cy - 1, title);
    console.print(cx, cy, text);
}

fn draw_centered_dialog(console: &Console, title: &String, text: &String) {
    let cx = console.width() / 2;
    let cy = console.height() / 2;
    draw_dialog(console, cx, cy, text, title);
}

fn draw_status_bar(mut console: &Console, text: String) {
    reset_colors(&console);
    console.set_alignment(TextAlignment::Left);
    let x = 0;
    let y = console.height() - 1;
    let width = console.width();
    let height = 1;
    console.print_rect(x, y, width, height, text);
}

fn draw_sidebar(mut console: &Console, state: &GameState) {
    reset_colors(&console);
    console.set_alignment(TextAlignment::Left);
    let x = console.width() - SIDEBAR_WIDTH;
    let y = 0;
    let width = SIDEBAR_WIDTH;
    let height = console.height();
    let text = concat!(
        "- THIS IS SIDEBAR -\n",
        "\n",
        "STATUS: 0\n",
        "STATUS: 0\n",
        "STATUS: 0\n",
        "STATUS: 0\n",
        "-- INVENTORY --\n",
        "THING ONE (1)\n",
        "THING TWO (2)\n");

    console.print_rect(x, y, width, height, text);
}

pub struct Menu {
    title: String,
    text: String,
    menu_type: MenuType
}

impl Menu {
    pub fn new(title: String, text: String, menu_type: MenuType) -> Menu {
        Menu{title, text, menu_type}
    }

    /**
     * Returns true if still in menu, false if the menu is still in use.
     */
    pub fn handle_input(&self, keypress: Key) -> bool {
        match keypress {
            Key { code: Escape, .. } => {
                return false;
            },
            _ => { return true; }
        }
    }

    pub fn draw(&self, console: &Console) {
        match self.menu_type {
            MenuType::CenteredDialog => draw_centered_dialog(
                console,
                &self.title,
                &self.text
            ),
            _ => {} // TODO Implement all types
        }
    }
}

pub struct UI {
    stack: Vec<Menu>
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

    pub fn handle_input(&mut self, keypress: Key, state: &mut GameState) -> bool {
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

    pub fn open_menu(&mut self, menu: Menu) {
        self.stack.push(menu);
    }

    pub fn in_menu(&mut self) -> bool {
        self.stack.len() > 0
    }
}
