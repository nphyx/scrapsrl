use tcod::{Console, RootConsole, FontLayout, FontType};
use tcod::colors::{Color, lerp};
use super::util::{clamp, distance};
use super::entity::{Entity, EntityCollection, Player, Coord};
use super::game_state::GameState;
use super::ui::UI;

use super::constants::{
  TORCH_RADIUS,
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
      .font("monofur-nf.png", FontLayout::AsciiInRow)
      .font_type(FontType::Greyscale)
      .font_dimensions(102,636)
      .size(SCREEN_WIDTH, SCREEN_HEIGHT)
      .title("SCRAPS: Bug Hunter")
      .init();

    root.set_default_background(DEFAULT_BG);
    root.set_default_foreground(DEFAULT_FG);
    root.clear();

    return Display{root}
  }
  pub fn draw(&mut self, state: &GameState, interface: &mut UI, player: &Player, entities: &EntityCollection) {
    let pc = &player.character;
    let light = Color::new(200, 180, 50);
    let dark = Color::new(0, 6, 18);
    let ground = DEFAULT_BG; //Color::new(0, 40, 25);

    self.root.clear();
    // Compute the FOV starting from the coordinates 20,20. Where we'll put the '@'
    // Use a max_radius of 10 and light the walls.

    for ((px, py), tile) in &state.tiles {
      let visible = state.map.is_in_fov(*px, *py);
      let dist = clamp(
        0.0,
        1.0,
        distance(pc.pos(), Coord{x: *px, y: *py})
        / TORCH_RADIUS as f32);
      let fg: Color;
      let bg: Color;
      let blend = lerp(light, dark, dist);
      if visible && (dist < TORCH_RADIUS as f32) {
        bg = lerp(ground, blend, 0.3);
        fg = lerp(tile.color, blend, 0.7);
      } else if visible {
        bg = lerp(ground, blend, 0.5);
        fg = lerp(tile.color, blend, 0.5);
      } else {
        bg = lerp(ground, dark, 0.5);
        fg = lerp(tile.color, dark, 0.7);
      }
      self.root.put_char_ex(*px, *py, tile.ch, fg, bg);
    }

    for entity in entities.iter() {
      if state.map.is_in_fov(entity.pos().x, entity.pos().y) {
        entity.draw(&mut self.root);
      }
    }

    player.draw(&mut self.root);
    interface.draw(&self.root, player, state, entities);
    self.root.flush();
  }
}
