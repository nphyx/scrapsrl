/*
 * TODO FIXME this whole module is presently unused and needs reimplementation someday
use super::util::*;
use crate::component::{Color, Description, Pos};
use crate::resource::{RegionMap, Tile};
use crate::util::*;

fn place_tree(map: &mut RegionMap, cx: i32, cy: i32, size: i32) {
    let pos = Pos { x: cx, y: cy };
    let fg = Color { r: 14, g: 10, b: 3 };
    let bg: Color;
    if let Some(tile) = map.get(pos) {
        bg = tile.bg;
    } else {
        bg = Color {
            r: 28,
            g: 24,
            b: 12,
        };
    }
    fill_rect(
        map,
        cx - (size / 2),
        cy - (size / 2),
        size,
        size,
        Tile::new(
            'o',
            fg,
            bg,
            false,
            false,
            true,
            Description::new("a tree", "It's a tree."),
        ),
    );
}

fn check_tree_placement(tree_places: &[(i32, i32)], cx: i32, cy: i32, size: i32) -> bool {
    for x in cx - size - 1..=cx + size {
        for y in cy - size - 1..=cy + size {
            if tree_places.contains(&(x, y)) {
                return false;
            }
        }
    }
    true
}

/// place large trees on the map, density is 0-1 with 1 being very dense
pub fn place_trees(
    noise: &Noise,
    map: &mut RegionMap,
    width: i32,
    height: i32,
    offset: [i32; 2],
    scale: f32,
    density: f32,
) {
    let mut tree_places: Vec<(i32, i32)> = vec![];
    for x in 0..width {
        for y in 0..height {
            let size = ((rand_up(fbm_offset(noise, [x, y], offset, scale * 10.0, 32)) * 4.0)
                .floor()
                + 1.0) as i32;
            if check_tree_placement(&tree_places, x, y, size) {
                let i = rand_up(fbm_offset(noise, [x, y], offset, scale, 32));
                if i > 1.0 - density {
                    place_tree(map, x, y, size);
                    tree_places.push((x, y));
                }
            }
        }
    }
}
*/
