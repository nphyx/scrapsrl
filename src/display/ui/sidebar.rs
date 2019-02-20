use super::util::*;
use crate::component::Character;
use crate::constants::{DEFAULT_BG, SIDEBAR_WIDTH};
use crate::resource::Assets;
use tcod::{Console, TextAlignment};

type TColor = tcod::colors::Color;

pub fn draw_sidebar_frame(mut console: &dyn Console, assets: &Assets) {
    reset_colors(&console);
    console.set_alignment(TextAlignment::Left);

    let x = console.width() - SIDEBAR_WIDTH;
    let y = 0;
    let height = console.height();

    vert_line(
        console,
        x,
        y,
        height,
        assets.get_icon("line_double").ch(true, true, false, false),
    );
}

pub fn draw_stats(mut console: &dyn Console, assets: &Assets, pc: &Character) {
    reset_colors(&console);
    console.set_alignment(TextAlignment::Left);
    let x = console.width() - SIDEBAR_WIDTH;
    let y = SIDEBAR_WIDTH - 3;

    console.print_rect(
        x + 2,
        y + 1,
        7,
        4,
        format!(
            "ATTR\nBody:{}\nMind:{}\nSoul:{}\n",
            pc.body(),
            pc.mind(),
            pc.soul()
        ),
    );

    draw_rect(console, x + 9, y + 1, 13, 5, assets.get_icon("line_single"));

    let horiz_line = assets.get_icon("line_single").ch(false, false, true, true);
    console.print_rect(
        x + 10,
        y + 1,
        14,
        4,
        format!(
            "POW{}SUB{}RES\nS:{} G:{} T:{}\nI:{} W:{} R:{}\nC:{} E:{} W:{}",
            horiz_line,
            horiz_line,
            pc.strength(),
            pc.grace(),
            pc.toughness(),
            pc.intellect(),
            pc.wits(),
            pc.resolve(),
            pc.charisma(),
            pc.empathy(),
            pc.will()
        ),
    );
}

use tcod::colors::Color;

fn draw_info(mut console: &dyn Console, ch: char, fg: Color, bg: Color, short: &str, long: &str) {
    let x = console.width() - SIDEBAR_WIDTH;
    let width = SIDEBAR_WIDTH;
    let y = SIDEBAR_WIDTH + 14;
    console.put_char_ex(x + 2, y, ch, fg, bg);
    console.print_rect(x + 4, y, width - 6, 1, short);
    console.print_rect(x + 4, y + 2, width - 6, 10, long);
}

use crate::component::{Colors, Description};
/// draw entity in infobox area of sidebar
pub fn draw_entity_info(
    console: &dyn Console,
    icon: char,
    colors: Colors,
    description: &Description,
) {
    draw_info(
        console,
        icon,
        TColor::from(colors.fg),
        DEFAULT_BG,
        &*description.short,
        &*description.long,
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
        t.short_desc,
        t.long_desc,
    );
}
