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
mod display;
mod cursor;
use crate::display::Display;
use crate::entity::{Coord, Entity, Character, body_layout, Object, Player, NPC, EntityCollection};
use crate::game_state::GameState;
use crate::constants::{
  TORCH_RADIUS,
  MAP_WIDTH,
  MAP_HEIGHT};

fn make_bug() -> NPC {
  let mut rng = rand::thread_rng();
  let mut bug = Character::blank();
  bug.set_ch('\u{f46f}');
  bug.set_pos(Coord{
    x: rng.gen_range(0, MAP_WIDTH),
    y: rng.gen_range(0, MAP_HEIGHT)
  });
  bug.set_color(Color{r: 32, g: 128, b: 225});
  bug.set_body_layout(body_layout::insectoid());

  return NPC::new(bug);
}

fn handle_bugs(interface: &mut ui::UI, player: &mut Player, entities: &mut EntityCollection) {
  let mut rng = rand::thread_rng();
  for bug in entities.iter_mut() {
    if player.pos() == bug.pos() {
      player.score += 1;
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
  }
}

fn main() {
  let mut display = Display::new();
  let cx = MAP_WIDTH / 2;
  let cy = MAP_HEIGHT / 2;
  let mut rng = rand::thread_rng();
  let mut fullscreen = false;
  let mut interface = ui::UI::new();
  let mut player = Player::new(Character::blank());
  let mut entities = EntityCollection::new();
  player.set_pos(Coord{x: cx, y: cy});
  player.character.set_body_layout(body_layout::humanoid());

  let (map, tiles) = mapgen::generate(MAP_WIDTH, MAP_HEIGHT);
  let mut state = GameState::new(map, tiles);
  let mut computer = Object::new();
  computer.set_ch('\u{fcbe}');
  computer.set_pos(Coord{x: rng.gen_range(0, MAP_WIDTH), y: rng.gen_range(0, MAP_HEIGHT)});
  entities.push(Box::new(computer));
  for _ in 0..3 {
    entities.push(Box::new(make_bug()));
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

    state.map.compute_fov(player.pos().x, player.pos().y, TORCH_RADIUS, true, FovAlgorithm::Basic);
    display.draw(&state, &mut interface, &player, &entities);
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
        if player.handle_input(&keypress, &state.map, &entities) {
          player.tick(&state);
          for entity in entities.iter_mut() {
            entity.tick(&state);
          }
          handle_bugs(&mut interface, &mut player, &mut entities);
        }
        match keypress {
          Key { code: Escape, .. } => break,
          _ => {}
        }
      }
    }
  }
}
