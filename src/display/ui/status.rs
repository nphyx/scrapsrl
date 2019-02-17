use super::util::*;
use crate::component::Character;
use crate::resource::GameState;
use tcod::console::{Console, TextAlignment};

pub fn draw_status_bar(mut console: &dyn Console, pc: &Character, state: &GameState) {
    reset_colors(&console);
    console.set_alignment(TextAlignment::Left);
    let x = 0;
    let y = console.height() - 1;
    let width = console.width();
    let height = 1;
    let mut text: String;

    if state.paused {
        text = "-- PAUSED --".to_string();
    } else if state.fast_forward {
        text = "-- FAST-FORWARDING -- [.] stop".to_string();
    } else if state.looking {
        text = "-- LOOKING --   [Enter] interact   [Esc] cancel".to_string();
    } else {
        let stamina: (u8, u8, u8) = pc.stamina();
        let focus: (u8, u8, u8) = pc.focus();
        let grit: (u8, u8, u8) = pc.grit();
        text = format!(
            concat!("stamina: {} ", "focus: {} ", "grit: {} [Enter] look"),
            horizontal_meter(stamina.0, stamina.1, stamina.2),
            horizontal_meter(focus.0, focus.1, focus.2),
            horizontal_meter(grit.0, grit.1, grit.2)
        );
    }
    console.print_rect(x, y, width, height, text);
}
