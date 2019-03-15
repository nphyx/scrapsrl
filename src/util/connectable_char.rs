use std::collections::HashMap;

struct ConnectableChar {
    pub base: char,
    pub horizontal_line: char,
    pub vertical_line: char,
    pub corner_tl: char,
    pub corner_tr: char,
    pub corner_bl: char,
    pub corner_br: char,
    pub tee_l: char,
    pub tee_r: char,
    pub tee_u: char,
    pub tee_d: char,
    pub cross: char,
    pub cap_l: char,
    pub cap_r: char,
    pub cap_u: char,
    pub cap_d: char,
}

type ConnectMap = HashMap<char, ConnectableChar>;

pub struct ConnectableChars {
    map: ConnectMap,
}

impl ConnectableChars {
    /// builds a map of base tiles and the tiles they can connect to
    /// TODO move the maps out to a config file and load them here
    pub fn new() -> ConnectableChars {
        let map = ConnectMap::new();
        /*
        map.insert(
            LINE,
            ConnectableChar {
                base: LINE,
                horiz: LINE_HORIZ,
                vert: LINE_VERT,
                corner_tl: LINE_TL,
                corner_tr: LINE_TR,
                corner_bl: LINE_BL,
                corner_br: LINE_BR,
                t_l: LINE_T_L,
                t_r: LINE_T_R,
                t_u: LINE_T_U,
                t_d: LINE_T_D,
                cross: LINE,
                cap_l: LINE_HORIZ,
                cap_r: LINE_HORIZ,
                cap_u: LINE_VERT,
                cap_d: LINE_VERT,
            },
        );
        map.insert(
            TREE_STUMP,
            ConnectableChar {
                base: TREE_STUMP,
                horiz: TREE_STUMP,
                vert: TREE_STUMP,
                corner_tl: TREE_TL,
                corner_tr: TREE_TR,
                corner_bl: TREE_BL,
                corner_br: TREE_BR,
                t_l: TREE_T_L,
                t_r: TREE_T_R,
                t_u: TREE_T_U,
                t_d: TREE_T_D,
                cross: TREE,
                cap_l: TREE_STUMP,
                cap_r: TREE_STUMP,
                cap_u: TREE_STUMP,
                cap_d: TREE_STUMP,
            },
        );
        map.insert(
            LINE_DBL,
            ConnectableChar {
                base: LINE_DBL,
                horiz: LINE_DBL_HORIZ,
                vert: LINE_DBL_VERT,
                corner_tl: LINE_DBL_TL,
                corner_tr: LINE_DBL_TR,
                corner_bl: LINE_DBL_BL,
                corner_br: LINE_DBL_BR,
                t_l: LINE_DBL_T_L,
                t_r: LINE_DBL_T_R,
                t_u: LINE_DBL_T_U,
                t_d: LINE_DBL_T_D,
                cross: LINE_DBL,
                cap_l: LINE_DBL_HORIZ,
                cap_r: LINE_DBL_HORIZ,
                cap_u: LINE_DBL_VERT,
                cap_d: LINE_DBL_VERT,
            },
        );
        */
        ConnectableChars { map }
    }

    /// checks whether a character can be connected
    /* FIXME UNUSED
    pub fn can_connect(&self, orig: char) -> bool {
      self.map.get(&orig).is_some()
    }
    */

    /// determines if the first char can be connected to adjacent chars and how to map it
    /// returns the appropriate char to use
    pub fn connect(
        &self,
        orig: char,
        up: char,
        down: char,
        left: char,
        right: char,
    ) -> Option<char> {
        let chosen: char;
        if let Some(connectable) = self.map.get(&orig) {
            let mut matched = (false, false, false, false);
            if up == connectable.base {
                matched.0 = true;
            }
            if down == connectable.base {
                matched.1 = true;
            }
            if left == connectable.base {
                matched.2 = true;
            }
            if right == connectable.base {
                matched.3 = true;
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
}
