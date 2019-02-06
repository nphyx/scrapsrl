use specs::{System, Write};
use crate::area_map::AreaMap;
// use crate::util::ConnectableChars;

// FIXME
pub struct ConnectTiles;

impl<'a> System<'a> for ConnectTiles {
  type SystemData = Write<'a, AreaMap<'static>>;

  fn run(&mut self, mut map: Self::SystemData) {
    /*
    let mut queue: Vec<(Position, Tile)> = Vec::new();

    for (position, tile, icon, connected, entity) in (&positions, &tiles, !&connected_tiles, &entities).join() {
      let connectables = ConnectableChars::new();
        let orig = tile.ch;
        match connectables.connect(
          &orig,
          self.get_ch(Position{x:coord.x, y:coord.y - 1}),
          self.get_ch(Position{x:coord.x, y:coord.y + 1}),
          self.get_ch(Position{x:coord.x - 1, y:coord.y}),
          self.get_ch(Position{x:coord.x + 1, y:coord.y})) {
          Some(chosen) => {
            queue.push((*coord, Tile{
              ch: chosen,
              ..*tile
            }));
          },
          None => {}
        }
      }
    }

    while  queue.len() > 0  {
      let item = queue.pop();
      match item {
        Some(item) => {
          let (coord, tile) = item;
          self.map.insert(coord, tile);
        },
        None => break
      }
    }
  */
  }
}
