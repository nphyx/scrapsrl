use specs::{WriteStorage, Entities};
use crate::mapgen::{Tile, ConnectedTile};

pub struct ConnectTiles;

impl<'a> System<'a> for ConnectTiles {
  type SystemData = (
    WriteStorage<'a, Tile>,
    WriteStorage<'a, ConnectedTile>,
    Entities<'a>
  );

  fn run(&mut self, (tiles, connected_tiles, entities): Self::SystemData) {
    let mut queue: Vec<(Coord, Tile)> = Vec::new();

    for (tile, connected, entity) in (&tiles, !&connected_tiles, &entities).join() {
      let connectables = ConnectableChars::new();


      for (coord, tile) in self.map.iter() {
        let orig = tile.ch;
        match connectables.connect(
          &orig,
          self.get_ch(Coord{x:coord.x, y:coord.y - 1}),
          self.get_ch(Coord{x:coord.x, y:coord.y + 1}),
          self.get_ch(Coord{x:coord.x - 1, y:coord.y}),
          self.get_ch(Coord{x:coord.x + 1, y:coord.y})) {
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
  }
}
