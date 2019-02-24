use super::util::*;
use crate::component::{Color, Position, Region};
use crate::resource::{
    tile_types::*, AreaMap, Assets, GeographyTemplate, StructureTemplate, Tile, WorldState,
};
use crate::util::*;
use tcod::noise::Noise;

fn choose_structure<'a>(
    assets: &Assets,
    noise: &Noise,
    pos: [i32; 2],
    offset: [i32; 2],
    geography: &GeographyTemplate,
) -> Option<StructureTemplate> {
    if let Some(ref structures) = geography.structures {
        let sample = rand_up(fbm_offset(noise, pos, offset, 1.0, 1));
        let structure_name = choose(structures.clone(), sample);
        return Some(assets.get_structure(&structure_name));
    }
    None
}

pub fn build(
    assets: &Assets,
    noise: &Noise,
    map: &mut AreaMap,
    region: &Region,
    world: &WorldState,
) {
    let offset = region.to_offset();
    for x in 0..map.width {
        for y in 0..map.height {
            let pos = [x, y];
            let sample = rand_up(fbm_offset(noise, pos, offset, 0.1, 1));

            // 10% chance of trying a structure, if the sample is lower than the
            // map's population
            if sample > (world.get_pop(*region)) {
                continue;
            }

            if let Some(structure) = choose_structure(assets, noise, pos, offset, &map.geography) {
                let mut width: i32 = structure.max_width;
                let mut height: i32 = structure.max_height;
                // first check we can fit the structure in here
                for sx in 0..width {
                    for sy in 0..height {
                        let maybe_tile = map.get(Position::new(x + sx, y + sy));
                        if maybe_tile.is_none() || maybe_tile.unwrap().type_id != TYPE_GRASS {
                            // if on the first row, shrink the width
                            if height == 0 {
                                width = sx + 1
                            }
                            // else shrink the height
                            else {
                                height = sy + 1
                            }
                            break;
                        }
                    }
                }

                // now place a structure of the size we've found
                if width >= structure.min_width && height >= structure.min_height {
                    println!("creating structure of size {}, {}", width, height);
                    let perimeter = structure.perimeter;
                    for sy in 0..=height {
                        for sx in 0..=width {
                            let pos = Position::new(sx + x, sy + y);
                            let fg = Color::new(255, 255, 255);
                            let bg = Color::new(32, 32, 32);
                            // FIXME update to use correct tile type params after
                            // redoing that system
                            if sx >= perimeter
                                && sx <= width - perimeter
                                && sy >= perimeter
                                && sy <= height - perimeter
                            {
                                map.set(pos, Tile::new(' ', fg, bg, false, false, TYPE_VEHICLE))
                            } else {
                                // other tiles should have been placed by now
                                if let Some(tile) = map.get_mut(pos) {
                                    tile.type_id = TYPE_VEHICLE;
                                }
                            }
                        }
                    }
                }
            }
        } // end x loop
    } // end y loop
}
