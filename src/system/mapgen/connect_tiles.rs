use crate::component::Pos;
use crate::resource::{Assets, RegionMap};

pub fn connect(assets: &Assets, map: &mut RegionMap) {
    let mut queue: Vec<(Pos, char)> = Vec::new();
    let v = Pos::new(0, 1);
    let h = Pos::new(1, 0);
    for pos in map.grid.bounds {
        let center = map.grid.unchecked_get(pos).icon;
        let icon = assets.get_icon_by_ch(center);
        let up = if pos.y > 0 {
            map.grid.unchecked_get(pos - v).icon
        } else {
            ' '
        };
        let down = if pos.y < map.height() - 1 {
            map.grid.unchecked_get(pos + v).icon
        } else {
            ' '
        };
        let left = if pos.x > 0 {
            map.grid.unchecked_get(pos - h).icon
        } else {
            ' '
        };
        let right = if pos.x < map.width() - 1 {
            map.grid.unchecked_get(pos + h).icon
        } else {
            ' '
        };
        let found = icon
            .connected(
                center == up,
                center == down,
                center == left,
                center == right,
            )
            .ch();
        if found != center {
            queue.push((pos, found));
        }
    }

    for (pos, icon) in queue {
        map.set_icon(pos, icon);
    }
}
