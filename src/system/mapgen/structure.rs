use super::util::*;
use crate::component::{Color, Position, Region};
use crate::resource::{
    tile_types::*, AreaMap, Assets, GeographyTemplate, StructureTemplate, StructureTile, Tile,
    WorldState,
};
use crate::util::*;
use rand::prelude::*;
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
        // should never fail, but if it does the placeholder string is not going to cause a
        // problem
        let structure_name = choose(&structures, sample).unwrap_or("Ooops".to_string());
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
    let mut count = 0;
    let max_structures = 2;
    let mut tries = 0;
    let max_tries = 100;
    let horiz: Vec<i32> = (0..map.width).collect();
    let vert: Vec<i32> = (0..map.height).collect();
    let mut rng = world.region_rng(*region);

    while count < max_structures && tries < max_tries {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        while tries < max_tries {
            x = choose(&horiz, rng.gen_range(0.0, 1.0)).unwrap_or(0);
            y = choose(&vert, rng.gen_range(0.0, 1.0)).unwrap_or(0);
            if let Some(tile) = map.get(Position::new(x, y)) {
                if tile.type_id != TYPE_GRASS {
                    tries += 1;
                } else {
                    break;
                }
            }
        }
        if tries >= max_tries {
            println!(
                "failed to create a new structure (made {}/{}), skipping",
                count, max_structures
            );
            break;
        }

        let pos = [x, y];
        let sample = rand_up(fbm_offset(noise, pos, offset, 0.1, 1));

        if let Some(structure) = choose_structure(assets, noise, pos, offset, &map.geography) {
            let width_range: Vec<i32> = (structure.min_width..=structure.max_width).collect();

            let height_range: Vec<i32> = (structure.min_height..=structure.max_height).collect();
            let mut width: i32 = choose(&width_range, sample).unwrap_or(0);
            let mut height: i32 = choose(&height_range, sample).unwrap_or(0);
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
                use wfc::{retry::NumTimes, wrap::WrapNone, RunOwn, Size};

                println!("creating structure of size {}, {}", width, height);
                let perimeter = structure.perimeter;
                /*
                let wx = x as u32;
                let wy = y as u32;
                */
                let wp = perimeter as u32;
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
                    let icon = assets.get_icon(&tile.icon).base_ch();
                    map.set(
                        pos,
                        Tile::new(
                            icon,
                            tile.fg(),
                            tile.bg(),
                            tile.transparent,
                            tile.walkable,
                            TYPE_VEHICLE,
                        ),
                    );
                });
                build_rect(
                    map,
                    assets,
                    &structure.perimeter_tile,
                    Position::new(x, y),
                    width,
                    height,
                );
                count += 1;
            }
        }
    }
}

fn build_horizontal_line(
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

fn build_vertical_line(
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

fn build_rect(
    map: &mut AreaMap,
    assets: &Assets,
    tile: &StructureTile,
    pos: Position,
    width: i32,
    height: i32,
) {
    let wall = assets.get_icon(&tile.icon);
    let fg = Color::new(64, 64, 64);
    let bg = Color::new(0, 0, 0);
    let mut ch = wall.ch(false, false, true, true);
    let transparent = false;
    let walkable = true;
    build_horizontal_line(
        map,
        Position::new(pos.x + 1, pos.y),
        width - 4,
        ch,
        fg,
        bg,
        transparent,
        walkable,
    );
    build_horizontal_line(
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
    build_vertical_line(
        map,
        Position::new(pos.x, pos.y + 1),
        height - 3,
        ch,
        fg,
        bg,
        transparent,
        walkable,
    );
    build_vertical_line(
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
