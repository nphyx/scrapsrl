use crate::area_map::{AreaMap, Tile};
use tcod::noise::Noise;
use tcod::colors::{Color, lerp};
use crate::component::Position;
use super::util::*;
use super::tile_types::*;

pub fn lay_grass(noise: &Noise, map: &mut AreaMap, width: i32, height: i32, offset: [i32; 2], noise_scale: f32) {
  let color_sg_fg = Color{r:112, g:141, b:64};
  let color_sg_bg = Color{r:42, g:54, b:28};
  let color_tg_fg = Color{r:118, g:121, b:72};
  let color_tg_bg = Color{r:38, g:36, b:21};
  for x in 0..width {
    for y in 0..height {
      let i = rand_up(fbm_offset(noise, [x, y], offset, noise_scale, 32));
      let bg = lerp(color_sg_bg, color_tg_bg, i);
      let fg = lerp(color_sg_fg, color_tg_fg, i);
      if i < 0.5 {
        map.set(
          Position{x, y},
          Tile::new(',', fg, bg, true, true, TYPE_GRASS));
      } else {
        map.set(
          Position{x, y},
          Tile::new('"', fg, bg, true, true, TYPE_GRASS_LONG));
      }
    }
  }
}

