use tcod::{Console, RootConsole, FontLayout, FontType, BackgroundFlag, Map,
  input::KeyPressFlags, input::{Key, KeyCode::*}, TextAlignment};
use tcod::console::Root;
use tcod::map::FovAlgorithm;
use super::component::*;
use super::resource::*;
use super::util::colors::*;
use super::util::{clamp, distance};

mod ui;

type TColor = tcod::colors::Color;

use super::constants::{
  MAP_WIDTH,
  MAP_HEIGHT,
  SCREEN_WIDTH,
  SCREEN_HEIGHT,
  DEFAULT_BG,
  DEFAULT_FG};

pub struct Display {
  pub root: Root,
  pub map: Map,
}


impl Display {
  /// initialize the display
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

    use tcod::system::set_fps;
    set_fps(60);

    Display{
      root,
      map: Map::new(MAP_WIDTH, MAP_HEIGHT)
    }
  }
}

use specs::{System, Read, Write, ReadStorage, Join};
impl<'a> System<'a> for Display {
  type SystemData  = (
    // I'll take one of everything
    ReadStorage<'a, Player>,
    ReadStorage<'a, Cursor>,
    ReadStorage<'a, Character>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Icon>,
    ReadStorage<'a, Colors>,
    ReadStorage<'a, Description>,

    Write<'a, GameState>,
    Read<'a, AreaMap>,
    Read<'a, UIQueue>,
    Write<'a, UserInput>
  );

