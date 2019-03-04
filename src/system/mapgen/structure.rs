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
    let max_structures = 2;
    let mut tries = 0;
    let max_tries = 100;
    let horiz: Vec<i32> = (0..map.width).collect();
    let vert: Vec<i32> = (0..map.height).collect();
    let mut rng = world.region_rng(*region);

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
            println!(
                "failed to create a new structure (made {}/{}), skipping",
                count, max_structures
            );
            break;
        }

        let sample = rand_up(fbm_offset(noise, top_left.to_array(), offset, 0.1, 1));

        if let Some(structure) =
            choose_structure(assets, noise, top_left.to_array(), offset, &map.geography)
        {
            let width_range: Vec<i32> = (structure.min_width..=structure.max_width).collect();

            let height_range: Vec<i32> = (structure.min_height..=structure.max_height).collect();
            let mut bottom_right = Position::new(
                choose(&width_range, sample).unwrap_or(0),
                choose(&height_range, sample).unwrap_or(0),
            );
            // first check we can fit the structure in here
            let room = Rect::new(top_left.clone(), bottom_right.clone());

            // now place a structure of the size we've found
            if room.width() >= structure.min_width && room.height() >= structure.min_height {
                populate_room(assets, map, &room, &structure, &mut rng);
                count += structure.building_slots;
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

    println!(
        "creating structure of size {}, {}",
        room.width(),
        room.height()
    );
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
        let icon = assets.get_icon(&tile.icon).base_ch();
        map.set(
            pos,
            Tile::new(
                icon,
                tile.fg(),
                tile.bg(),
                tile.transparent,
                tile.walkable,
                true,
                Description::default(),
            ),
        );
    });
    /*
    build_rect(
        map,
        assets,
        &structure.perimeter_tile,
        Position::new(x, y),
        width,
        height,
    );
    */
}
