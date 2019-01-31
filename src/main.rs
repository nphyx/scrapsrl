extern crate rand;
extern crate tcod;

use tcod::{Console, RootConsole, FontLayout, FontType, BackgroundFlag};
use tcod::colors::{Color, lerp};
use tcod::input::Key;
use tcod::input::KeyCode::{Up, Down, Left, Right, Escape, F11};
use tcod::map::FovAlgorithm;
use rand::prelude::*;
mod mapgen;
mod tiles;
mod ui;
mod game_state;
use crate::game_state::GameState;
use crate::ui::SIDEBAR_WIDTH;


const TILE_HEIGHT: i32 = 24;
const TILE_WIDTH: i32 = 16;

const SCREEN_HEIGHT: i32 = 1080 / TILE_HEIGHT - 1;
const SCREEN_WIDTH: i32 = 1920 / TILE_WIDTH;

const MAP_WIDTH: i32 = SCREEN_WIDTH - SIDEBAR_WIDTH;
const MAP_HEIGHT: i32 = SCREEN_HEIGHT - 1;

const TORCH_RADIUS: i32 = 20;

const DEFAULT_BG: Color = Color{r: 0, g:12, b:9};
const DEFAULT_FG: Color = Color{r: 225, g: 255, b: 232};

fn clamp(a: f32, b: f32, x: f32) -> f32 {
    if x < a { a } else if x > b { b } else { x }
}

fn plan(mut x:i32, mut y:i32, mut tx:i32, mut ty:i32, map: &tcod::map::Map) -> (i32, i32) {
    tx = clamp(0.0, (MAP_WIDTH - 1) as f32, tx as f32) as i32;
    ty = clamp(0.0, (MAP_HEIGHT - 1) as f32, ty as f32) as i32;
    if map.is_walkable(tx, ty) {
        x = tx;
        y = ty;
    }
    return (x, y);
}

fn move_bug(bx: i32, by: i32, map: &tcod::map::Map) -> (i32, i32) {
    let mut rng = rand::thread_rng();
    let tbx = rng.gen_range(bx - 1, bx + 2);
    let tby = rng.gen_range(by - 1, by + 2);
    return plan(bx, by, tbx, tby, &map);
}

fn distance(px: f32, py: f32, dx: f32, dy: f32) -> f32 {
    return ((dx - px).powf(2.0) + (dy - py).powf(2.0)).sqrt()
}

fn main() {
    let mut root = RootConsole::initializer()
        .font("monofur-nf.png", FontLayout::AsciiInRow)
        .font_type(FontType::Greyscale)
        .font_dimensions(128,507)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("SCRAPS: Bug Hunter")
        .init();

    root.set_default_background(DEFAULT_BG);
    root.set_default_foreground(DEFAULT_FG);

    
    let cx = MAP_WIDTH / 2;
    let cy = MAP_HEIGHT / 2;
    let mut x = cx;
    let mut y = cy;
    let mut tx; // planned x loc
    let mut ty; // planned y loc
    let mut rng = rand::thread_rng();
    let mut bx = rng.gen_range(0, MAP_WIDTH);
    let mut by = rng.gen_range(0, MAP_HEIGHT);
    let mut fullscreen = false;
    let mut state = GameState::new();
    let mut interface = ui::UI::new();
    root.clear();

    let (mut map, tiles) = mapgen::generate(MAP_WIDTH, MAP_HEIGHT);

    // Compute the FOV starting from the coordinates 20,20. Where we'll put the '@'
    // Use a max_radius of 10 and light the walls.
    map.compute_fov(20,20, TORCH_RADIUS, true, FovAlgorithm::Basic);

    let light = Color::new(200, 180, 50);
    let dark = Color::new(0, 6, 18);
    let ground = DEFAULT_BG; //Color::new(0, 40, 25);

    while !root.window_closed() {
        root.clear();
        // Compute the FOV starting from the coordinates 20,20. Where we'll put the '@'
        // Use a max_radius of 10 and light the walls.
        map.compute_fov(x, y, 15, true, FovAlgorithm::Basic);

        for ((px, py), tile) in &tiles {
            let visible = map.is_in_fov(*px, *py);
            let dist = clamp(
                0.0,
                1.0,
                distance(x as f32, y as f32, *px as f32, *py as f32)
                / TORCH_RADIUS as f32);
            let fg: Color;
            let bg: Color;
            let blend = lerp(light, dark, dist);
            if visible && (dist < TORCH_RADIUS as f32) {
                bg = lerp(ground, blend, 0.3);
                fg = lerp(tile.color, blend, 0.7);
            } else if visible {
                bg = lerp(ground, blend, 0.5);
                fg = lerp(tile.color, blend, 0.5);
            } else {
                bg = lerp(ground, dark, 0.5);
                fg = lerp(tile.color, dark, 0.7);
            }
            root.put_char_ex(*px, *py, tile.ch, fg, bg);
        }

        if map.is_in_fov(bx, by) {
            root.put_char(bx, by, '\u{f46f}', BackgroundFlag::None);
            root.set_char_foreground(bx, by, Color{r: 32, g: 128, b: 225});
        }
        root.put_char(x, y, '\u{e213}', BackgroundFlag::None);

        if x == bx && y == by {
            state.score += 1;
            bx = rng.gen_range(0, MAP_WIDTH);
            by = rng.gen_range(0, MAP_HEIGHT);

            interface.open(
                ui::Menu::new(
                    format!("Got 'em"),
                    format!("[esc to continue]"),
                    ui::MenuType::CenteredDialog),
                &mut state);
        }
        interface.draw(&root, &state);
        root.flush();
        let keypress = root.wait_for_keypress(true);
        // libtcod 1.5.1 has a bug where `wait_for_keypress` emits two events:
        // one for key down and one for key up. So we ignore the "key up" ones.
        if keypress.pressed {
            ty = y;
            tx = x;
            // handle buttons that should always work even in menus
            match keypress {
                Key { code: F11, .. } => {
                    fullscreen = !fullscreen;
                    root.set_fullscreen(fullscreen);
                },
                _ => {}
            }
            if !interface.handle_input(keypress, &mut state) {
                match keypress {
                    Key { code: Escape, .. } => break,
                    Key { code: Up, .. } => ty = y - 1,
                    Key { code: Down, .. } => ty = y + 1,
                    Key { code: Left, .. } => tx = x - 1,
                    Key { code: Right, .. } => tx = x + 1,
                    _ => {}
                }
                let bp = move_bug(bx, by, &map);
                bx = bp.0;
                by = bp.1;

                let p = plan(x, y, tx, ty, &map);
                x = p.0;
                y = p.1;
            }
        }
    }
}
