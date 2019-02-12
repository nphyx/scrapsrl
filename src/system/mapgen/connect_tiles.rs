use crate::component::Position;
use crate::resource::AreaMap;
use crate::util::ConnectableChars;

pub fn connect(map: &mut AreaMap) {
    let connectables = ConnectableChars::new();
    let mut queue: Vec<(i32, i32, char)> = Vec::new();
    for x in 0..map.width {
        for y in 0..map.height {
            if let Some(t) = map.get(Position { x, y }) {
                if let Some(icon) = connectables.connect(
                    t.icon,
                    map.get_icon(Position { x, y: y - 1 }),
                    map.get_icon(Position { x, y: y + 1 }),
                    map.get_icon(Position { x: x - 1, y }),
                    map.get_icon(Position { x: x + 1, y }),
                ) {
                    queue.push((x, y, icon));
                }
            }
        }
    }

    for (x, y, icon) in queue {
        map.set_icon(Position { x, y }, icon);
    }
}
