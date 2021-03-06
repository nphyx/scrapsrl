use super::util::*;
use crate::component::Region;
use crate::constants::SIDEBAR_WIDTH;
use crate::resource::{Assets, GameState, MapMode, WorldState};
use tcod::{Console, TextAlignment};

type TColor = tcod::colors::Color;

pub fn draw_worldmap(
    mut console: &dyn Console,
    assets: &Assets,
    region: Region,
    world: &WorldState,
    state: &GameState,
) {
    reset_colors(&console);
    console.set_alignment(TextAlignment::Left);
    let base_x = console.width() - SIDEBAR_WIDTH + 3;
    let mut x = base_x;
    let base_y = 1;
    let mut y = base_y;
    let width = SIDEBAR_WIDTH - 6;
    let height = SIDEBAR_WIDTH - 6;
    let hw = width / 2;
    let hh = height / 2;
    let fg = TColor::new(16, 16, 16);
    let mut bg: TColor;
    let mut ch: char;
    let horiz_line = assets
        .get_icon("line_single")
        .connected(false, false, true, true)
        .ch();
    draw_rect(
        console,
        base_x - 1,
        y - 1,
        width + 2,
        height + 2,
        assets.get_icon("line_single"),
    );
    for ry in region.y - hh..=region.y + hh {
        for rx in region.x - hw..=region.x + hw {
            let up = world.get_road(Region { x: rx, y: ry - 1 });
            let down = world.get_road(Region { x: rx, y: ry + 1 });
            let left = world.get_road(Region { x: rx - 1, y: ry });
            let right = world.get_road(Region { x: rx + 1, y: ry });
            let cur = world.get_road(Region { x: rx, y: ry });
            match state.map_mode {
                MapMode::Hybrid => {
                    if cur.lanes_x == 0 && cur.lanes_y == 0 {
                        ch = world.get_icon(Region { x: rx, y: ry });
                    } else {
                        let size = if cur.lanes_x > 4 || cur.lanes_y > 4 {
                            "map_hybrid_large"
                        } else if cur.lanes_x > 1 || cur.lanes_y > 1 {
                            "map_hybrid_medium"
                        } else {
                            "map_hybrid_small"
                        };
                        ch = assets
                            .get_icon(size)
                            .connected(
                                cur.lanes_y > 0 && up.lanes_y > 0,
                                cur.lanes_y > 0 && down.lanes_y > 0,
                                cur.lanes_x > 0 && left.lanes_x > 0,
                                cur.lanes_x > 0 && right.lanes_x > 0,
                            )
                            .ch();
                    }
                }
                MapMode::Street => {
                    if cur.lanes_x == 0 && cur.lanes_y == 0 {
                        ch = ' ';
                    } else {
                        let size = if cur.lanes_x > 4 || cur.lanes_y > 4 {
                            "map_street_large"
                        } else if cur.lanes_x > 1 || cur.lanes_y > 1 {
                            "map_street_medium"
                        } else {
                            "map_street_small"
                        };
                        ch = assets
                            .get_icon(size)
                            .connected(
                                cur.lanes_y > 0 && up.lanes_y > 0,
                                cur.lanes_y > 0 && down.lanes_y > 0,
                                cur.lanes_x > 0 && left.lanes_x > 0,
                                cur.lanes_x > 0 && right.lanes_x > 0,
                            )
                            .ch();
                    }
                }
                MapMode::Terrain => {
                    ch = world.get_icon(Region { x: rx, y: ry });
                }
            }
            // let cur_region = Region::new(rx, ry);
            // let pop_col = 128; // ((world.get_pop(cur_region) * 128.0).floor()) as u8 + 64;
            if region.y == ry && region.x == rx {
                bg = TColor::new(128, 220, 128);
            } else {
                bg = TColor::new(128, 128, 128);
            }
            console.put_char_ex(x, y, ch, fg, bg);
            x += 1;
        }
        x = base_x;
        y += 1;
    }
    console.set_alignment(TextAlignment::Center);
    let mode_string: &str = match state.map_mode {
        MapMode::Hybrid => "[M]ode:hybrid",
        MapMode::Terrain => "[M]ode:terrain",
        MapMode::Street => "[M]ode:street",
    };
    console.print_rect(x + width / 2, base_y - 1, width, 1, mode_string);

    console.print_rect(
        x + width / 2,
        y,
        width,
        1,
        format!(
            "x:{}{}y:{}{}pop:{:.*}",
            region.x,
            horiz_line,
            region.y,
            horiz_line,
            1,
            world.get_pop(region) * 10.0
        ),
    );
}
