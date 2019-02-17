use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct IconTemplate {
    base: char,
    #[serde(default)]
    corner_tl: Option<char>,
    #[serde(default)]
    corner_tr: Option<char>,
    #[serde(default)]
    corner_bl: Option<char>,
    #[serde(default)]
    corner_br: Option<char>,
    #[serde(default)]
    vertical: Option<char>,
    #[serde(default)]
    horizontal: Option<char>,
    #[serde(default)]
    tee_l: Option<char>,
    #[serde(default)]
    tee_r: Option<char>,
    #[serde(default)]
    tee_u: Option<char>,
    #[serde(default)]
    tee_d: Option<char>,
    #[serde(default)]
    cap_l: Option<char>,
    #[serde(default)]
    center: Option<char>,
    #[serde(default)]
    cap_r: Option<char>,
    #[serde(default)]
    cap_u: Option<char>,
    #[serde(default)]
    cap_d: Option<char>,
}

impl Default for IconTemplate {
    fn default() -> IconTemplate {
        IconTemplate {
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
        }
    }
}

impl IconTemplate {
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
            (false, true, false, false) => self.cap_u.unwrap_or(self.base),
            (true, false, false, false) => self.cap_d.unwrap_or(self.base),
            (false, false, false, false) => self.base,
        }
    }
    pub fn base_ch(&self) -> char {
        self.base
    }
}
