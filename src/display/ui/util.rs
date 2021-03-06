use crate::resource::Icon;
use tcod::colors::Color;
use tcod::{BackgroundFlag, Console};

const TEXT_COLOR: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};
const DIALOG_BG: Color = Color { r: 0, g: 0, b: 0 };

/// makes a horizontal meter like [###----X] where # is full, - is empty, X is blocked
pub fn horizontal_meter(max: u8, cur: u8, cap: u8) -> String {
    let gap: u8 = max - cur - cap;
    return format!(
        "[{:\u{2588}<3$}{:\u{2592}<4$}{:\u{2593}<5$}]",
        "", "", "", cur as usize, gap as usize, cap as usize
    );
}

/// clears console colors
pub fn reset_colors(mut console: &dyn Console) {
    console.set_default_foreground(TEXT_COLOR);
    console.set_default_background(DIALOG_BG);
}

/// Counts the number of lines a string will have to be at a given width
pub fn count_lines(text: &str, max_length: i32) -> i32 {
    let len: i32 = text.len() as i32;
    let mut lines = 0;
    for c in text.chars() {
        if c == '\n' {
            lines += 1;
        }
    }
    lines += ((len - lines) as f32 / max_length as f32).ceil() as i32;
    lines
}

/// draws a rectangle, using connected tiles if the icon supports it
pub fn draw_rect(mut console: &dyn Console, x: i32, y: i32, width: i32, height: i32, icon: &Icon) {
    // lay out lines
    vert_line(
        console,
        x,
        y,
        height,
        icon.connected(true, true, false, false).ch(),
    );
    vert_line(
        console,
        x + width - 1,
        y,
        height,
        icon.connected(true, true, false, false).ch(),
    );
    horiz_line(
        console,
        x,
        y,
        width,
        icon.connected(false, false, true, true).ch(),
    );
    horiz_line(
        console,
        x,
        y + height - 1,
        width,
        icon.connected(false, false, true, true).ch(),
    );
    console.put_char(
        // top left corner
        x,
        y,
        icon.connected(false, true, false, true).ch(),
        BackgroundFlag::None,
    );
    console.put_char(
        // top right corner
        x + width - 1,
        y,
        icon.connected(false, true, true, false).ch(),
        BackgroundFlag::None,
    );
    console.put_char(
        // bottom left corner
        x,
        y + height - 1,
        icon.connected(true, false, false, true).ch(),
        BackgroundFlag::None,
    );
    console.put_char(
        // bottom right corner
        x + width - 1,
        y + height - 1,
        icon.connected(true, false, true, false).ch(),
        BackgroundFlag::None,
    );
}

/// fills a rectangular area
pub fn fill(mut console: &dyn Console, x: i32, y: i32, tx: i32, ty: i32, ch: char) {
    for cx in x..tx {
        for cy in y..ty {
            console.put_char(cx, cy, ch, BackgroundFlag::None);
        }
    }
}

/// draws a horizontal line
pub fn horiz_line(console: &dyn Console, x: i32, y: i32, width: i32, ch: char) {
    fill(console, x, y, x + width, y + 1, ch);
}

/// draws a vertical line
pub fn vert_line(console: &dyn Console, x: i32, y: i32, height: i32, ch: char) {
    fill(console, x, y, x + 1, y + height, ch);
}
