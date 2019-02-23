use super::util::*;
use crate::component::{Color, Position};
use crate::resource::{tile_types::*, AreaMap, Assets, GeographyTemplate, Tile};
use crate::util::colors::lerp;
use tcod::noise::Noise;

/* we'll want to get grass color in other functions maybe
pub fn grass_bg_color(
    noise: &Noise,
    pos: [i32; 2],
    offset: [i32; 2],
    noise_scale: f32,
    octaves: u32,
) -> Color {
    let color_sg_bg = Color {
        r: 42,
        g: 54,
        b: 28,
    };
    let color_tg_bg = Color {
        r: 38,
        g: 36,
        b: 21,
    };
    let i = rand_up(fbm_offset(noise, pos, offset, noise_scale, octaves));
    lerp(color_sg_bg, color_tg_bg, i)
}
*/

/// Selects a background color for the cover tile, blending based on noise_sample
fn select_bg(geography: &GeographyTemplate, noise_sample: f32) -> Color {
    let mut color: Color = Color::new(0, 0, 0);
    let mut last_freq = 0.0;
    let mut last_color = Color::new(0, 0, 0);
    if let Some(ref cover_set) = geography.ground_cover {
        let mut cover_list = cover_set.iter();
        if let Some(selected) = cover_list.nth(0) {
            last_freq = selected.frequency;
            color = selected.colors.bg;
            last_color = selected.colors.bg;
        }
        for cover in cover_list {
            if noise_sample < last_freq {
                return last_color;
            }
            let i = (noise_sample - last_freq) / (cover.frequency - last_freq);
            color = lerp(last_color, cover.colors.bg, i);
            last_freq = cover.frequency;
            last_color = cover.colors.bg;
            if last_freq > noise_sample {
                break;
            }
        }
    }
    if noise_sample > last_freq {
        return last_color;
    }
    return color;
}

/// Selects a foreground color for the cover tile (does not blend, unlike fg)
fn select_fg(geography: &GeographyTemplate, noise_sample: f32) -> Color {
    let mut color: Color = Color::new(0, 0, 0);
    let mut sum_freq = 0.0;
    if let Some(ref cover_set) = geography.ground_cover {
        let mut cover_list = cover_set.iter();
        if let Some(cover) = cover_list.nth(0) {
            color = cover.colors.fg;
        }
        for cover in cover_list {
            sum_freq += cover.frequency;
            if sum_freq < noise_sample {
                color = cover.colors.fg;
            } else {
                return color;
            }
        }
    }
    return color;
}

/// Selects the icon to display for the ground cover
fn select_icon(geography: &GeographyTemplate, noise_sample: f32) -> String {
    let mut icon_name = "?".to_string();
    let mut last_freq = 0.0;
    if let Some(ref cover_set) = geography.ground_cover {
        let mut cover_list = cover_set.iter();
        if let Some(cover) = cover_list.nth(0) {
            icon_name = cover.icon.clone().name;
            last_freq = cover.frequency;
        }
        for cover in cover_list {
            if noise_sample < last_freq {
                return icon_name;
            }
            let i = (noise_sample - last_freq) / (cover.frequency - last_freq);
            if i > 0.5 {
                icon_name = cover.icon.clone().name;
                last_freq = cover.frequency;
            }
            if last_freq > noise_sample {
                break;
            }
        }
    }
    return icon_name;
}

pub fn base(
    noise: &Noise,
    map: &mut AreaMap,
    offset: [i32; 2],
    noise_scale: f32,
    geography: &GeographyTemplate,
    templates: &Assets,
) {
    for x in 0..map.width {
        for y in 0..map.height {
            let i = rand_up(fbm_offset(noise, [x, y], offset, noise_scale, 32));
            let bg = select_bg(&geography, i);
            let fg = select_fg(&geography, i);
            let icon = templates.get_icon(&select_icon(&geography, i)).base_ch();
            map.set(
                Position { x, y },
                Tile::new(icon, fg, bg, true, true, TYPE_GRASS),
            );
        }
    }
}

/// places scatter objects based on geography template
pub fn scatter(
    noise: &Noise,
    map: &mut AreaMap,
    offset: [i32; 2],
    noise_scale: f32,
    geography: &GeographyTemplate,
    templates: &Assets,
) {
    if let Some(ref scatter_list) = geography.scatter {
        for x in 0..map.width {
            for y in 0..map.height {
                let mut scale = 0.0; // this gets twiddled every pass
                for ref scatter_obj in scatter_list.iter() {
                    scale = scale + (scatter_obj.frequency * noise_scale);
                    let i = rand_up(fbm_offset(noise, [x, y], offset, scale + noise_scale, 8));
                    if i < scatter_obj.frequency {
                        let icon = templates.get_icon(&scatter_obj.icon.name).base_ch();
                        let pos = Position { x, y };
                        let mut bg = Color::default();
                        if let Some(tile) = map.get(pos) {
                            bg = tile.bg;
                        }
                        map.set(
                            Position { x, y },
                            Tile::new(icon, scatter_obj.colors.fg, bg, true, true, TYPE_GRASS),
                        );
                    }
                }
            }
        }
    }
}
