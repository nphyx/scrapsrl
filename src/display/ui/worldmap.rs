use super::util::*;
use crate::component::Region;
use crate::constants::SIDEBAR_WIDTH;
use crate::resource::{Assets, WorldState};
use tcod::{Console, TextAlignment};

type TColor = tcod::colors::Color;

pub fn draw_worldmap(
    mut console: &dyn Console,
    assets: &Assets,
    region: Region,
    world: &WorldState,
) {
    reset_colors(&console);
    console.set_alignment(TextAlignment::Left);
    let base_x = console.width() - SIDEBAR_WIDTH + 3;
    let mut x = base_x;
    let mut y = 1;
    let width = SIDEBAR_WIDTH - 6;
    let height = SIDEBAR_WIDTH - 6;
    let hw = width / 2;
    let hh = height / 2;
    let fg = TColor::new(255, 255, 255);
    let bg_base = TColor::new(32, 32, 32);
    let mut bg: TColor;
    let mut ch: char;
    let horiz_line = assets
        .get_icon(&"line_single".to_string())
        .ch(false, false, true, true);
    draw_rect(
        console,
        base_x - 1,
        y - 1,
        width + 2,
        height + 2,
        assets.get_icon(&"line_single".to_string()),
    );
    for ry in region.y - hh..=region.y + hh {
        for rx in region.x - hw..=region.x + hw {
            let up = world.get_road(Region { x: rx, y: ry - 1 });
            let down = world.get_road(Region { x: rx, y: ry + 1 });
            let left = world.get_road(Region { x: rx - 1, y: ry });
            let right = world.get_road(Region { x: rx + 1, y: ry });
            let cur = world.get_road(Region { x: rx, y: ry });
            if cur.lanes_x == 0 && cur.lanes_y == 0 {
                ch = ' ';
            } else {
                let size = if cur.lanes_x > 3 || cur.lanes_y > 3 {
                    "line_double"
                } else {
                    "line_single"
                };
                if cur.lanes_x == 0 && cur.lanes_y > 0 {
                    ch = assets
                        .get_icon(&size.to_string())
                        .ch(true, true, false, false);
                } else if cur.lanes_x > 0 && cur.lanes_y == 0 {
                    ch = assets
                        .get_icon(&size.to_string())
                        .ch(false, false, true, true);
                } else {
                    ch = assets.get_icon(&size.to_string()).ch(
                        up.lanes_y > 0,
                        down.lanes_y > 0,
                        left.lanes_x > 0,
                        right.lanes_x > 0,
                    );
                }
            }
            if region.y == ry && region.x == rx {
                bg = TColor::new(128, 128, 128);
            } else {
                bg = bg_base;
            }
            console.put_char_ex(x, y, ch, fg, bg);
            x += 1;
        }
        x = base_x;
        y += 1;
    }
    console.set_alignment(TextAlignment::Center);
    console.print_rect(
        x + width / 2,
        y,
        width,
        1,
        format!("lat:{}{}long:{}", region.y, horiz_line, region.x),
    );
}
