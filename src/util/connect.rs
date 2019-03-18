use crate::component::Pos;
use crate::resource::Assets;
use crate::util::Grid;

pub fn connect_chars<T>(
    assets: &Assets,
    grid: &mut Grid<T>,
    get: &Fn(&Grid<T>, Pos) -> char,
    set: &mut FnMut(&mut Grid<T>, Pos, char),
) {
    let mut queue: Vec<(Pos, char)> = Vec::new();
    let v = Pos::new(0, 1);
    let h = Pos::new(1, 0);
    let bounds = grid.bounds;
    for pos in bounds {
        let center = get(grid, pos);
        let icon = assets.get_icon_by_ch(center);
        let up = if pos.y > 0 { get(grid, pos - v) } else { ' ' };
        let down = if pos.y < bounds.height() - 1 {
            get(grid, pos + v)
        } else {
            ' '
        };
        let left = if pos.x > 0 { get(grid, pos - h) } else { ' ' };
        let right = if pos.x < bounds.width() - 1 {
            get(grid, pos + h)
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

    for (pos, ch) in queue {
        set(grid, pos, ch);
    }
}
