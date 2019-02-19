#[macro_use]
extern crate specs_derive;
use specs::{Builder, DispatcherBuilder, RunNow, World as Game};

mod component;
mod constants;
mod display;
mod resource;
mod system;
mod util;
use self::component::Position;
use self::component::*;
use self::constants::{MAP_HEIGHT, MAP_WIDTH};
use self::display::Display;
use self::resource::*;
use self::system::input::*;
use self::system::*;

fn main() {
    let mut game = Game::new();
    component::init(&mut game);

    let ui_queue = UIQueue::default();

    let mut maps = AreaMaps::default();
    let mut collisions = CollisionMaps::default();
    maps.init(Region::new(0, 0), constants::CHUNK_RADIUS);
    collisions.init(Region::new(0, 0), constants::CHUNK_RADIUS);

    game.add_resource(GameState::default());
    game.add_resource(WorldState::default());
    game.add_resource(UserInput::default());
    game.add_resource(maps);
    game.add_resource(collisions);
    game.add_resource(ui_queue);
    game.add_resource(Assets::default());

    let mut window_closed = false;

    // set up player
    game.create_entity()
        .with(Player)
        .with(Solid)
        .with(Position {
            x: MAP_WIDTH / 2,
            y: MAP_HEIGHT / 2,
        })
        .with(Region { x: 0, y: 0 })
        .with(MovePlan { x: 0, y: 0 })
        .with(IconRef::new("person_male"))
        .with(Colors {
            fg: Color::new(255, 255, 255),
            bg: Color::new(255, 255, 255),
        })
        .with(Character::default())
        .build();

    let mut display = Display::new();
    let mut dispatcher = DispatcherBuilder::new()
        // do game state maintenance. sadly not really taking advantage
        // of parallelism but maybe eventually it can
        .with(AssetLoader::default(), "", &[])
        .with(PreTick, "", &[])
        // handle user input first
        .with(UIInput, "ui_input", &[])
        .with(SystemInput, "system_input", &["ui_input"])
        .with(CursorInput, "cursor_input", &["system_input"])
        .with(PlayerInput, "player_input", &["cursor_input"])
        .with(FallthroughInput, "fallthrough_input", &["player_input"])
        .with(WorldGen, "world_gen", &["fallthrough_input"])
        .with(
            MapGenerator::new(MAP_WIDTH, MAP_HEIGHT),
            "map_gen",
            &["fallthrough_input", "world_gen"],
        )
        .with(CollisionSystem, "collision_system", &["map_gen"])
        // let AI decide what it wants to do
        .with(AI, "ai", &["collision_system"])
        // process AI and player actions
        .with(
            RegionSystem,
            "region",
            &["ai", "player_input", "collision_system"],
        )
        .with(
            BumpInteract,
            "bump_interact",
            &["region", "collision_system"],
        )
        .with(Movement, "movement", &["region", "bump_interact"])
        .with(PostTick, "", &["movement"])
        .with(Notify, "interact_notify", &[])
        .with(Stage, "game_stage", &[])
        .build();

    dispatcher.setup(&mut game.res);

    while !window_closed {
        display.run_now(&game.res);
        dispatcher.dispatch(&game.res);
        game.maintain();
        window_closed = game.read_resource::<GameState>().close_game;
    }
}