  /// the drawing the game megafunction, what a disaster area
  fn run(
      &mut self,
      data: Self::SystemData) {
    let (
      players,
      cursors,
      characters,
      positions,
      icons,
      colors,
      descriptions,

      mut state,
      map,
      ui_queue,
      mut keypress,
    ) = data;

    self.root.set_fullscreen(state.fullscreen);

    // wipe screen and prepare for new draw
    self.root.clear();
    self.root.set_default_background(DEFAULT_BG);
    self.root.set_default_foreground(DEFAULT_FG);

    let dot_dot_dot = (((state.frame / 15) % 4)) as usize;
    match state.stage {
      GameStage::LoadingAssets => {
        self.root.set_alignment(TextAlignment::Left);
        self.root.print_rect(
          SCREEN_WIDTH / 2 - 9,
          SCREEN_HEIGHT / 2,
          SCREEN_WIDTH,
          1,
          format!("Loading assets{:.<1$}", "", dot_dot_dot)
        );
        self.root.flush();
        self.accept_input(&mut keypress, &mut state);
        return;
      },
      GameStage::Initializing => {
        self.root.set_alignment(TextAlignment::Left);
        self.root.print_rect(
          SCREEN_WIDTH / 2 - 7,
          SCREEN_HEIGHT / 2,
          SCREEN_WIDTH,
          1,
          format!("Initializing{:.<1$}", "", dot_dot_dot)
        );
        self.root.flush();
        self.accept_input(&mut keypress, &mut state);
        return;
      },
      _ => {/* other stages all follow main draw setup below */}
    }

    let mut cursor_pos: Position = Position::default();
    let mut player_pos: Position = Position::default();
    let mut has_cursor: bool = false;

    for (pos, character, _player)
    in (&positions, &characters, &players).join() {
      ui::draw_stats(&self.root, character);
      ui::draw_status_bar(&self.root, character, &state);
      player_pos.x = pos.x;
      player_pos.y = pos.y;
    }

    // find the cursor position
    for (pos, _cursor) in (&positions, &cursors).join() {
      cursor_pos.x = pos.x;
      cursor_pos.y = pos.y;
      has_cursor = true;
    }

    // find an entity under the cursor, if it exists
    if has_cursor && self.map.is_in_fov(cursor_pos.x, cursor_pos.y) {
      let mut found_entity = false;
      for(pos, icon, color, desc)
      in (&positions, &icons, &colors, &descriptions).join() {
        if pos.x == cursor_pos.x && pos.y == cursor_pos.y {
          ui::draw_entity_info(&self.root, icon, color, desc);
          found_entity = true;
        }
      }
      if !found_entity {
        let tile = map.get(cursor_pos);
        match tile {
          Some(tile) => { ui::draw_tile_info(&self.root, tile); },
          _ => {}
        }
      }
    }

    // update fov map before computing fov
    for (pos, tile) in map.iter() {
      self.map.set(pos.x, pos.y, tile.transparent, tile.walkable)
    }

    // Compute the FOV
    self.map.compute_fov(player_pos.x, player_pos.y, SCREEN_WIDTH, true, FovAlgorithm::Basic);

    // draw all tiles
    for (pos, tile) in map.iter() {
      self.root.put_char_ex(pos.x, pos.y, tile.icon, tile.fg, tile.bg);
    }

    // draw all npcs, also snag the one under the cursor if applicable
    for(pos, icon, color, ..) in (&positions, &icons, &colors, !&players).join() {
      if self.map.is_in_fov(pos.x, pos.y) {
        self.root.put_char(pos.x, pos.y, icon.ch, BackgroundFlag::None);
        self.root.set_char_foreground(pos.x, pos.y, TColor::from(color.fg));
      }
    }

    // TODO compute time of day adjustment, sunset gradient, and moon phase :D
    let time_of_day_rel = state.world_time_relative();

    let light = Color::new(255, 240, 128);
    let ambient = Color::new(0, 6, 18);

    // TODO calculate relative contrast and maintain for out-of-vis objects
    let bg_gray = Color::new(8, 8, 8);
    let fg_gray = Color::new(24, 24, 24);

    // lighting pass SUPER SLOW
    for (pos, _) in map.iter() {
      let orig_fg = Color::from(self.root.get_char_foreground(pos.x, pos.y));
      let orig_bg = Color::from(self.root.get_char_background(pos.x, pos.y));
      let mut fg = orig_fg;
      let mut bg = orig_bg;
      let dist = distance(player_pos, pos);

      // this figures out the radius of the player-emitted light area
      let rel_dist = clamp(
        0.0,
        1.0,
        dist.powf(1.25) / (MAP_WIDTH as f32)
      ).sqrt();
      // ignore the trigonometric man behind the curtain
      let frame = (state.frame % 360) as f32 / 8.0;
      let flicker_mod = frame.cos() * 0.005;

      let blend = lerp(light, ambient, clamp(0.0, 1.0, rel_dist - flicker_mod));

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
      self.root.set_char_foreground(pos.x, pos.y, TColor::from(fg));
      self.root.set_char_background(pos.x, pos.y, TColor::from(bg), BackgroundFlag::Set);
    }

    // draw player last, make sure it ends up on top
    for (pos, icon, color, ..) in (&positions, &icons, &colors, &players).join() {
      self.root.put_char(pos.x, pos.y, icon.ch, BackgroundFlag::None);
      self.root.set_char_foreground(pos.x, pos.y, TColor::from(color.fg))
    }

    for (pos, ..) in (&positions, &cursors).join() {
      self.root.set_char_background(pos.x, pos.y, TColor::new(110, 180, 144), BackgroundFlag::Overlay);
    }

    /*
    self.root.set_alignment(TextAlignment::Right);
    self.root.print_rect(SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1, 12, 1, format!("time: {:.*}", 2, state.world_time_relative()));
    */

    match ui_queue.get() {
      Some(widget) => { ui::draw_centered_dialog(&self.root, widget); },
      _ => {}
    }

    self.root.flush();


    self.accept_input(&mut keypress, &mut state);
  }
}

impl Display {
  fn accept_input(&mut self, keypress: &mut UserInput, state: &mut GameState) {
    let key_input = self.root.check_for_keypress(KeyPressFlags::all());
    match key_input {
      // we don't match modifier keys as an input
      Some(Key { code: Control, .. }) |
      Some(Key { code: Alt, .. }) |
      Some(Key { code: Shift, .. }) => { },
      // only match when pressed = on, tcod fires on down + up
      Some(Key { pressed: true, ..}) => {
        keypress.set(key_input);
        match keypress.get() {
          Some(_) => { // flush the rest of the key queue manually
            loop {
              match self.root.check_for_keypress(KeyPressFlags::all()) {
                None => { break; },
                _ => {}
              }
            }
          },
          _ => {}
        }
      },
      _ => {}
    }
    state.close_game = self.root.window_closed() || state.close_game;
  }
}
