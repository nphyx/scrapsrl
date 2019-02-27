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
            if sample < world.get_pop(*region) {
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
                        .collapse_retrying(NumTimes(1000), &mut rng)
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
                    draw_outer_wall(map, assets, Position::new(x, y), width, height);
                }
            }
        } // end x loop
    } // end y loop
}

fn draw_horizontal_line(
    map: &mut AreaMap,
    pos: Position,
    width: i32,
    ch: char,
    fg: Color,
    bg: Color,
    transparent: bool,
    walkable: bool,
) {
    for x in pos.x..=pos.x + width {
        let cpos = Position::new(x, pos.y);
        map.set(
            cpos,
            Tile::new(ch, fg, bg, transparent, walkable, TYPE_VEHICLE),
        );
    }
}

fn draw_vertical_line(
    map: &mut AreaMap,
    pos: Position,
    height: i32,
    ch: char,
    fg: Color,
    bg: Color,
    transparent: bool,
    walkable: bool,
) {
    for y in pos.y..pos.y + height {
        let cpos = Position::new(pos.x, y);
        map.set(
            cpos,
            Tile::new(ch, fg, bg, transparent, walkable, TYPE_VEHICLE),
        );
    }
}

fn draw_outer_wall(map: &mut AreaMap, assets: &Assets, pos: Position, width: i32, height: i32) {
    let wall = assets.get_icon("structure_wall_slat");
    let fg = Color::new(64, 64, 64);
    let bg = Color::new(0, 0, 0);
    let mut ch = wall.ch(false, false, true, true);
    let transparent = false;
    let walkable = true;
    draw_horizontal_line(
        map,
        Position::new(pos.x + 1, pos.y),
        width - 4,
        ch,
        fg,
        bg,
        transparent,
        walkable,
    );
    draw_horizontal_line(
        map,
        Position::new(pos.x + 1, pos.y + height - 2),
        width - 4,
        ch,
        fg,
        bg,
        transparent,
        walkable,
    );
    ch = wall.ch(true, true, false, false);
    draw_vertical_line(
        map,
        Position::new(pos.x, pos.y + 1),
        height - 3,
        ch,
        fg,
        bg,
        transparent,
        walkable,
    );
    draw_vertical_line(
        map,
        Position::new(pos.x + width - 2, pos.y + 1),
        height - 3,
        ch,
        fg,
        bg,
        transparent,
        walkable,
    );
    // top left
    ch = wall.ch(false, true, false, true);
    map.set(
        pos,
        Tile::new(ch, fg, bg, transparent, walkable, TYPE_VEHICLE),
    );
    // top right
    ch = wall.ch(false, true, true, false);
    map.set(
        Position::new(pos.x + width - 2, pos.y),
        Tile::new(ch, fg, bg, transparent, walkable, TYPE_VEHICLE),
    );
    // bottom left
    ch = wall.ch(true, false, false, true);
    map.set(
        Position::new(pos.x, pos.y + height - 2),
        Tile::new(ch, fg, bg, transparent, walkable, TYPE_VEHICLE),
    );
    // bottom right
    ch = wall.ch(true, false, true, false);
    map.set(
        Position::new(pos.x + width - 2, pos.y + height - 2),
        Tile::new(ch, fg, bg, transparent, walkable, TYPE_VEHICLE),
    );
}
