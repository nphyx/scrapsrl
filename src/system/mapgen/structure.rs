use super::util::*;
use crate::component::{Pos, Region};
use crate::resource::{Assets, RegionMap, StructureTemplate, WorldState};
use crate::util::*;
use rand::prelude::*;
use tcod::noise::Noise;

fn choose_structure<'a>(
    assets: &'a Assets,
    noise: &Noise,
    pos: Pos,
    region: Region,
    structures: &Vec<String>,
) -> Option<&'a StructureTemplate> {
    let sample = rand_up(fbm_offset(
        noise,
        pos.to_array(),
        region.to_offset(),
        1.0,
        1,
    ));
    let choice = &choose(structures, sample).unwrap();
    assets.get_structure(choice)
}

fn choose_structure_dimensions(
    sample: f32,
    grid: &Grid<Tile>,
    t_l: Pos,
    structure: &StructureTemplate,
) -> Rect<usize> {
    let width_range: Vec<usize> = (structure.min_width..=structure.max_width).collect();

    let height_range: Vec<usize> = (structure.min_height..=structure.max_height).collect();
    let b_r = Pos::new(
        (choose(&width_range, sample).unwrap_or(0) + t_l.x).min(t_l.x + grid.width() - 1),
        (choose(&height_range, sample).unwrap_or(0) + t_l.y).min(t_l.y + grid.height() - 1),
    );
    let bounds = Rect::new(t_l, b_r);
    if !grid.bounds.contains(bounds) {
        return Rect::new(Pos::new(0, 0), Pos::new(0, 0));
    }
    /*
    if grid.bounds.width() < MAP_WIDTH && grid.bounds.height() < MAP_HEIGHT {
        dbg!(map_constructed(&grid));
    }
    */
    grid.fit_rect(bounds, &|tile: &Tile| -> bool { tile.constructed })
}

use crate::resource::Tile;
use rand_pcg::*;
fn populate_structure(
    assets: &Assets,
    structure_grid: &mut Grid<Tile>,
    bounds: &Rect<usize>,
    structure: &StructureTemplate,
    rng: &mut Pcg32,
) {
    use wfc::{retry::NumTimes, wrap::WrapNone, RunOwn};
    let table = structure.get_pattern_table();
    let stats = wfc::GlobalStats::new(table);
    let wfc_runner = RunOwn::new_wrap(bounds.to_wave_size(), &stats, WrapNone, rng);
    let wave = wfc_runner
        .collapse_retrying(NumTimes(1000), rng)
        .expect("failed to generate structure");
    let grid = wave.grid();
    let mapchar = structure.get_mapchar();
    grid.enumerate().for_each(|(coord, wc)| {
        let tile = structure.get_tile(mapchar[&wc.chosen_pattern_id().expect("")]);
        let pos = Pos::from(coord) + bounds.t_l;
        structure_grid.unchecked_set(pos, tile.to_tile(assets))
    });
}

