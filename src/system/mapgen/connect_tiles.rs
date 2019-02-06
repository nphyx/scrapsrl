use crate::area_map::{AreaMap, Tile};
use crate::util::ConnectableChars;
use crate::component::Position;

fn some_icon(tile: Option<Tile>) -> Option<char> {
  match tile {
    Some(t) => Some(t.icon),
    None => None
  }
}

pub fn connect(map: &mut AreaMap) {
  let connectables = ConnectableChars::new();
  let mut queue: Vec<(i32, i32, char)> = Vec::new();
  for x in 0..map.width {
    for y in 0..map.height {
      match map.get(Position{x, y}) {
        Some(t) => {
          match connectables.connect(
            &t.icon,
            map.get_icon(Position{x, y: y - 1}),
            map.get_icon(Position{x, y: y + 1}),
            map.get_icon(Position{x: x - 1, y}),
            map.get_icon(Position{x: x + 1, y})) {
            Some(icon) => {
              queue.push((x, y, icon));
            },
            None => {}
          }
        },
        None => {}
      }
    }
  }

  for (x, y, icon) in queue {
    map.set_icon(Position{x, y}, icon);
  }
}
