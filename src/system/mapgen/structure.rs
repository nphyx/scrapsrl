use super::util::*;
use crate::component::{Pos, Region};
use crate::resource::{AreaMap, Assets, GeographyTemplate, StructureTemplate, WorldState};
use crate::util::*;
use rand::prelude::*;
use tcod::noise::Noise;

fn choose_structure(
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
        let structure_name = choose(&structures, sample).unwrap_or_else(|| "Ooops".to_string());
        return Some(assets.get_structure(&structure_name));
    }
    None
}

fn choose_structure_dimensions(
    sample: f32,
    map: &AreaMap,
    t_l: Pos,
    structure: &StructureTemplate,
) -> Rect<usize> {
    let width_range: Vec<usize> = (structure.min_width..=structure.max_width).collect();

    let height_range: Vec<usize> = (structure.min_height..=structure.max_height).collect();
    // these are -2 to give space for the structure perimeter
    let b_r = Pos::new(
        (choose(&width_range, sample).unwrap_or(0) + t_l.x).min(map.width() - 2),
        (choose(&height_range, sample).unwrap_or(0) + t_l.y).min(map.height() - 2),
    );
    // first check we can fit the structure in here
    map.fit_rect(Rect::new(t_l, b_r))
}

pub fn build(
    assets: &Assets,
    noise: &Noise,
    map: &mut AreaMap,
    region: Region,
    world: &WorldState,
) -> Result<bool, &'static str> {
    let offset = region.to_offset();
    let mut count: u8 = 0;
    // roughly 1.5 slots per .1 pop
    let max_structures: u8 = (world.get_pop(region) * 15.0).floor() as u8;
    let mut tries = 0;
    let max_tries = 100;
    // these start at 1 to give room for a structure's perimeter tiles
    let horiz: Vec<usize> = (1..map.width()).collect();
    let vert: Vec<usize> = (1..map.height()).collect();
    let mut rng = world.region_rng(region);
    // map has no possible structures, let's bail
    if map.geography.structure_len() == 0 {
        return Err("no structures available for this geography");
    }

    while count < max_structures && tries < max_tries {
        let mut top_left = Pos::new(0, 0);
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
            let bounds = choose_structure_dimensions(sample, map, top_left, &structure);

            // now place a structure of the size we've found
            if structure.fits_in(bounds) {
                let mut subgrid: Grid<Tile> = Grid::with_bounds(bounds); //map.subgrid(available_area)?;
                let mut bounds = map.bounding_rect();
                count += structure.building_slots;
                // draw a wall (TODO connect the tiles, once tile connection is rebuilt)
                let wall = structure.perimeter_tile.to_tile(assets);
                for pos in bounds.iter_perimeter() {
                    subgrid.set(pos, wall.clone());
                }
                bounds.shrink_perimeter(1);
                populate_structure(assets, &mut subgrid, &bounds, &structure, &mut rng);
                map.paste_into(Default::default(), subgrid)?;
            }
        }
    }
    Ok(true)
}

use crate::resource::Tile;
use rand_pcg::*;
fn populate_structure(
    assets: &Assets,
    map_grid: &mut Grid<Tile>,
    room: &Rect<usize>,
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
        let tile = structure.get_tile(mapchar[&wc.chosen_pattern_id().expect("")]);
        let pos = Pos::from(coord) + room.t_l;
        map_grid.set(pos, tile.to_tile(assets))
    });
}
