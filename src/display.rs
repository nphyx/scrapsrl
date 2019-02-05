use tcod::{Console, RootConsole, FontLayout, FontType, BackgroundFlag, TextAlignment, Map};
use tcod::colors::{lerp, Color};
use tcod::map::FovAlgorithm;
use super::component::*;
use super::game_state::GameState;
use super::util::colors::*;
use super::util::{clamp, distance};
use super::WindowClosed;

use super::constants::{
  MAP_WIDTH,
  MAP_HEIGHT,
  SCREEN_WIDTH,
  SCREEN_HEIGHT,
  DEFAULT_BG,
  DEFAULT_FG};

/*
pub trait DrawSelf {
  fn draw(&self, console: &mut Console);
  fn draw_at(&self, console: &mut Console, x: i32, y: i32);
}
*/

pub struct Display {
  pub root: RootConsole,
  pub map: Map,
}


impl<'a> Display {
  /// initialize the display
  pub fn new() -> Display {
    let mut root = RootConsole::initializer()
      .font("monofur-nf-24-square.png", FontLayout::AsciiInRow)
      .font_type(FontType::Greyscale)
      .font_dimensions(256,256)
      .size(SCREEN_WIDTH, SCREEN_HEIGHT)
      .title("SCRAPS: Bug Hunter")
      .init();
    let map = Map::new(MAP_WIDTH, MAP_HEIGHT);

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
    ReadStorage<'a, Opaque>,
    ReadStorage<'a, Solid>,
    ReadStorage<'a, Icon>,
    ReadStorage<'a, Colors>,
    ReadStorage<'a, Tile>,
    Read<'a, GameState>,
    Write<'a, WindowClosed>,
    Write<'a, UserInput>
  );

  fn run(
      &mut self,
      (
        players,
        characters,
        positions,
        opaques,
        solids,
        icons,
        colors,
        tiles,
        state,
        mut window_closed,
        mut keypress
      ): Self::SystemData) {

    let light = Color::new(255, 240, 128);
    let ambient = Color::new(0, 6, 18);

    // TODO calculate relative contrast and maintain for out-of-vis objects
    let bg_gray = Color::new(8, 8, 8);
    let fg_gray = Color::new(24, 24, 24);

    let mut player_pos: Position = Position::default();
    for (pos, _player) in (&positions, &players).join() {
      player_pos = pos.clone();
    }

    // update map before computing fov
    for (pos, ..) in (&positions, &opaques, &solids).join() {
      self.map.set(pos.x, pos.y, false, false);
    }

    for (pos, ..) in (&positions, !&opaques, &solids).join() {
      self.map.set(pos.x, pos.y, true, false);
    }

    for (pos, ..) in (&positions, &opaques, !&solids).join() {
      self.map.set(pos.x, pos.y, false, true);
    }

    for (pos, ..) in (&positions, !&opaques, !&solids).join() {
      self.map.set(pos.x, pos.y, true, true);
    }

    // Compute the FOV
    self.map.compute_fov(player_pos.x, player_pos.y, SCREEN_WIDTH, true, FovAlgorithm::Basic);

    // prep map
    self.root.clear();

    // draw all tiles
    for(pos, icon, colors, _tile) in (&positions, &icons, &colors, &tiles).join() {
      self.root.put_char_ex(pos.x, pos.y, icon.ch, colors.fg, colors.bg);
    }

    // draw all npcs
    for(pos, icon, color, ..) in (&positions, &icons, &colors, !&players, !&tiles).join() {
      if self.map.is_in_fov(pos.x, pos.y) {
        self.root.put_char(pos.x, pos.y, icon.ch, BackgroundFlag::None);
        self.root.set_char_foreground(pos.x, pos.y, color.fg);
      }
    }

    // TODO compute time of day adjustment, sunset gradient, and moon phase :D
    let time_of_day_rel = 0.0; //state.world_time_relative();

    // lighting pass SUPER SLOW
    for pos  in (&positions).join() {
      let orig_fg = self.root.get_char_foreground(pos.x, pos.y);
      let orig_bg = self.root.get_char_background(pos.x, pos.y);
      let mut fg = orig_fg.clone();
      let mut bg = orig_bg.clone();
      let dist = distance(player_pos, *pos);

      let rel_dist = clamp(
        0.0,
        1.0,
        dist.powf(1.25) / (MAP_WIDTH as f32)
      ).sqrt();
      let blend = lerp(light, ambient, rel_dist);
      if self.map.is_in_fov(pos.x, pos.y) {
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
      self.root.set_char_foreground(pos.x, pos.y, fg);
      self.root.set_char_background(pos.x, pos.y, bg, BackgroundFlag::Set);
    }

    // draw player last, make sure it ends up on top
    for (pos, icon, color, ..) in (&positions, &icons, &colors, &players).join() {
      self.root.put_char(pos.x, pos.y, icon.ch, BackgroundFlag::None);
      self.root.set_char_foreground(pos.x, pos.y, color.fg)
    }
    // interface.draw(&self.root, player, state, entities);
    self.root.set_alignment(TextAlignment::Right);
    // self.root.print_rect(SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1, 12, 1, format!("time: {:.*}", 2, state.world_time_relative()));
    self.root.flush();

    keypress.key = Some(self.root.wait_for_keypress(true));
    *window_closed = WindowClosed(self.root.window_closed() || state.close_game);
  }
}
