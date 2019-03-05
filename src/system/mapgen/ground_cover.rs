use super::util::*;
use crate::component::{Color, Description, Position};
use crate::resource::{AreaMap, Assets, GeographyTemplate, GroundCover, Tile};
use crate::util::colors::lerp;
use crate::util::*;
use tcod::noise::Noise;

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
fn select_ground_cover(geography: &GeographyTemplate, noise_sample: f32) -> GroundCover {
    let mut selected_cover = GroundCover::default();
    let mut last_freq = 0.0;
    if let Some(ref cover_set) = geography.ground_cover {
        let mut cover_list = cover_set.iter();
        if let Some(cover) = cover_list.nth(0) {
            selected_cover = cover.clone();
            last_freq = cover.frequency;
        }
        for cover in cover_list {
            if noise_sample < last_freq {
                return selected_cover.clone();
            }
            let i = (noise_sample - last_freq) / (cover.frequency - last_freq);
            if i > 0.5 {
                selected_cover = cover.clone();
                last_freq = cover.frequency;
            }
            if last_freq > noise_sample {
                break;
            }
        }
    }
    selected_cover
}

pub fn base(
    noise: &Noise,
    map: &mut AreaMap,
    offset: [i32; 2],
    noise_scale: f32,
    templates: &Assets,
) {
    for x in 0..map.width {
        for y in 0..map.height {
            let i = rand_up(fbm_offset(noise, [x, y], offset, noise_scale, 32));
            let bg = select_bg(&map.geography, i);
            let fg = select_fg(&map.geography, i);
            let selected_cover = select_ground_cover(&map.geography, i);
            let icon = templates.get_icon(&selected_cover.icon.name).base_ch();
            map.set(
                Position { x, y },
                Tile::new(
                    icon,
                    fg,
                    bg,
                    true,
                    true,
                    false,
                    Description::new(&selected_cover.short, &selected_cover.long),
                ),
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
    templates: &Assets,
) {
    if let Some(ref scatter_list) = map.geography.scatter {
        use std::collections::HashMap;
        let mut queue: HashMap<Position, Tile> = HashMap::new();
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
                        queue.insert(
                            Position { x, y },
                            Tile::new(
                                icon,
                                scatter_obj.colors.fg,
                                bg,
                                true,
                                true,
                                false,
                                Description::new(&scatter_obj.short, &scatter_obj.long),
                            ),
                        );
                    }
                }
            }
        }
        for (pos, tile) in queue.drain() {
            map.set(pos, tile);
        }
    }
}
