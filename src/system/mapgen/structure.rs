use super::util::*;
use crate::component::{Description, Position, Region};
use crate::resource::{AreaMap, Assets, GeographyTemplate, StructureTemplate, Tile, WorldState};
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
    let mut count: u8 = 0;
    // roughly 1.5 slots per .1 pop
    let max_structures: u8 = (world.get_pop(*region) * 15.0).floor() as u8;
    let mut tries = 0;
    let max_tries = 100;
    // these start at 1 to give room for a structure's perimeter tiles
    let horiz: Vec<i32> = (1..map.width).collect();
    let vert: Vec<i32> = (1..map.height).collect();
    let mut rng = world.region_rng(*region);
    // map has no possible structures, let's bail
    if map.geography.structure_len() == 0 {
        return;
    }

    while count < max_structures && tries < max_tries {
        let mut top_left = Position::new(0, 0);
        while tries < max_tries {
            top_left.x = choose(&horiz, rng.gen_range(0.0, 1.0)).unwrap_or(0);
            top_left.y = choose(&vert, rng.gen_range(0.0, 1.0)).unwrap_or(0);
            if let Some(tile) = map.get(top_left) {
                if tile.constructed {
                    tries += 1;
                } else {
                    break;
                }
            }
        }
        if tries >= max_tries {
            break;
        }

        let sample = rand_up(fbm_offset(noise, top_left.to_array(), offset, 0.1, 1));

        if let Some(structure) =
            choose_structure(assets, noise, top_left.to_array(), offset, &map.geography)
        {
            let width_range: Vec<i32> = (structure.min_width..=structure.max_width).collect();

            let height_range: Vec<i32> = (structure.min_height..=structure.max_height).collect();
            // these are -2 to give space for the structure perimeter
            let bottom_right = Position::new(
                (choose(&width_range, sample).unwrap_or(0) + top_left.x).min(map.width - 2),
                (choose(&height_range, sample).unwrap_or(0) + top_left.y).min(map.height - 2),
            );
            let rect = Rect::new(top_left.clone(), bottom_right.clone());
            // first check we can fit the structure in here
            let mut room = map.fit_rect(rect);

            // now place a structure of the size we've found
            if room.width() >= structure.min_width && room.height() >= structure.min_height {
                count += structure.building_slots;
                // draw a wall (TODO connect the tiles, once tile connection is rebuilt)
                for pos in room.iter_perimeter() {
                    map.set(pos, structure.perimeter_tile.to_tile(assets));
                }
                room.shrink_perimeter(1);
                populate_room(assets, map, &room, &structure, &mut rng);
            }
        }
    }
}

use rand_pcg::*;
fn populate_room(
    assets: &Assets,
    map: &mut AreaMap,
    room: &Rect,
    structure: &StructureTemplate,
    rng: &mut Pcg32,
) {
    use wfc::{retry::NumTimes, wrap::WrapNone, RunOwn};
    let table = structure.get_pattern_table();
    let stats = wfc::GlobalStats::new(table);
    let wfc_runner = RunOwn::new_wrap(room.to_wave_size(), &stats, WrapNone, rng);
    let wave = wfc_runner
        .collapse_retrying(NumTimes(1000), rng)
        .expect("failed to generate structure");
    let grid = wave.grid();
    let mapchar = structure.get_mapchar();
    grid.enumerate().for_each(|(coord, wc)| {
        let tile = structure.get_tile(*mapchar.get(&wc.chosen_pattern_id().expect("")).unwrap());
        let pos = Position::from(coord) + room.t_l;
        map.set(pos, tile.to_tile(assets));
    });
}