/// Builds structures recursively.
/// TODO revisit this, maybe find a way to have fewer parameters, it's kind of
/// junky passing all this stuff around.
fn build_structure(
    assets: &Assets,
    rng: &mut Pcg32,
    noise: &Noise,
    region: Region,
    structure: &StructureTemplate,
    mut bounds: Rect<usize>,
) -> Option<Grid<Tile>> {
    if structure.fits_in(bounds) {
        // now place a structure of the size we've found
        let mut grid: Grid<Tile> = Grid::with_bounds(bounds);
        bounds.shrink_perimeter(structure.perimeter);
        // fill in base tiles
        populate_structure(assets, &mut grid, &bounds, &structure, rng);
        // wipe out constructed status so rooms can be built on top
        for pos in grid.bounds.iter() {
            grid.unchecked_get_mut(pos).constructed = false;
        }
        let mut room_list = structure.interior_structures.clone();
        let mut remaining_grid = grid.bounds;
        let mut done = false;
        while !done {
            let mut built = 0;
            room_list.shuffle(rng);
            for room_name in &room_list {
                if let Some(room) = assets.get_structure(&room_name) {
                    let sample = rand_up(fbm_offset(
                        noise,
                        remaining_grid.t_l.to_array(),
                        region.to_offset(),
                        0.1,
                        1,
                    ));
                    let bounds =
                        choose_structure_dimensions(sample, &grid, remaining_grid.t_l, &room);
                    if let Some(room_grid) =
                        build_structure(assets, rng, noise, region, room, bounds)
                    {
                        built += 1;
                        grid.paste_into(Pos::new(0, 0), room_grid).ok()?;
                    }
                }
            }
            if built == 0 {
                done = true
            };
            remaining_grid =
                grid.fit_rect(grid.bounds, &|tile: &Tile| -> bool { tile.constructed });
        }
        if structure.perimeter > 0 {
            // draw a wall (TODO connect the tiles, once tile connection is rebuilt)
            bounds.expand_perimeter(structure.perimeter);
            if let Some(wall_template) = &structure.perimeter_tile {
                let mut wall = wall_template.to_tile(assets);
                wall.constructed = false;
                for pos in bounds.iter_perimeter() {
                    grid.unchecked_set(pos, wall.clone());
                }
            }
            bounds.shrink_perimeter(structure.perimeter);
        }
        // now mark as constructed, allowing wall overlap
        for pos in bounds.iter() {
            grid.unchecked_get_mut(pos).constructed = true;
        }
        return Some(grid);
    }
    None
}

// for debugging
#[allow(unused)]
pub fn map_constructed(grid: &Grid<Tile>) -> Grid<bool> {
    let bounds = grid.bounds;
    let mut c_grid: Grid<bool> = Grid::with_bounds(bounds);
    for pos in grid.bounds.iter() {
        c_grid.unchecked_set(pos, grid.unchecked_get(pos).constructed);
    }
    c_grid
}

pub fn build(
    assets: &Assets,
    noise: &Noise,
    map: &mut RegionMap,
    region: Region,
    world: &WorldState,
) -> Result<bool, &'static str> {
    let mut count: u8 = 0;
    // roughly 1.5 slots per .1 pop
    let max_structures: u8 = (world.get_pop(region) * 15.0).floor() as u8;
    let mut tries = 0;
    let max_tries = 100;
    // these start at 1 to give room for a structure's perimeter tiles
    let horiz: Vec<usize> = (1..map.width()).collect();
    let vert: Vec<usize> = (1..map.height()).collect();
    let rng = &mut world.region_rng(region);
    // map has no possible structures, let's bail
    if map.geography.structure_len() == 0 {
        return Err("no structures available for this geography");
    }

    while count < max_structures && tries < max_tries {
        let mut top_left = Pos::new(0, 0);
        while tries < max_tries {
            top_left.x = choose(&horiz, rng.gen_range(0.0, 1.0)).unwrap_or(0);
            top_left.y = choose(&vert, rng.gen_range(0.0, 1.0)).unwrap_or(0);
            if map.get(top_left).map_or(false, |t| t.constructed) {
                tries += 1;
            } else {
                break;
            }
        }
        if tries >= max_tries {
            break;
        }

        if let Some(structures) = map.geography.structures_ref() {
            if let Some(structure) = choose_structure(assets, noise, top_left, region, structures) {
                let sample = rand_up(fbm_offset(
                    noise,
                    top_left.to_array(),
                    region.to_offset(),
                    0.1,
                    1,
                ));
                let bounds = choose_structure_dimensions(sample, &map.grid, top_left, &structure);
                if let Some(structure_grid) =
                    build_structure(assets, rng, noise, region, structure, bounds)
                {
                    map.paste_into(Default::default(), structure_grid)?;
                    count += structure.building_slots;
                }
            }
        }
    }
    Ok(true)
}
