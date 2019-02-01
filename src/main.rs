extern crate rand;
extern crate tcod;

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
mod display;
use crate::display::Display;
use crate::util::plan;
use crate::entity::{Coord, Entity, Character};
use crate::game_state::GameState;
use crate::constants::{
  TORCH_RADIUS,
  MAP_WIDTH,
  MAP_HEIGHT};


fn move_bug(&pos: &Coord, map: &tcod::map::Map) -> Option<Coord> {
  let mut rng = rand::thread_rng();
  let to = Coord{
    x: rng.gen_range(pos.x - 1, pos.x + 2),
    y: rng.gen_range(pos.y - 1, pos.y + 2)
  };
  plan(&to, &map)
}

fn main() {
  let mut display = Display::new();
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

  let (map, tiles) = mapgen::generate(MAP_WIDTH, MAP_HEIGHT);
  let mut state = GameState::new(Character::blank(), map, tiles);
  state.player.set_pos(Coord{x: cx, y: cy});

  // Compute the FOV starting from the coordinates 20,20. Where we'll put the '@'
  // Use a max_radius of 10 and light the walls.
  // state.map.compute_fov(20,20, TORCH_RADIUS, true, FovAlgorithm::Basic);

  interface.open_menu(
    ui::Chain::new(vec![
        Box::new(ui::Notification::new(
          format!("SCRAPS: Bug Hunter"),
          format!("Your task, should you choose to accept it, is to catch bugs."),
        )),
        Box::new(ui::Notification::new(
          format!("Start Game"),
          format!("Find and catch the bugs!"),
        )),
    ])
  );

  while !display.root.window_closed() {
    // game success state
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

    state.map.compute_fov(state.player.pos().x, state.player.pos().y, TORCH_RADIUS, true, FovAlgorithm::Basic);
    display.draw(&state, &bug, &mut interface);
    let keypress = display.root.wait_for_keypress(true);
    // libtcod 1.5.1 has a bug where `wait_for_keypress` emits two events:
    // one for key down and one for key up. So we ignore the "key up" ones.
    if keypress.pressed {
      // handle buttons that should always work even in menus
      match keypress {
        Key { code: F11, .. } => {
          fullscreen = !fullscreen;
          display.root.set_fullscreen(fullscreen);
        },
        _ => {}
      }
      if !interface.handle_input(keypress, &mut state) {
        if state.player.handle_input(&keypress, &state.map) {
          match move_bug(&bug.pos(), &state.map) {
            Some(coord) => bug.set_pos(coord),
            _ => {}
          }
          state.player.tick();
        }
        match keypress {
          Key { code: Escape, .. } => break,
          _ => {}
        }
      }
    }
  }
}
