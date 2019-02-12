use super::util::*;
use crate::component::Character;
use crate::constants::{DEFAULT_BG, SIDEBAR_WIDTH};
use crate::util::icons::*;
use tcod::{Console, TextAlignment};

type TColor = tcod::colors::Color;

pub fn draw_stats(mut console: &dyn Console, pc: &Character) {
    reset_colors(&console);
    console.set_alignment(TextAlignment::Left);
    let x = console.width() - SIDEBAR_WIDTH;
    let y = 0;
    let width = SIDEBAR_WIDTH;
    let height = console.height();
    vert_line(console, x, y, height, LINE_DBL_VERT);
    let text = format!(
        concat!(
            "   THIS IS SIDEBAR\n",
            "\n",
            "ATTR   \u{250c}POW SUB RES\u{2510}\n",
            "Body:{} |S:{} G:{} T:{}|\n",
            "Mind:{} |I:{} W:{} R:{}|\n",
            "Soul:{} |C:{} E:{} W:{}|\n",
            "       \u{2514}-----------\u{2518}\n",
            "\n"
        ),
        pc.body(),
        pc.strength(),
        pc.grace(),
        pc.toughness(),
        pc.mind(),
        pc.intellect(),
        pc.wits(),
        pc.resolve(),
        pc.soul(),
        pc.charisma(),
        pc.empathy(),
        pc.will()
    );

    console.print_rect(x + 2, y, width - 2, height, text);
}

use tcod::colors::Color;

fn draw_info(
    mut console: &dyn Console,
    ch: char,
    fg: Color,
    bg: Color,
    short: String,
    long: String,
) {
    let x = console.width() - SIDEBAR_WIDTH;
    let width = SIDEBAR_WIDTH;
    let y = 13;
    console.put_char_ex(x + 2, y, ch, fg, bg);
    console.print_rect(x + 4, y, width - 6, 1, short);
    console.print_rect(x + 4, y + 2, width - 6, 10, long);
}

use crate::component::{Colors, Description, Icon};
/// draw entity in infobox area of sidebar
pub fn draw_entity_info(
    console: &dyn Console,
    icon: Icon,
    colors: Colors,
    description: &Description,
) {
    draw_info(
        console,
        icon.ch,
        TColor::from(colors.fg),
        DEFAULT_BG,
        description.short.clone(),
        description.long.clone(),
    );
}

use crate::resource::{get_tile_descriptions, Tile};
/// draw tile in info box area of sidebar
pub fn draw_tile_info(console: &dyn Console, tile: Tile) {
    let t = &get_tile_descriptions()[tile.type_id as usize];
    draw_info(
        console,
        tile.icon,
        TColor::from(tile.fg),
        TColor::from(tile.bg),
        t.short_desc.to_string(),
        t.long_desc.to_string(),
    );
}
