use tcod::console::{Console, TextAlignment, BackgroundFlag};
use tcod::colors::Color;
use crate::game_state::GameState;
use crate::constants::{SIDEBAR_WIDTH, MAP_WIDTH, MAP_HEIGHT, DIALOG_WIDTH};

const TEXT_COLOR: Color = Color{r: 255, g: 255, b: 255};
const DIALOG_BG: Color = Color{r: 0, g: 0, b: 0};
const CORNER_TL_CH: char = '\u{2554}';
const CORNER_TR_CH: char = '\u{2557}';
const CORNER_BL_CH: char = '\u{255a}';
const CORNER_BR_CH: char = '\u{255d}';
const HORIZ_LINE_CH: char = '\u{2550}';
const VERT_LINE_CH: char = '\u{2551}';


pub fn reset_colors(mut console: &Console) {
    console.set_default_foreground(TEXT_COLOR);
    console.set_default_background(DIALOG_BG);
}

pub fn count_lines(text: &String, max_length: i32) -> i32 {
    let len:i32 = text.len() as i32;
    let mut lines = 0;
    for c in text.chars() {
        if c == '\n' { lines += 1; }
    }
    lines += ((len - lines) as f32 / max_length as f32).ceil() as i32;
    lines
}

pub fn fill(mut console: &Console, x:i32, y:i32, tx:i32, ty:i32, ch: char) {
    for cx in x..tx {
        for cy in y..ty {
            console.put_char(cx, cy, ch, BackgroundFlag::None);
        }
    }
}

pub fn horiz_line(console: &Console, x:i32, y:i32, width: i32, ch: char) {
    fill(console, x, y, x + width, y + 1, ch);
}

pub fn vert_line(console: &Console, x:i32, y:i32, height: i32, ch: char) {
    fill(console, x, y, x + 1, y + height, ch);
}

pub fn draw_dialog(
        mut console: &Console,
        cx: i32,
        cy:i32,
        title: &String,
        body: &String,
        footer: &String) {
    reset_colors(&console);
    let width = DIALOG_WIDTH + 4;
    let height: i32 = count_lines(body, width) + 4;
    let half_width:i32 = width / 2;
    let half_height:i32 = height / 2;
    console.set_alignment(TextAlignment::Left);
    console.rect(
        cx - half_width,
        cy - half_height,
        width,
        height,
        true,
        BackgroundFlag::Set);
    horiz_line(console, cx - half_width, cy - half_height, width, HORIZ_LINE_CH);
    horiz_line(console, cx - half_width, cy + half_height, width, HORIZ_LINE_CH);
    vert_line(console, cx - half_width, cy - half_height, height, VERT_LINE_CH);
    vert_line(console, cx + half_width, cy - half_height, height, VERT_LINE_CH);
    console.put_char(cx - half_width, cy - half_height, CORNER_TL_CH, BackgroundFlag::None);
    console.put_char(cx + half_width, cy - half_height, CORNER_TR_CH, BackgroundFlag::None);
    console.put_char(cx - half_width, cy + half_height, CORNER_BL_CH, BackgroundFlag::None);
    console.put_char(cx + half_width, cy + half_height, CORNER_BR_CH, BackgroundFlag::None);
    console.set_alignment(TextAlignment::Center);
    console.print(cx, cy - half_height, title);
    console.print_rect(cx, cy - half_height + 2, width - 4, height - 4, body);
    console.set_alignment(TextAlignment::Right);
    console.print(cx + half_width - 2, cy + half_height, footer);
}

pub fn draw_centered_dialog(console: &Console, title: &String, body: &String, footer: &String) {
    let cx = MAP_WIDTH / 2;
    let cy = MAP_HEIGHT / 2;
    draw_dialog(console, cx, cy, title, body, footer);
}

pub fn draw_status_bar(mut console: &Console, text: String) {
    reset_colors(&console);
    console.set_alignment(TextAlignment::Left);
    let x = 0;
    let y = console.height() - 1;
    let width = console.width();
    let height = 1;
    console.print_rect(x, y, width, height, text);
}

pub fn draw_sidebar(mut console: &Console, _state: &GameState) {
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
