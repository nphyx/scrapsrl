use tcod::{Console, RootConsole, FontLayout, FontType, BackgroundFlag, TextAlignment};
use tcod::system::*;
use tcod::colors::{Color, lerp};
use tcod::map::FovAlgorithm;
use super::util::{clamp, distance};
use super::entity::{Entity, EntityCollection, Player, Coord};
use super::game_state::GameState;
use super::util::colors::*;
use super::ui::UI;

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
  pub root: RootConsole
}

impl Display {
  pub fn new() -> Display {
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

    return Display{root}
  }
  pub fn draw(&mut self, state: &mut GameState, interface: &mut UI, player: &Player, entities: &EntityCollection) {
    state.map.compute_fov(player.pos().x, player.pos().y, SCREEN_WIDTH, true, FovAlgorithm::Basic);
    let light = Color::new(255, 240, 128);
    let ambient = Color::new(0, 6, 18);
    // TODO calculate relative contrast and maintain for out-of-vis objects
    let bg_gray = Color::new(8, 8, 8);
    let fg_gray = Color::new(24, 24, 24);

    self.root.clear();
    // Compute the FOV starting from the coordinates 20,20. Where we'll put the '@'
    // Use a max_radius of 10 and light the walls.

    for (coord, tile) in state.tiles.map.iter() {
      self.root.put_char_ex(coord.x, coord.y, tile.ch, tile.fg, tile.bg);
    }

    for entity in entities.iter() {
      if state.map.is_in_fov(entity.pos().x, entity.pos().y) {
        entity.draw(&mut self.root);
      }
    }

    // TODO compute time of day adjustment, sunset gradient, and moon phase :D
    let time_of_day_rel = 0.8;

    // lighting pass SUPER SLOW
    for x in 0..MAP_WIDTH {
      for y in 0..MAP_HEIGHT {
        let orig_fg = self.root.get_char_foreground(x, y);
        let orig_bg = self.root.get_char_background(x, y);
        let mut fg = orig_fg.clone();
        let mut bg = orig_bg.clone();
        let dist = distance(player.pos(), Coord{x, y});

        let rel_dist = clamp(
          0.0,
          1.0,
          dist.powf(1.25) / (MAP_WIDTH as f32)
        ).sqrt();
        let blend = lerp(light, ambient, rel_dist);
        if state.map.is_in_fov(x, y) {
          bg = soft_light(&soft_light(&bg, &blend), &blend); 
          fg = soft_light(&soft_light(&fg, &blend), &blend);
          fg = lerp(fg, lerp(orig_fg, color_dodge(&orig_fg, &light), 0.15), time_of_day_rel);
          bg = lerp(bg, lerp(orig_bg, color_dodge(&orig_bg, &light), 0.1), time_of_day_rel);
        } else {
          fg = screen(&lerp(fg, fg_gray, rel_dist), &ambient);
          bg = overlay(&lerp(bg, bg_gray, rel_dist), &ambient);
          fg = lerp(fg, desaturate(&fg), time_of_day_rel);
          bg = lerp(bg, desaturate(&bg), time_of_day_rel);
        }
        // fg = screen(&fg, &ambient);
        // bg = screen(&bg, &ambient);
        self.root.set_char_foreground(x, y, fg);
        self.root.set_char_background(x, y, bg, BackgroundFlag::Set);
      }
    }

    player.draw(&mut self.root);
    interface.draw(&self.root, player, state, entities);
    self.root.set_alignment(TextAlignment::Right);
    self.root.print_rect(SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1, 9, 1, format!("fps: {}", get_fps()));
    self.root.flush();
  }
}
