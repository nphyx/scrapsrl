use serde::{Deserialize, Serialize};

#[allow(unused)]
pub enum DoorState {
    Closed,
    Open,
    Frame,
}

#[allow(unused)]
pub enum DoorOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Icon {
    /// base tile (no connections in any direction)
    base: char,
    #[serde(default)]
    /// top left corner, connecting down and right
    corner_tl: Option<char>,
    #[serde(default)]
    /// top right corner, connecting down and left
    corner_tr: Option<char>,
    #[serde(default)]
    /// bottom left corner, connecting up and right
    corner_bl: Option<char>,
    #[serde(default)]
    /// bottom right corner, connecting up and left
    corner_br: Option<char>,
    #[serde(default)]
    /// connected on top and bottom sides
    vertical: Option<char>,
    #[serde(default)]
    /// connected on left and right sides
    horizontal: Option<char>,
    #[serde(default)]
    /// tee shape with leg pointing left
    tee_l: Option<char>,
    #[serde(default)]
    /// tee shape with leg pointing right
    tee_r: Option<char>,
    #[serde(default)]
    /// tee shape with leg pointing up
    tee_u: Option<char>,
    #[serde(default)]
    /// tee shape with leg pointing down
    tee_d: Option<char>,
    #[serde(default)]
    /// center tile, connected on all four sides
    center: Option<char>,
    #[serde(default)]
    /// left endcap, connected only to right
    cap_l: Option<char>,
    #[serde(default)]
    /// right endcap, connected only to left
    cap_r: Option<char>,
    #[serde(default)]
    /// up endcap, connected only below
    cap_u: Option<char>,
    #[serde(default)]
    /// down endcap, connected only above
    cap_d: Option<char>,
    #[serde(default)]
    /// for wall tiles, a door (for a horizontal wall)
    door_closed: Option<char>,
    #[serde(default)]
    /// for wall tiles, an open door (horizontal walls)
    door_open: Option<char>,
    #[serde(default)]
    /// for wall tiles, a door frame with no door
    door_frame: Option<char>,
    #[serde(default)]
    /// for wall tiles, a door (on vertical walls)
    door_vertical: Option<char>,
    #[serde(default)]
    /// for wall tiles, an open door (on vertical walls)
    door_vertical_open: Option<char>,
    #[serde(default)]
    /// for wall tiles, a list of window variants
    windows: Vec<char>,
    #[serde(default)]
    /// variants on the base character which should be chosen at random and have no specific
    /// connections
    variants: Vec<char>,
}

/// An icon spec, loaded from a resource file. Contains the base icon and all its variants.
impl Default for Icon {
    fn default() -> Icon {
        Icon {
            base: '?',
            corner_tl: None,
            corner_tr: None,
            corner_bl: None,
            corner_br: None,
            vertical: None,
            horizontal: None,
            tee_l: None,
            tee_r: None,
            tee_u: None,
            tee_d: None,
            cap_l: None,
            center: None,
            cap_r: None,
            cap_u: None,
            cap_d: None,
            door_closed: None,
            door_open: None,
            door_frame: None,
            door_vertical: None,
            door_vertical_open: None,
            windows: Vec::new(),
            variants: Vec::new(),
        }
    }
}

impl Icon {
    /// get a char for a connected tile, given its connections in cardinal directions
    pub fn ch(&self, above: bool, below: bool, left: bool, right: bool) -> char {
        match (above, below, left, right) {
            (true, true, false, false) => self.vertical.unwrap_or(self.base),
            (false, false, true, true) => self.horizontal.unwrap_or(self.base),
            (false, true, false, true) => self.corner_tl.unwrap_or(self.base),
            (false, true, true, false) => self.corner_tr.unwrap_or(self.base),
            (true, false, false, true) => self.corner_bl.unwrap_or(self.base),
            (true, false, true, false) => self.corner_br.unwrap_or(self.base),
            (true, true, true, false) => self.tee_l.unwrap_or(self.base),
            (true, true, false, true) => self.tee_r.unwrap_or(self.base),
            (true, false, true, true) => self.tee_u.unwrap_or(self.base),
            (false, true, true, true) => self.tee_d.unwrap_or(self.base),
            (true, true, true, true) => self.center.unwrap_or(self.base),
            (false, false, false, true) => self.cap_l.unwrap_or(self.base),
            (false, false, true, false) => self.cap_r.unwrap_or(self.base),
            (false, true, false, false) => self.cap_d.unwrap_or(self.base),
            (true, false, false, false) => self.cap_u.unwrap_or(self.base),
            (false, false, false, false) => self.base,
        }
    }

    /// get the base char
    pub fn base_ch(&self) -> char {
        self.base
    }

    #[allow(unused)]
    /// for wall tiles, get a door by state and orientation
    pub fn door_ch(&self, state: DoorState, orientation: DoorOrientation) -> char {
        match orientation {
            DoorOrientation::Vertical => {
                match state {
                    DoorState::Open => self.door_vertical_open.unwrap_or(self.base),
                    // there is no difference between door closed/frame state
                    _ => self.door_vertical.unwrap_or(self.base),
                }
            }
            DoorOrientation::Horizontal => match state {
                DoorState::Open => self.door_open.unwrap_or(self.base),
                DoorState::Closed => self.door_open.unwrap_or(self.base),
                DoorState::Frame => self.door_open.unwrap_or(self.base),
            },
        }
    }

    #[allow(unused)]
    /// for wall tiles, get one of the windows
    pub fn window_ch(&self, sample: f32) -> char {
        use crate::util::choose;
        choose(&self.windows, sample).unwrap_or(self.base)
    }

    #[allow(unused)]
    /// get a count of the number of windows
    pub fn window_len(&self) -> usize {
        self.windows.len()
    }

    /// choose one of the variant tiles (if available)
    pub fn variant_ch(&self, sample: f32) -> char {
        use crate::util::choose;
        choose(&self.variants, sample).unwrap_or(self.base)
    }
}
