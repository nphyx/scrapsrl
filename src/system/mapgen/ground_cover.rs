use crate::area_map::{AreaMap};
use tcod::noise::Noise;
use tcod::colors::{Color, lerp};
use crate::component::Position;
use super::util::*;

pub fn lay_grass(noise: &Noise, map: &mut AreaMap, width: i32, height: i32, offset: [i32; 2], noise_scale: f32) {
  let desc_sg_short = "grass";
  let desc_sg_long = "Just some ordinary grass.";
  let desc_tg_short = "tall grass";
  let desc_tg_long = "Some tall grass.";
  let opaque = false;
  let solid = false;
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
          prep_tile(',', fg, bg, opaque, solid, desc_sg_short, desc_sg_long));
      } else {
        map.set(
          Position{x, y},
          prep_tile('"', fg, bg, opaque, solid, desc_tg_short, desc_tg_long));
      }
    }
  }
}

