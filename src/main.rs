extern crate rand;
extern crate tcod;
extern crate specs;
extern crate frappe;
#[macro_use]
extern crate specs_derive;

use tcod::colors::Color;
use rand::prelude::*;
use specs::{World, DispatcherBuilder, RunNow};

mod area_map;
mod component;
mod constants;
mod cursor;
mod game_state;
mod display;
mod resource;
mod system;
mod ui;
mod util;
use self::component::Position;
use self::display::Display;
use self::resource::*;
use self::system::*;
use self::util::icons::*;
use self::game_state::GameState;
use self::component::*;
use self::constants::{
  MAP_WIDTH,
  MAP_HEIGHT};

fn make_bug(world: &mut World) {
  let mut rng = rand::thread_rng();
  world.create_entity()
    .with(Solid)
    .with(AIBrain::default())
    .with(Character::blank())
    .with(Icon{ch: ICON_BUG})
    .with(Position{
      x: rng.gen_range(0, MAP_WIDTH),
      y: rng.gen_range(0, MAP_HEIGHT)})
    .with(MovePlan{
      x: rng.gen_range(0, MAP_WIDTH),
      y: rng.gen_range(0, MAP_HEIGHT)})
    .with(Colors{
      fg: Color{r: 32, g: 128, b: 225},
      bg: Color{r: 32, g: 128, b: 225}})
    .with(Description{
      short: "a shockroach".to_string(),
      long: "A housecat-sized cockroach. Electric sparks arc between its antenna.".to_string()})
  .build();
}

use specs::{Builder};
fn make_computer(world: &mut World) {
  // let mut rng = rand::thread_rng();
  world.create_entity()
    .with(Position{
      x: MAP_WIDTH / 2 + 1, //rng.gen_range(0, MAP_WIDTH), 
      y: MAP_HEIGHT / 2 + 1 // rng.gen_range(0, MAP_HEIGHT)})
    })
    .with(Icon{ch: ICON_OLD_COMPUTER})
    .with(Solid)
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

  let mut world = World::new();
  component::init(&mut world);
  world.add_resource(GameState::new());
  world.add_resource(ui::UI::new());
  world.add_resource(self::resource::MapGenRequested(true));
  world.add_resource(self::resource::WindowClosed(false));
  world.add_resource(UserInput{key: None});
  world.add_resource(area_map::AreaMap::default());

  let mut window_closed = false;

  // set up player
  world.create_entity()
    .with(Player)
    .with(Position{x:MAP_WIDTH/2, y:MAP_HEIGHT/2})
    .with(MovePlan{x:MAP_WIDTH/2, y:MAP_HEIGHT/2})
    .with(Icon{ch:ICON_MALE})
    .with(Colors{
      fg: Color::new(255, 255, 255),
      bg: Color::new(255, 255, 255)
    })
    .with(Character::default())
    .build();

  make_bug(&mut world);
  make_computer(&mut world);
    

  /*
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
    */
  /*
  */

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

  let mut display = Display::new();
  let mut dispatcher = DispatcherBuilder::new()
    .with(Time, "", &[])
    .with(MapGenerator::new(MAP_WIDTH, MAP_HEIGHT), "map_gen", &[])
    .with(CollisionMap, "collision_map", &["map_gen"])
    .with(HandleSystemInput, "system_input", &["map_gen"])
    .with(HandlePlayerInput, "player_input", &["system_input", "collision_map"])
    .with(HandleFallthroughInput, "fallthrough_input", &["player_input"])
    .with(AI, "ai", &["collision_map"])
    .with(Movement, "movement", &["ai", "player_input", "collision_map"])
    .build();

  dispatcher.setup(&mut world.res);

  while !window_closed {
    dispatcher.dispatch(&mut world.res);
    world.maintain();
    window_closed = 
      world.read_resource::<WindowClosed>().clone().0 ||
      world.read_resource::<GameState>().close_game;
    display.run_now(&mut world.res);
  }
}
