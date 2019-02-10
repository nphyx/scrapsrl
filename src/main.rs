extern crate rand;
extern crate tcod;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use rand::prelude::*;
use specs::{World, DispatcherBuilder, RunNow};

mod component;
mod constants;
mod display;
mod resource;
mod system;
mod util;
use self::component::Position;
use self::display::Display;
use self::resource::*;
use self::system::*;
use self::system::input::*;
use self::util::icons::*;
use self::component::*;
use self::constants::{MAP_WIDTH, MAP_HEIGHT};

fn make_bug(world: &mut World) {
  let mut rng = rand::thread_rng();
  let x = rng.gen_range(0, MAP_WIDTH);
  let y = rng.gen_range(0, MAP_HEIGHT);
  let template = EntityTemplate::create()
    .brain()
    .solid()
    .character(Character::blank())
    .icon(ICON_BUG)
    .colors(Color{r: 32, g: 128, b: 225}, Color{r: 32, g: 128, b: 225})
    .description("a shockroach", "A housecat-sized cockroach. Electric sparks arc between its antenna.")
    .build();

  template.to_world(world)
    .with(Position{x, y})
    .with(MovePlan{x: 0, y: 0})
    .build();
    /*
  world.create_entity()
    .with(Solid)
    .with(Character::blank())
    .with(Icon{ch: ICON_BUG})
    .with(Position{x, y})
    .with(MovePlan{x: 0, y: 0})
    .with(Colors{
      fg: Color{r: 32, g: 128, b: 225},
      bg: Color{r: 32, g: 128, b: 225}})
    .with(Description{
      short: "a shockroach".to_string(),
      long: "A housecat-sized cockroach. Electric sparks arc between its antenna.".to_string()})
  .build();
  */
}

use specs::{Builder};
fn make_computer(world: &mut World) {
  world.create_entity()
    .with(Position{
      x: MAP_WIDTH / 2 + 1,
      y: MAP_HEIGHT / 2 + 1
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
    .with(NotificationInteraction{
      header: ICON_OLD_COMPUTER.to_string(),
      body: "Bleep, bloop!".to_string()
    })
    .build();
}

fn main() {
  let mut rng = rand::thread_rng();
  let mut world = World::new();
  let mut state = GameState::default();
  state.world_seed = rng.gen_range(0, std::u32::MAX);
  state.map_gen_queued = true;
  component::init(&mut world);

  let ui_queue = UIQueue::default();

  world.add_resource(state);
  world.add_resource(UserInput::default());
  world.add_resource(AreaMap::default());
  world.add_resource(ui_queue);
  world.add_resource(Templates::default());

  let mut window_closed = false;

  // set up player
  world.create_entity()
    .with(Player)
    .with(Solid)
    .with(Position{x:MAP_WIDTH/2, y:MAP_HEIGHT/2})
    .with(MovePlan{x:0, y:0})
    .with(Icon{ch:ICON_MALE})
    .with(Colors{
      fg: Color::new(255, 255, 255),
      bg: Color::new(255, 255, 255)
    })
    .with(Character::default())
    .build();

  make_bug(&mut world);
  make_computer(&mut world);
    

  world.create_entity()
    .with(Position{
      x: rng.gen_range(0, MAP_WIDTH),
      y: rng.gen_range(0, MAP_HEIGHT)})
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
    .with(Solid)
    .with(Position{
      x: rng.gen_range(0, MAP_WIDTH),
      y: rng.gen_range(0, MAP_HEIGHT)})
    .with(Icon{ch:ICON_HATCHBACK})
    .with(Colors{
      fg: Color{r: 128, g: 128, b:128},
      bg: Color{r: 128, g: 128, b:128}
    })
    .with(Description{
      short: "a hatchback".to_string(),
      long: "A kind of vehicle with a door on the back.".to_string()
    })
    .with(NotificationInteraction{
      header: ICON_HATCHBACK.to_string(),
      body: "There's nothing inside.".to_string()
    })
    .build();

  
  let mut display = Display::new();
  let mut dispatcher = DispatcherBuilder::new()
    // do game state maintenance. sadly not really taking advantage
    // of parallelism but maybe eventually it can
    .with(PreTick, "", &[])
    // handle user input first
    .with(UIInput, "ui_input", &[])
    .with(SystemInput, "system_input", &["ui_input"])
    .with(CursorInput, "cursor_input", &["system_input"])
    .with(PlayerInput, "player_input", &["cursor_input"])
    .with(FallthroughInput, "fallthrough_input", &["player_input"])
    .with(MapGenerator::new(MAP_WIDTH, MAP_HEIGHT), "map_gen", &["fallthrough_input"])
    .with(CollisionMap, "collision_map", &["map_gen"])
    // let AI decide what it wants to do
    .with(AI, "ai", &["collision_map"])
    // process AI and player actions
    .with(AreaChange, "area_change", &["ai", "player_input", "collision_map"])
    .with(BumpInteract, "bump_interact", &["collision_map"])
    .with(Movement, "movement", &["area_change", "bump_interact"])
    .with(PostTick, "", &["movement"])
    .with(Notify, "interact_notify", &[])
    .with(Stage, "game_stage", &[])
    .build();

  dispatcher.setup(&mut world.res);

  while !window_closed {
    display.run_now(&mut world.res);
    dispatcher.dispatch(&mut world.res);
    world.maintain();
    window_closed = world.read_resource::<GameState>().close_game;
  }
}
