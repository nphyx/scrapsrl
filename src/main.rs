extern crate rand;
extern crate tcod;

use tcod::{Console, RootConsole, FontLayout, FontType, BackgroundFlag};
use tcod::colors::{Color, lerp};
use tcod::input::Key;
use tcod::input::KeyCode::{F11, Escape};
use tcod::map::FovAlgorithm;
use rand::prelude::*;
mod mapgen;
mod tile;
mod ui;
mod game_state;
mod constants;
mod entity;
mod util;
mod player;
use crate::util::{clamp, plan};
use crate::entity::{Coord, Entity, Character};
use crate::game_state::GameState;
use crate::constants::{
    TORCH_RADIUS,
    MAP_WIDTH,
    MAP_HEIGHT,
    SCREEN_WIDTH,
    SCREEN_HEIGHT,
    DEFAULT_BG,
    DEFAULT_FG};


fn move_bug(&pos: &Coord, map: &tcod::map::Map) -> Option<Coord> {
    let mut rng = rand::thread_rng();
    let to = Coord{
        x: rng.gen_range(pos.x - 1, pos.x + 2),
        y: rng.gen_range(pos.y - 1, pos.y + 2)
    };
    plan(&to, &map)
}

fn distance(px: f32, py: f32, dx: f32, dy: f32) -> f32 {
    return ((dx - px).powf(2.0) + (dy - py).powf(2.0)).sqrt()
}

fn main() {
    let mut root = RootConsole::initializer()
        .font("monofur-nf.png", FontLayout::AsciiInRow)
        .font_type(FontType::Greyscale)
        .font_dimensions(102,636)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("SCRAPS: Bug Hunter")
        .init();

    root.set_default_background(DEFAULT_BG);
    root.set_default_foreground(DEFAULT_FG);

    
    let cx = MAP_WIDTH / 2;
    let cy = MAP_HEIGHT / 2;
    let mut rng = rand::thread_rng();
    let mut fullscreen = false;
    let mut interface = ui::UI::new();
    let mut bug = Character::blank();
    bug.set_pos(Coord{
        x: rng.gen_range(0, MAP_WIDTH),
        y: rng.gen_range(0, MAP_HEIGHT)
    });
    root.clear();

    let (map, tiles) = mapgen::generate(MAP_WIDTH, MAP_HEIGHT);
    let mut state = GameState::new(Character::blank(), map, tiles);
    state.player.set_pos(Coord{x: cx, y: cy});

    // Compute the FOV starting from the coordinates 20,20. Where we'll put the '@'
    // Use a max_radius of 10 and light the walls.
    state.map.compute_fov(20,20, TORCH_RADIUS, true, FovAlgorithm::Basic);

    let light = Color::new(200, 180, 50);
    let dark = Color::new(0, 6, 18);
    let ground = DEFAULT_BG; //Color::new(0, 40, 25);

    interface.open_menu(
        ui::Notification::new(
            format!("Start Game"),
            format!("Find and catch the bugs!"),
            )
    );

    while !root.window_closed() {
        root.clear();
        // Compute the FOV starting from the coordinates 20,20. Where we'll put the '@'
        // Use a max_radius of 10 and light the walls.
        state.map.compute_fov(state.player.pos().x, state.player.pos().y, 15, true, FovAlgorithm::Basic);

        for ((px, py), tile) in &state.tiles {
            let visible = state.map.is_in_fov(*px, *py);
            let dist = clamp(
                0.0,
                1.0,
                distance(state.player.pos().x as f32, state.player.pos().y as f32, *px as f32, *py as f32)
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

        if state.map.is_in_fov(bug.pos().x, bug.pos().y) {
            root.put_char(bug.pos().x, bug.pos().y, '\u{f46f}', BackgroundFlag::None);
            root.set_char_foreground(bug.pos().x, bug.pos().y, Color{r: 32, g: 128, b: 225});
        }
        root.put_char(state.player.pos().x, state.player.pos().y, '\u{e213}', BackgroundFlag::None);

        if state.player.pos() == bug.pos() {
            state.score += 1;
            bug.set_pos(Coord{
                x: rng.gen_range(0, MAP_WIDTH),
                y: rng.gen_range(0, MAP_HEIGHT)
            });

            interface.open_menu(
                ui::Notification::new(
                    format!("Success"),
                    format!("Got 'em!"),
                    )
                );
        }
        interface.draw(&root, &state);
        root.flush();
        let keypress = root.wait_for_keypress(true);
        // libtcod 1.5.1 has a bug where `wait_for_keypress` emits two events:
        // one for key down and one for key up. So we ignore the "key up" ones.
        if keypress.pressed {
            // handle buttons that should always work even in menus
            match keypress {
                Key { code: F11, .. } => {
                    fullscreen = !fullscreen;
                    root.set_fullscreen(fullscreen);
                },
                _ => {}
            }
            if !interface.handle_input(keypress, &mut state) {
                if state.player.handle_input(&keypress, &state.map) {
                    println!("player acted");
                    match move_bug(&bug.pos(), &state.map) {
                        Some(coord) => bug.set_pos(coord),
                        _ => {}
                    }
                    state.player.tick();
                } else {
                    match keypress {
                        Key { code: Escape, .. } => break,
                        _ => {}
                    }

                }
            }
        }
    }
}
