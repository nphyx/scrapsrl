use super::MapGenBundle;
use crate::resource::Tile;
use crate::util::{connect_chars, Grid};

pub fn connect(bundle: &mut MapGenBundle) {
    /*
     */
    let assets = &*bundle.assets;
    let grid = &mut (bundle.map).grid;

    let get = &|grid: &Grid<Tile>, pos| -> char { grid.unchecked_get(pos).icon };
    let set = &mut |grid: &mut Grid<Tile>, pos, ch| {
        grid.unchecked_get_mut(pos).icon = ch;
    };

    connect_chars(assets, grid, get, set);
    /*
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
    }
    */
}
