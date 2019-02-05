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

mod ui;
mod game_state;
mod constants;
mod util;
mod display;
mod cursor;
mod component;
mod system;
mod resource;
use self::component::Position;
use self::display::Display;
use self::resource::*;
use self::system::*;
use self::util::icons::*;
// use self::entity::{Coord, Entity, Character, body_layout, Object, Player, NPC, EntityCollection, EntityInteraction, make_object_entity};
use self::game_state::GameState;
use self::component::*;
use self::constants::{
  MAP_WIDTH,
  MAP_HEIGHT};

fn make_bug(world: &mut World) {
  let mut rng = rand::thread_rng();
  world.create_entity()
    .with(Solid)
    .with(Character::blank())
    .with(Icon{ch: ICON_BUG})
    .with(Position{
      x: rng.gen_range(0, MAP_WIDTH),
      y: rng.gen_range(0, MAP_HEIGHT)})
    .with(Colors{
      fg: Color{r: 32, g: 128, b: 225},
      bg: Color{r: 32, g: 128, b: 225}})
  //bug.set_body_layout(body_layout::insectoid());
    .with(Description{
      short: "a shockroach".to_string(),
      long: "A housecat-sized cockroach. Electric sparks arc between its antenna.".to_string()})
  .build();
  /*
  let mut npc_wrap = NPC::new(bug);
  npc_wrap.set_notification(
    Notification::new(
      format!("Success"),
      format!("Got 'em!")));
  npc_wrap
  */
}

use specs::{Builder};
fn make_computer(world: &mut World) {
  let mut rng = rand::thread_rng();
  world.create_entity()
    .with(Position{x: rng.gen_range(0, MAP_WIDTH), y: rng.gen_range(0, MAP_HEIGHT)})
    .with(Icon{ch: ICON_OLD_COMPUTER})
    .with(Colors{
      fg: Color::new(130,130,127),
      bg: Color::new(35,35,32)
    })
    .with(Description{
      short: "an old computer".to_string(),
      long: "An old-world electronic device. Looks like it's still working.".to_string()
    })
    .build();
    /*
  computer.set_pos(Coord);
  computer.set_notification(
    Notification::new(
      format!(" {} ", ICON_OLD_COMPUTER).to_string(),
      "Bleep, bloop!".to_string())
  );
  computer.set_desc();
  return computer;
  */
}

/*
fn handle_player_interact(state: &mut GameState, interface: &mut ui::UI, player: &mut Player, entities: &mut EntityCollection) {
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
}
*/

fn main() {
  // let cx = MAP_WIDTH / 2;
  // let cy = MAP_HEIGHT / 2;
  // let mut fullscreen = false;
  // let mut interface = ui::UI::new();
  // let mut player = Player::new(Character::blank());
  // let mut entities = EntityCollection::new();

  let mut world = World::new();
  // let (map, tiles) = mapgen::generate(MAP_WIDTH, MAP_HEIGHT);
  let display = Display::new();
  let keypress = Key::default();
  component::init(&mut world);
  world.add_resource(GameState::new());
  world.add_resource(ui::UI::new());
  world.add_resource(self::resource::MapGenRequested(true));
  world.add_resource(self::resource::WindowClosed(false));
  world.add_resource(keypress);

  let mut window_closed = false;

  // set up player
  world.create_entity()
    .with(Position{x:MAP_WIDTH/2, y:MAP_HEIGHT/2})
    .with(Icon{ch:ICON_MALE})
    .with(Colors{
      fg: Color::new(255, 255, 255),
      bg: Color::new(255, 255, 255)
    })
    .with(Player)
    .with(Character::default())
    .build();
    

  world.create_entity()
    .with(Position{x:0, y:0})
    .with(Icon{ch:ICON_TABLET})
    .with(Colors{
      fg: Color{r: 128, g: 128, b:128},
      bg: Color{r: 128, g: 128, b:128}
    })
    .with(Description{
      short: "a mobile device".to_string(),
      long: "This device was used to track the activity of serfs.".to_string()
    })
    .build();

  world.create_entity()
    .with(Position{x:0, y:0})
    .with(Icon{ch:ICON_HATCHBACK})
    .with(Colors{
      fg: Color{r: 128, g: 128, b:128},
      bg: Color{r: 128, g: 128, b:128}
    })
    .with(Description{
      short: "a hatchback".to_string(),
      long: "A kind of vehicle with a door on the back.".to_string()
    })
    .build();

  let mut dispatcher = DispatcherBuilder::new()
    .with(MapGenerator::new(MAP_WIDTH, MAP_HEIGHT), "map_gen", &[])
    .with(DrawIcon, "draw_icon", &["map_gen"])
    .with(Describe, "describe", &["draw_icon", "map_gen"])
    .with_thread_local(display)
    .build();

  dispatcher.setup(&mut world.res);

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
