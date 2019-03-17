use super::MapGenBundle;
use crate::component::Pos;

pub fn connect(bundle: &mut MapGenBundle) {
    let mut queue: Vec<(Pos, char)> = Vec::new();
    let v = Pos::new(0, 1);
    let h = Pos::new(1, 0);
    for pos in bundle.map.grid.bounds {
        let center = bundle.map.grid.unchecked_get(pos).icon;
        let icon = bundle.assets.get_icon_by_ch(center);
        let up = if pos.y > 0 {
            bundle.map.grid.unchecked_get(pos - v).icon
        } else {
            ' '
        };
        let down = if pos.y < bundle.map.height() - 1 {
            bundle.map.grid.unchecked_get(pos + v).icon
        } else {
            ' '
        };
        let left = if pos.x > 0 {
            bundle.map.grid.unchecked_get(pos - h).icon
        } else {
            ' '
        };
        let right = if pos.x < bundle.map.width() - 1 {
            bundle.map.grid.unchecked_get(pos + h).icon
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
        bundle.map.set_icon(pos, icon);
    }
}
