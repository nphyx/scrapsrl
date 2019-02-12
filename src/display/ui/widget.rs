use super::util::*;
use crate::constants::{DIALOG_WIDTH, MAP_HEIGHT, MAP_WIDTH};
use crate::resource::*;
use crate::util::icons::*;
use std::sync::{Arc, Mutex};
use tcod::console::Console;
/// Draws UI widgets (dialogs, etc)
use tcod::{BackgroundFlag, TextAlignment};

/// Draws a dialog box.
pub fn draw_dialog(
    mut console: &dyn Console,
    cx: i32,
    cy: i32,
    title: String,
    body: String,
    footer: String,
) {
    reset_colors(&console);
    let width = DIALOG_WIDTH + 4;
    let height: i32 = count_lines(&body, width) + 4;
    let half_width: i32 = width / 2;
    let half_height: i32 = height / 2;
    console.set_alignment(TextAlignment::Left);
    console.rect(
        cx - half_width,
        cy - half_height,
        width,
        height,
        true,
        BackgroundFlag::Set,
    );
    horiz_line(
        console,
        cx - half_width,
        cy - half_height,
        width,
        LINE_DBL_HORIZ,
    );
    horiz_line(
        console,
        cx - half_width,
        cy + half_height,
        width,
        LINE_DBL_HORIZ,
    );
    vert_line(
        console,
        cx - half_width,
        cy - half_height,
        height,
        LINE_DBL_VERT,
    );
    vert_line(
        console,
        cx + half_width,
        cy - half_height,
        height,
        LINE_DBL_VERT,
    );
    console.put_char(
        cx - half_width,
        cy - half_height,
        LINE_DBL_TL,
        BackgroundFlag::None,
    );
    console.put_char(
        cx + half_width,
        cy - half_height,
        LINE_DBL_TR,
        BackgroundFlag::None,
    );
    console.put_char(
        cx - half_width,
        cy + half_height,
        LINE_DBL_BL,
        BackgroundFlag::None,
    );
    console.put_char(
        cx + half_width,
        cy + half_height,
        LINE_DBL_BR,
        BackgroundFlag::None,
    );
    console.set_alignment(TextAlignment::Center);
    console.print(cx, cy - half_height, title);
    console.print_rect(cx, cy - half_height + 2, width - 4, height - 4, body);
    console.set_alignment(TextAlignment::Right);
    console.print(cx + half_width - 2, cy + half_height, footer);
}

pub fn draw_centered_dialog(console: &dyn Console, dialog: &Arc<Mutex<dyn Widget>>) {
    let cx = MAP_WIDTH / 2;
    let cy = MAP_HEIGHT / 2;
    let lock = dialog.lock().unwrap();
    draw_dialog(
        console,
        cx,
        cy,
        lock.get_title(),
        lock.get_body(),
        lock.get_footer(),
    );
}
