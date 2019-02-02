extern crate rand;
extern crate tcod;

use tcod::input::Key;
use tcod::input::KeyCode::{F11, Escape};
use tcod::map::FovAlgorithm;
use tcod::colors::Color;
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
use crate::entity::{Coord, Entity, Character, body_layout};
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

fn make_bug() -> Character {
  let mut rng = rand::thread_rng();
  let mut bug = Character::blank();
  bug.set_ch('\u{f46f}');
  bug.set_pos(Coord{
    x: rng.gen_range(0, MAP_WIDTH),
    y: rng.gen_range(0, MAP_HEIGHT)
  });
  bug.set_color(Color{r: 32, g: 128, b: 225});
  bug.set_body_layout(body_layout::insectoid());

  return bug;
}

fn handle_bugs(state: &mut GameState, interface: &mut ui::UI) {
  let mut rng = rand::thread_rng();
  for bug in state.entities.iter_mut() {
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
    else {
      match move_bug(&bug.pos(), &state.map) {
        Some(coord) => bug.set_pos(coord),
        _ => {}
      }
    }
  }
}

fn main() {
  let mut display = Display::new();
  let cx = MAP_WIDTH / 2;
  let cy = MAP_HEIGHT / 2;
  let mut rng = rand::thread_rng();
  let mut fullscreen = false;
  let mut interface = ui::UI::new();

  let (map, tiles) = mapgen::generate(MAP_WIDTH, MAP_HEIGHT);
  let mut state = GameState::new(Character::blank(), map, tiles);
  state.player.set_pos(Coord{x: cx, y: cy});
  state.player.character.set_body_layout(body_layout::humanoid());
  for _ in 0..3 {
    state.entities.push(Box::new(make_bug()));
  }

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

    state.map.compute_fov(state.player.pos().x, state.player.pos().y, TORCH_RADIUS, true, FovAlgorithm::Basic);
    display.draw(&state, &mut interface);
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
          handle_bugs(&mut state, &mut interface);
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
