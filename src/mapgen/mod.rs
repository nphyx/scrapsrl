use std::collections::{HashMap};
use rand::prelude::*;
use tcod::map::Map;
use tcod::colors::Color;
use super::constants::{MAP_WIDTH, MAP_HEIGHT};
use super::util::clamp;
use super::tile::Tile;
mod connectable_tiles;

const PLANT: Tile = Tile{color: Color{r:24, g:180, b:78},
    ch: '\u{e22f}', solid: false };
const TREE_BARK: Tile = Tile{color: Color{r:128, g:97, b:15}, 
        ch: '\u{256c}', solid: true };
const GRASS: Tile = Tile{color: Color{r:89, g:97, b:15},
        ch: ',', solid: false };
const TALL_GRASS: Tile = Tile{color: Color{r:89, g:97, b:15}, 
        ch: '"', solid: false };
const TREE_TRUNK: Tile = Tile{color: Color{r: 128, g:97, b:15},
        ch: '0', solid: true };

type Tiles = HashMap<(i32, i32), Tile>;

pub fn put_tree(map: &mut Map, tiles: &mut Tiles, cx: i32, cy: i32) {

    let min_x = clamp(0, MAP_WIDTH, cx - 1);
    let min_y = clamp(0, MAP_HEIGHT, cy - 1);
    let max_x = clamp(0, MAP_WIDTH, cx + 2);
    let max_y = clamp(0, MAP_HEIGHT, cy + 2);

    for x in min_x..max_x {
        for y in min_y..max_y {
            map.set(x,y,false,false);
            tiles.insert((x, y), TREE_BARK);
        }
    }

    let left = clamp(0, MAP_WIDTH, cx-2);
    let right = clamp(0, MAP_WIDTH, cx+2);
    map.set(left, cy,false, false);
    tiles.insert((left, cy), TREE_BARK);
    map.set(right, cy, false, false);
    tiles.insert((right, cy), TREE_BARK);

    map.set(cx, cy,false, false);
    tiles.insert((cx, cy), TREE_TRUNK);
}

pub fn connect_tiles(map: &mut Map, tiles: &mut Tiles) {
    let connectables = connectable_tiles::build_connectables();

    let mut queue: Vec<(i32, i32, Tile)> = Vec::new();

    for ((x, y), tile) in tiles.iter() {
        let ch = tile.ch;
        // TODO maybe put this shiz at a higher level where it draws the map
        match connectables.get(&ch) {
            Some(connectable) => {
                let mut up = false;
                let mut down = false;
                let mut left = false;
                let mut right = false;
                let mut ch = connectable.base;
                match tiles.get(&(x - 1, *y)) {
                    Some(tile) => if tile.ch == ch { left = true; },
                    _ => {}
                }
                match tiles.get(&(x + 1, *y)) {
                    Some(tile) => if tile.ch == ch { right = true; },
                    _ => {}
                }
                match tiles.get(&(*x, y - 1)) {
                    Some(tile) => if tile.ch == ch { up = true; },
                    _ => {}
                }
                match tiles.get(&(*x, y + 1)) {
                    Some(tile) => if tile.ch == ch { down = true; },
                    _ => {}
                }
                match (up, down, left, right) {
                    (true, true, false, false) => ch = connectable.vert,
                    (false, false, true, true) => ch = connectable.horiz,
                    (false, true, false, true) => ch = connectable.corner_tl,
                    (false, true, true, false) => ch = connectable.corner_tr,
                    (true, false, false, true) => ch = connectable.corner_bl,
                    (true, false, true, false) => ch = connectable.corner_br,
                    (true, true, true, false) => ch = connectable.t_l,
                    (true, true, false, true) => ch = connectable.t_r,
                    (true, false, true, true) => ch = connectable.t_u,
                    (false, true, true, true) => ch = connectable.t_d,
                    (true, true, true, true) => ch = connectable.cross,
                    (false, false, false, true) => ch = connectable.cap_l,
                    (false, false, true, false) => ch = connectable.cap_r,
                    (false, true, false, false) => ch = connectable.cap_u,
                    (true, false, false, false) => ch = connectable.cap_d,
                    (false, false, false, false) => ch = connectable.base
                }
                queue.push((*x, *y, Tile{
                    color: tile.color,
                    ch: ch,
                    solid: tile.solid
                }));
            },
            None => {}
        }
    }

    while  queue.len() > 0  {
        let item = queue.pop();
        match item {
            Some(item) => {
                let (x, y, tile) = item;
                tiles.insert((x, y), tile);
            },
            None => break
        }
    }
}

pub fn generate(width: i32, height: i32) -> (Map, HashMap<(i32, i32), Tile>) {
    let mut map = Map::new(width, height);
    let mut tiles: Tiles = HashMap::new();
    let mut rng = rand::thread_rng();

    // Set the map.

    // lay grass
    for x in 0..width {
        for y in 0..height {
            let r:f32 = rng.gen();
            // Place some walls randomly.
            if r < 0.3 {
                tiles.insert((x, y), PLANT);
                map.set(x,y,true,true);
            } else if r < 0.7 {
                tiles.insert((x, y), GRASS);
                map.set(x,y,true,true);
            } else {
                tiles.insert((x, y), TALL_GRASS);
                map.set(x,y,true,true);
            }
        }
    }
    // place trees
    let count:i32 = rng.gen_range(5, 40);
    for _ in 0..count {
        let cx:i32 = rng.gen_range(2, width - 2);
        let cy:i32 = rng.gen_range(2, height - 2);
        put_tree(&mut map, &mut tiles, cx, cy);
    }

    connect_tiles(&mut map, &mut tiles);

    return (map, tiles);
}
