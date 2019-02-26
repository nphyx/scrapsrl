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

            if let Some(mut structure) =
                choose_structure(assets, noise, pos, offset, &map.geography)
            {
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
                    use rand::prelude::*;
                    use rand_pcg::*;
                    use wfc::{retry::NumTimes, wrap::WrapNone, RunOwn, Size};

                    println!("creating structure of size {}, {}", width, height);
                    let perimeter = structure.perimeter;
                    let wx = x as u32;
                    let wy = y as u32;
                    let wp = perimeter as u32;
                    let structure_seed: u64 =
                        // TODO is this too sloppy? probably works fine
                        (world.seed() as u64) * 1024 + (wx as u64) * 512 + wy as u64;
                    let mut rng = Pcg32::seed_from_u64(structure_seed);
                    let table = structure.get_pattern_table();
                    let stats = wfc::GlobalStats::new(table);
                    let wfc_runner = RunOwn::new_wrap(
                        Size::new(width as u32 - wp, height as u32 - wp),
                        &stats,
                        WrapNone,
                        &mut rng,
                    );
                    let wave = wfc_runner
                        .collapse_retrying(NumTimes(100), &mut rng)
                        .expect("failed to generate structure");
                    let grid = wave.grid();
                    let mapchar = structure.get_mapchar();
                    grid.enumerate().for_each(|(coord, wc)| {
                        let tile = structure
                            .get_tile(*mapchar.get(&wc.chosen_pattern_id().expect("")).unwrap());
                        let pos = Position::new(coord.x + x, coord.y + y);
                        let icon = assets.get_icon(&tile.icon.name).base_ch();
                        let colors = tile.colors;
                        map.set(
                            pos,
                            Tile::new(
                                icon,
                                colors.fg,
                                colors.bg,
                                tile.transparent,
                                tile.walkable,
                                TYPE_VEHICLE,
                            ),
                        );
                    });
                }
            }
        } // end x loop
    } // end y loop
}
