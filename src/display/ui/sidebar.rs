use tcod::{Console, TextAlignment};
use crate::component::Character;
use crate::game_state::GameState;
use crate::constants::{SIDEBAR_WIDTH};
use crate::util::icons::*;
use super::util::*;

pub fn draw(mut console: &Console, pc: &Character, state: &GameState) {
  reset_colors(&console);
  console.set_alignment(TextAlignment::Left);
  let x = console.width() - SIDEBAR_WIDTH;
  let y = 0;
  let width = SIDEBAR_WIDTH;
  let height = console.height();
  let mut did_cursor_draw = false;
  vert_line(console, x, y, height, LINE_DBL_VERT);
  let text = format!(
    concat!(
      "   THIS IS SIDEBAR\n",
      "\n",
      "ATTR   \u{250c}POW SUB RES\u{2510}\n",
      "Body:{} |S:{} G:{} T:{}|\n",
      "Mind:{} |I:{} W:{} R:{}|\n",
      "Soul:{} |C:{} E:{} W:{}|\n",
      "       \u{2514}-----------\u{2518}\n",
      "\n"),
      pc.body(),
      pc.strength(),
      pc.grace(),
      pc.toughness(),
      pc.mind(),
      pc.intellect(),
      pc.wits(),
      pc.resolve(),
      pc.soul(),
      pc.charisma(),
      pc.empathy(),
      pc.will());

  console.print_rect(x + 2, y, width - 2, height, text);
  /*
  if player.cursor.active {
    if state.map.is_in_fov(player.cursor.pos.x, player.cursor.pos.y) {
      for entity in entities.iter() {
        if entity.pos() == player.cursor.pos {
          entity.draw_at(&mut console, x + 2, y + 13);
          console.print_rect(x + 4, y + 13, width - 5, 10, entity.desc());
          did_cursor_draw = true;
        }
      }
      if !did_cursor_draw {
        let cursor_tile = state.tiles.get(player.cursor.pos);
        match cursor_tile {
          Some(tile) => {
            console.put_char(x + 2, y + 13, tile.ch, BackgroundFlag::None);
            console.set_char_foreground(x + 2, y + 13, tile.fg);
            console.print_rect(x + 4, y + 13, width - 5, 10, tile.desc);
          },
          None => {}
        }
      }
    }
  }
  */
}
