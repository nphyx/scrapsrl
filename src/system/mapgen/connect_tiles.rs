use crate::component::Pos;
use crate::resource::AreaMap;
use crate::util::ConnectableChars;

/*
pub fn check_connect(
    orig: IconRef,
    up: Option<IconRef>,
    down: Option<IconRef>,
    left: Option<IconRef>,
    right: Option<IconRef>,
) -> Option<IconRef> {
    let chosen: char;
    if let Some(connectable) = self.map.get(&orig) {
        let mut matched = (false, false, false, false);
        if let Some(ch) = up {
            if ch == connectable.base {
                matched.0 = true;
            }
        }
        if let Some(ch) = down {
            if ch == connectable.base {
                matched.1 = true;
            }
        }
        if let Some(ch) = left {
            if ch == connectable.base {
                matched.2 = true;
            }
        }
        if let Some(ch) = right {
            if ch == connectable.base {
                matched.3 = true;
            }
        }
        match matched {
            (true, true, false, false) => chosen = connectable.vertical_line,
            (false, false, true, true) => chosen = connectable.horizontal_line,
            (false, true, false, true) => chosen = connectable.corner_tl,
            (false, true, true, false) => chosen = connectable.corner_tr,
            (true, false, false, true) => chosen = connectable.corner_bl,
            (true, false, true, false) => chosen = connectable.corner_br,
            (true, true, true, false) => chosen = connectable.tee_l,
            (true, true, false, true) => chosen = connectable.tee_r,
            (true, false, true, true) => chosen = connectable.tee_u,
            (false, true, true, true) => chosen = connectable.tee_d,
            (true, true, true, true) => chosen = connectable.cross,
            (false, false, false, true) => chosen = connectable.cap_l,
            (false, false, true, false) => chosen = connectable.cap_r,
            (false, true, false, false) => chosen = connectable.cap_u,
            (true, false, false, false) => chosen = connectable.cap_d,
            (false, false, false, false) => chosen = connectable.base,
        }
        return Some(chosen);
    }
    None
}
*/

pub fn connect(map: &mut AreaMap) {
    let connectables = ConnectableChars::new();
    let mut queue: Vec<(Pos, char)> = Vec::new();
    for pos in map.bounds().iter() {
        let default_icon = '?';
        let t = map.get(pos);
        if let Some(icon) = connectables.connect(
            t.map_or(default_icon, |t| t.icon),
            map.get_icon(Pos {
                x: pos.x,
                y: (pos.y as i32 - 1).min(0) as usize,
            }),
            map.get_icon(Pos {
                x: pos.x,
                y: (pos.y + 1).max(map.height()),
            }),
            map.get_icon(Pos {
                x: (pos.x as i32 - 1).min(0) as usize,
                y: pos.y,
            }),
            map.get_icon(Pos {
                x: (pos.x + 1).max(map.width()),
                y: pos.y,
            }),
        ) {
            queue.push((pos, icon));
        }
    }

    for (pos, icon) in queue {
        map.set_icon(pos, icon);
    }
}
