extern crate rand;
extern crate tcod;
extern crate specs;
extern crate frappe;
#[macro_use]
extern crate specs_derive;

use tcod::input::Key;
// use tcod::input::KeyCode::{F11, Escape};
use tcod::colors::Color;
use rand::prelude::*;
use specs::{World, DispatcherBuilder};

mod mapgen;
mod ui;
mod game_state;
mod constants;
mod entity;
mod util;
mod display;
mod cursor;
mod component;
mod system;
use self::component::Position;
use self::ui::Notification;
use self::util::icons::*;
use self::display::Display;
use self::system::*;
use self::entity::{Coord, Entity, Character, body_layout, Object, Player, NPC, EntityCollection, EntityInteraction, make_object_entity};
use self::game_state::GameState;
use self::constants::{
  TORCH_RADIUS,
  MAP_WIDTH,
  MAP_HEIGHT};

fn make_bug() -> NPC {
  let mut rng = rand::thread_rng();
  let mut bug = Character::blank();
  bug.set_ch(ICON_BUG);
  bug.set_pos(Coord{
    x: rng.gen_range(0, MAP_WIDTH),
    y: rng.gen_range(0, MAP_HEIGHT)
  });
  bug.set_color(Color{r: 32, g: 128, b: 225});
  bug.set_body_layout(body_layout::insectoid());
  bug.set_desc("A housecat-sized cockroach. Electric sparks arc between its antenna.".to_string());
  let mut npc_wrap = NPC::new(bug);
  npc_wrap.set_notification(
    Notification::new(
      format!("Success"),
      format!("Got 'em!")));
  npc_wrap
}

fn make_computer() -> Object {
  let mut rng = rand::thread_rng();
  let mut computer = Object::new();
  computer.set_ch(ICON_OLD_COMPUTER);
  computer.set_pos(Coord{x: rng.gen_range(0, MAP_WIDTH), y: rng.gen_range(0, MAP_HEIGHT)});
  computer.set_notification(
    Notification::new(
      format!(" {} ", ICON_OLD_COMPUTER).to_string(),
      "Bleep, bloop!".to_string())
  );
  computer.set_desc("An old-world electronic device. Looks like it's still working.".to_string());
  return computer;
}

fn handle_player_interact(state: &mut GameState, interface: &mut ui::UI, player: &mut Player, entities: &mut EntityCollection) {
  /*
  match player.wants_interact_at {
    Some(coord) => {
      for entity in entities.iter_mut() {
        if entity.pos() == coord {
          match entity.player_interact(player, state) {
            EntityInteraction::Notification(notice) => {
              interface.open_menu(notice);
              break;
            },
            EntityInteraction::None => {}
          }
        }
      }
    },
    None => {}
  }
  */
}

#[derive(Default,Clone)]
pub struct WindowClosed(bool);

fn main() {
  // let cx = MAP_WIDTH / 2;
  // let cy = MAP_HEIGHT / 2;
  // let mut fullscreen = false;
  // let mut interface = ui::UI::new();
  // let mut player = Player::new(Character::blank());
  // let mut entities = EntityCollection::new();

  let mut world = World::new();
  let (map, tiles) = mapgen::generate(MAP_WIDTH, MAP_HEIGHT);
  let display = Display::new(map);
  let keypress = Key::default();
  world.register::<entity::Character>();
  world.register::<entity::Player>();
  world.register::<component::Position>();
  world.register::<component::Description>();
  world.register::<component::Icon>();
  world.register::<component::Color>();
  world.add_resource(GameState::new(tiles));
  world.add_resource(tiles);
  world.add_resource(ui::UI::new());
  world.add_resource(WindowClosed(false));
  world.add_resource(keypress);

  let mut window_closed = false;

  let tablet = make_object_entity(&mut world, ICON_TABLET, "mobile device".to_string());
  let car = make_object_entity(&mut world, ICON_HATCHBACK, "car".to_string());
  let mut dispatcher = DispatcherBuilder::new()
    .with(DrawIcon, "draw_icon", &[])
    .with(Describe, "describe", &["draw_icon"])
    .with_thread_local(display)
    .build();

  /*
  player.set_pos(Coord{x: cx, y: cy});
  player.character.set_body_layout(body_layout::humanoid());
  player.character.set_ch(ICON_MALE);

  entities.push(Box::new(make_computer()));
  for _ in 0..3 {
    entities.push(Box::new(make_bug()));
  }
  */

  // Compute the FOV starting from the coordinates 20,20. Where we'll put the '@'
  // Use a max_radius of 10 and light the walls.
  /* state.map.compute_fov(20,20, TORCH_RADIUS, true, FovAlgorithm::Basic);

  interface.open_menu(
    ui::Chain::new(vec![
        Box::new(ui::Notification::new(
          format!("SCRAPS: Bug Hunter"), format!("Your task, should you choose to accept it, is to catch bugs."),
        )),
        Box::new(ui::Notification::new(
          format!("Start Game"),
          format!("Find and catch the bugs!"),
        )),
    ])
  );
  */

  while !window_closed {
    // game success state
    

    dispatcher.dispatch(&mut world.res);
    world.maintain();

    window_closed = world.read_resource::<WindowClosed>().clone().0;

    /*display.draw(&mut world);
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
        if player.handle_input(&keypress, &map) {
          // only implement system-level keys and process ticks 
          // when player is not doing something
          match keypress {
            Key { code: Escape, .. } => break,
            _ => {}
          }
          handle_player_interact(&mut state, &mut interface, &mut player, &mut entities);
          state.tick();
          player.tick(&state);
          for entity in entities.iter_mut() {
            entity.tick(&state);
          }
        } else { 
        }
      }
    }
    */
  }
}
