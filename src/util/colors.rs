use super::clamp;
use crate::component::Color;

fn to_fcel(a: u8) -> f32 {
    if a == 0 {
        0.0
    } else {
        f32::from(a) / 255.0
    }
}
fn to_ucel(a: f32) -> u8 {
    (clamp(0.0, 1.0, a) * 255.0).floor() as u8
}

#[allow(unused)]
pub fn screen_cel(a: u8, b: u8) -> u8 {
    to_ucel(1.0 - (1.0 - to_fcel(a)) * (1.0 - to_fcel(b)))
}
#[allow(unused)]
pub fn screen(a: Color, b: Color) -> Color {
    Color::new(
        screen_cel(a.r, b.r),
        screen_cel(a.g, b.g),
        screen_cel(a.b, b.b),
    )
}

fn multiply_cel(a: u8, b: u8) -> u8 {
    ((f32::from(a) / 255.0) * (f32::from(b) / 255.0) * 255.0) as u8
}
pub fn multiply(a: Color, b: Color) -> Color {
    Color::new(
        multiply_cel(a.r, b.r),
        multiply_cel(a.g, b.g),
        multiply_cel(a.b, b.b),
    )
}

#[allow(unused)]
pub fn overlay(a: Color, b: Color) -> Color {
    Color::new(
        if a.r > 127 {
            screen_cel(a.r, b.r)
        } else {
            multiply_cel(a.r, b.r)
        },
        if a.g > 127 {
            screen_cel(a.g, b.g)
        } else {
            multiply_cel(a.g, b.g)
        },
        if a.b > 127 {
            screen_cel(a.b, b.b)
        } else {
            multiply_cel(a.b, b.b)
        },
    )
}

fn soft_light_cel(a: u8, b: u8, i: f32) -> u8 {
    let af = to_fcel(a);
    let bf = to_fcel(b);
    lerp_cel(
        a,
        to_ucel(((1.0 - (2.0 * bf)) * af * af) + (2.0 * bf * af)),
        i,
    )
}

pub fn soft_light(a: Color, b: Color, i: f32) -> Color {
    Color::new(
        soft_light_cel(a.r, b.r, i),
        soft_light_cel(a.g, b.g, i),
        soft_light_cel(a.b, b.b, i),
    )
}

pub fn color_dodge_cel(a: u8, b: u8) -> u8 {
    let af = to_fcel(a);
    let bf = to_fcel(b);
    to_ucel(if af == 0.0 {
        0.0
    } else if (bf - 1.0).abs() < std::f32::EPSILON {
        1.0
    } else {
        (af / (1.0 - bf)).min(1.0)
    })
}
pub fn color_dodge(a: Color, b: Color) -> Color {
    Color::new(
        color_dodge_cel(a.r, b.r),
        color_dodge_cel(a.g, b.g),
        color_dodge_cel(a.b, b.b),
    )
}

pub fn desaturate(a: Color, i: f32) -> Color {
    let desat = to_ucel((to_fcel(a.r) + to_fcel(a.g) + to_fcel(a.b)) / 3.0);
    lerp(a, Color::new(desat, desat, desat), i)
}

fn lerp_cel(a: u8, b: u8, i: f32) -> u8 {
    let af = to_fcel(a);
    let bf = to_fcel(b);
    to_ucel(af + i * (bf - af))
}

pub fn lerp(a: Color, b: Color, i: f32) -> Color {
    Color::new(
        lerp_cel(a.r, b.r, i),
        lerp_cel(a.g, b.g, i),
        lerp_cel(a.b, b.b, i),
    )
}

#[allow(unused)]
pub fn add_cel(a: u8, b: u8, i: f32) -> u8 {
    let af = to_fcel(a);
    let bf = to_fcel(b);
    to_ucel(af + i * bf)
}

#[allow(unused)]
pub fn add(a: Color, b: Color, i: f32) -> Color {
    Color::new(
        add_cel(a.r, b.r, i),
        add_cel(a.g, b.g, i),
        add_cel(a.b, b.b, i),
    )
}

#[allow(unused)]
pub fn color_clamp(a: Color, b: Color, c: Color) -> Color {
    Color::new(
        clamp(a.r, b.r, c.r),
        clamp(a.g, b.g, c.g),
        clamp(a.g, b.g, c.g),
    )
}

#[allow(unused)]
pub fn min_cel(a: u8, b: u8) -> u8 {
    let af = to_fcel(a);
    let bf = to_fcel(b);
    to_ucel(af.min(bf))
}
#[allow(unused)]
pub fn color_min(a: Color, b: Color) -> Color {
    Color::new(min_cel(a.r, b.r), min_cel(a.g, b.g), min_cel(a.b, b.b))
}

#[allow(unused)]
pub fn max_cel(a: u8, b: u8) -> u8 {
    let af = to_fcel(a);
    let bf = to_fcel(b);
    to_ucel(af.max(bf))
}
#[allow(unused)]
pub fn color_max(a: Color, b: Color) -> Color {
    Color::new(max_cel(a.r, b.r), max_cel(a.g, b.g), max_cel(a.b, b.b))
}
