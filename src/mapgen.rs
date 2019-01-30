use std::collections::{HashMap};
use tcod::map::Map;
use tcod::colors::Color;
use super::tiles::Tile;
use rand::prelude::*;

const PLANT: Tile = Tile{color: Color{r:24, g:180, b:78},
    ch: '\u{e22f}', solid: false };
const WALL: Tile = Tile{color: Color{r:128, g:97, b:15}, 
        ch: '\u{2593}', solid: true };
const GRASS: Tile = Tile{color: Color{r:89, g:97, b:15},
        ch: ',', solid: false };
const TALL_GRASS: Tile = Tile{color: Color{r:89, g:97, b:15}, 
        ch: '"', solid: false };

pub fn generate(width: i32, height: i32) -> (Map, HashMap<(i32, i32), Tile>) {
    let mut map = Map::new(width, height);
    let mut tiles: HashMap<(i32, i32), Tile> = HashMap::new();

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
        let cx:i32 = rng.gen_range(1, width - 2);
        let cy:i32 = rng.gen_range(1, height - 2);
        for x in cx-1..cx+2 {
            for y in cy-1..cy+2 {
                map.set(x,y,false,false);
                tiles.insert((x, y), WALL);
            }
        }
        map.set(cx-2, cy,false, false);
        tiles.insert((cx-2, cy), WALL);
        map.set(cx+2, cy,false, false);
        tiles.insert((cx+2, cy), WALL);

    }

    return (map, tiles);
}
