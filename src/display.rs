use tcod::{Console, RootConsole, FontLayout, FontType, BackgroundFlag, TextAlignment, Map};
use tcod::colors::{lerp};
use tcod::input::Key;
use tcod::map::FovAlgorithm;
use super::util::{clamp, distance};
use super::entity::{Coord, Player, Character};
use super::component::{Position, Icon, Color};
use super::game_state::GameState;
use super::util::colors::*;
use super::WindowClosed;
use super::mapgen::Tiles;

use super::constants::{
  MAP_WIDTH,
  MAP_HEIGHT,
  SCREEN_WIDTH,
  SCREEN_HEIGHT,
  DEFAULT_BG,
  DEFAULT_FG};

pub trait DrawSelf {
  fn draw(&self, console: &mut Console);
  fn draw_at(&self, console: &mut Console, x: i32, y: i32);
}

pub struct Display {
  pub root: RootConsole,
  pub map: Map,
}


impl<'a> Display {
  pub fn new(map: Map) -> Display {
    let mut root = RootConsole::initializer()
      .font("monofur-nf-24-square.png", FontLayout::AsciiInRow)
      .font_type(FontType::Greyscale)
      .font_dimensions(256,256)
      .size(SCREEN_WIDTH, SCREEN_HEIGHT)
      .title("SCRAPS: Bug Hunter")
      .init();

    root.set_default_background(DEFAULT_BG);
    root.set_default_foreground(DEFAULT_FG);
    root.clear();

    return Display{root, map}
  }
}

use specs::{System, Read, Write, ReadStorage, Join};
impl<'a> System<'a> for Display {
  type SystemData  = (
    ReadStorage<'a, Player>,
    ReadStorage<'a, Character>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Icon>,
    ReadStorage<'a, Color>,
    Read<'a, GameState>,
    Read<'a, Tiles>,
    Write<'a, WindowClosed>,
    Write<'a, Key>
  );

  fn run(&mut self, (players, characters, positions, icons, colors, state, mut window_closed, mut keypress): Self::SystemData) {
    let mut player_pos: &Position = &Position{x:0, y:0};
    for (pos, player) in (&positions, &players).join() {
      player_pos = pos;
      self.map.compute_fov(pos.x, pos.y, SCREEN_WIDTH, true, FovAlgorithm::Basic);
    }
    let light = Color::new(255, 240, 128).to_tcod();
    let ambient = Color::new(0, 6, 18).to_tcod();

    // TODO calculate relative contrast and maintain for out-of-vis objects
    let bg_gray = Color::new(8, 8, 8).to_tcod();
    let fg_gray = Color::new(24, 24, 24).to_tcod();

    self.root.clear();
    // Compute the FOV starting from the coordinates 20,20. Where we'll put the '@'
    // Use a max_radius of 10 and light the walls.

    /*
    for (coord, tile) in state.tiles.map.iter() {
      self.root.put_char_ex(coord.x, coord.y, tile.ch, tile.fg, tile.bg);
    }
    */

    for(pos, icon, color, ..) in (&positions, &icons, &colors, !&players).join() {
      if self.map.is_in_fov(pos.x, pos.y) {
        self.root.put_char(pos.x, pos.y, icon.ch, BackgroundFlag::None);
        self.root.set_char_foreground(pos.x, pos.y, color.to_tcod())
      }
    }

    // TODO compute time of day adjustment, sunset gradient, and moon phase :D
    let time_of_day_rel = state.world_time_relative();
    println!("{}", time_of_day_rel);

    // lighting pass SUPER SLOW
    for x in 0..MAP_WIDTH {
      for y in 0..MAP_HEIGHT {
        let orig_fg = self.root.get_char_foreground(x, y);
        let orig_bg = self.root.get_char_background(x, y);
        let mut fg = orig_fg.clone();
        let mut bg = orig_bg.clone();
        let dist = distance(Coord{x: player_pos.x, y: player_pos.y}, Coord{x, y});

        let rel_dist = clamp(
          0.0,
          1.0,
          dist.powf(1.25) / (MAP_WIDTH as f32)
        ).sqrt();
        let blend = lerp(light, ambient, rel_dist);
        if self.map.is_in_fov(x, y) {
          bg = soft_light(&soft_light(&bg, &blend), &blend); 
          fg = soft_light(&soft_light(&fg, &blend), &blend);
          fg = lerp(fg, lerp(orig_fg, color_dodge(&orig_fg, &light), 0.15), time_of_day_rel);
          bg = lerp(bg, lerp(orig_bg, color_dodge(&orig_bg, &light), 0.1), time_of_day_rel);
        } else {
          fg = screen(&lerp(fg, fg_gray, rel_dist), &ambient);
          bg = overlay(&lerp(bg, bg_gray, rel_dist), &ambient);
          fg = lerp(fg, lerp(orig_fg, desaturate(&orig_fg), 0.25), time_of_day_rel);
          bg = lerp(bg, lerp(orig_bg, desaturate(&orig_bg), 0.25), time_of_day_rel);
        }
        // fg = screen(&fg, &ambient);
        // bg = screen(&bg, &ambient);
        self.root.set_char_foreground(x, y, fg);
        self.root.set_char_background(x, y, bg, BackgroundFlag::Set);
      }
    }

    for (pos, icon, color, ..) in (&positions, &icons, &colors, &players).join() {
      self.root.put_char(pos.x, pos.y, icon.ch, BackgroundFlag::None);
      self.root.set_char_foreground(pos.x, pos.y, color.to_tcod())
    }
    // interface.draw(&self.root, player, state, entities);
    self.root.set_alignment(TextAlignment::Right);
    // self.root.print_rect(SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1, 12, 1, format!("time: {:.*}", 2, state.world_time_relative()));
    self.root.flush();

    *keypress = self.root.wait_for_keypress(true);
    *window_closed = WindowClosed(self.root.window_closed());
  }
}
