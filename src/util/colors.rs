use tcod::colors::Color;
use super::clamp;

fn to_fcel(a: u8) -> f32 { if a == 0 { 0.0 } else { a as f32/255.0 } }
fn to_ucel(a: f32) -> u8 { (a * 255.0).floor() as u8 }

pub fn screen_cel(a: u8, b: u8) -> u8 { to_ucel(1.0 - ( 1.0 - to_fcel(a) ) * ( 1.0 - to_fcel(b))) }
pub fn screen(a: &Color, b: &Color) -> Color {
  Color::new(
    screen_cel(a.r, b.r),
    screen_cel(a.g, b.g),
    screen_cel(a.b, b.b)
  )
}

fn multiply_cel(a: u8, b: u8) -> u8 { ((a as f32/255.0) * (b as f32/255.0) * 255.0) as u8 }
pub fn multiply(a: &Color, b: &Color) -> Color {
  Color::new(
    multiply_cel(a.r, b.r),
    multiply_cel(a.g, b.g),
    multiply_cel(a.b, b.b)
  )
}

pub fn overlay(a: &Color, b: &Color) -> Color {
  Color::new(
    if a.r > 127 { screen_cel(a.r, b.r) } else { multiply_cel(a.r, b.r) },
    if a.g > 127 { screen_cel(a.g, b.g) } else { multiply_cel(a.g, b.g) },
    if a.b > 127 { screen_cel(a.b, b.b) } else { multiply_cel(a.b, b.b) },
  )
}

fn soft_light_cel(a: u8, b: u8) -> u8 {
  let af = to_fcel(a);
  let bf = to_fcel(b);
  (255.0 * (((1.0 - (2.0 * bf)) * af * af) + (2.0 * bf * af))) as u8
}

pub fn soft_light(a: &Color, b:&Color) -> Color {
  Color::new(
    soft_light_cel(a.r, b.r),
    soft_light_cel(a.g, b.g),
    soft_light_cel(a.b, b.b)
  )
}

pub fn color_dodge_cel(a: u8, b: u8) -> u8 {
  let af = to_fcel(a);
  let bf = to_fcel(b);
  to_ucel(
    if af == 0.0 { 0.0 }
    else if bf == 1.0 { 1.0 }
    else { (af / (1.0 - bf)).min(1.0) }
  )
}
pub fn color_dodge(a: &Color, b:&Color) -> Color {
  Color::new(
    color_dodge_cel(a.r, b.r),
    color_dodge_cel(a.g, b.g),
    color_dodge_cel(a.b, b.b)
  )
}

pub fn desaturate(a: &Color) -> Color {
  let desat = to_ucel(
    (to_fcel(a.r) + to_fcel(a.g) + to_fcel(a.b)) / 3.0
  );
  Color::new(desat, desat, desat)
}
