use std::collections::{HashMap};
use tcod::map::Map;
use tcod::colors::Color;
use super::tiles::Tile;
use rand::prelude::*;
use crate::constants::{MAP_WIDTH, MAP_HEIGHT};
use crate::util::clamp;

const PLANT: Tile = Tile{color: Color{r:24, g:180, b:78},
    ch: '\u{e22f}', solid: false };
const WALL: Tile = Tile{color: Color{r:128, g:97, b:15}, 
        ch: '\u{2593}', solid: true };
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
            tiles.insert((x, y), WALL);
        }
    }

    let left = clamp(0, MAP_WIDTH, cx-2);
    let right = clamp(0, MAP_WIDTH, cx+2);
    map.set(left, cy,false, false);
    tiles.insert((left, cy), WALL);
    map.set(right, cy, false, false);
    tiles.insert((right, cy), WALL);

    map.set(cx, cy,false, false);
    tiles.insert((cx, cy), TREE_TRUNK);
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

    return (map, tiles);
}
