use std::collections::HashMap;

struct ConnectableChar {
  pub base: char,
  pub horiz: char,
  pub vert: char,
  pub corner_tl: char,
  pub corner_tr: char,
  pub corner_bl: char,
  pub corner_br: char,
  pub t_l: char,
  pub t_r: char,
  pub t_u: char,
  pub t_d: char,
  pub cross: char,
  pub cap_l: char,
  pub cap_r: char,
  pub cap_u: char,
  pub cap_d: char
}

type ConnectMap = HashMap<char, ConnectableChar>;

pub struct ConnectableChars {
  map: ConnectMap
}

impl ConnectableChars {
  /// builds a map of base tiles and the tiles they can connect to
  /// TODO move the maps out to a config file and load them here
  pub fn new() -> ConnectableChars {
    let mut map = ConnectMap::new();
    map.insert('+', ConnectableChar{
      base: '+',
      horiz: '-',
      vert: '|',
      corner_tl: '+',
      corner_tr: '+',
      corner_bl: '+',
      corner_br: '+',
      t_l: '+',
      t_r: '+',
      t_u: '+',
      t_d: '+',
      cross: 'x',
      cap_l: '<',
      cap_r: '>',
      cap_u: '^',
      cap_d: 'v' 
    });
    map.insert('\u{256c}', ConnectableChar{
      base: '\u{256c}',
      horiz: '\u{2550}',
      vert: '\u{2551}',
      corner_tl: '\u{2554}',
      corner_tr: '\u{2557}',
      corner_bl: '\u{255a}',
      corner_br: '\u{255d}',
      t_l: '\u{2563}',
      t_r: '\u{2560}',
      t_u: '\u{2569}',
      t_d: '\u{2566}',
      cross: '\u{256c}',
      cap_l: 'O',
      cap_r: 'O',
      cap_u: 'O',
      cap_d: 'O' 
    });
    ConnectableChars{map}
  }

  /// checks whether a character can be connected
  pub fn can_connect(&self, orig: &char) -> bool {
    match self.map.get(&orig) {
      Some(ch) => true,
      None => false
    }
  }

  /// determines if the first char can be connected to adjacent chars and how to map it
  /// returns the appropriate char to use
  pub fn connect(
    &self,
    orig: &char,
    up: Option<char>,
    down: Option<char>,
    left: Option<char>,
    right: Option<char>) -> Option<char> {
    let chosen: char;
    match self.map.get(&orig) {
      Some(connectable) => {
        let mut matched = (false, false, false, false);
        match up {
          Some(ch) => if ch == connectable.base { matched.0 = true; },
          None => {}
        }
        match down {
          Some(ch) => if ch == connectable.base { matched.1 = true; },
          None => {}
        }
        match left {
          Some(ch) => if ch == connectable.base { matched.2 = true; },
          None => {}
        }
        match right {
          Some(ch) => if ch == connectable.base { matched.3 = true; },
          None => {}
        }
        match matched {
          (true, true, false, false) => chosen = connectable.vert,
          (false, false, true, true) => chosen = connectable.horiz,
          (false, true, false, true) => chosen = connectable.corner_tl,
          (false, true, true, false) => chosen = connectable.corner_tr,
          (true, false, false, true) => chosen = connectable.corner_bl,
          (true, false, true, false) => chosen = connectable.corner_br,
          (true, true, true, false) => chosen = connectable.t_l,
          (true, true, false, true) => chosen = connectable.t_r,
          (true, false, true, true) => chosen = connectable.t_u,
          (false, true, true, true) => chosen = connectable.t_d,
          (true, true, true, true) => chosen = connectable.cross,
          (false, false, false, true) => chosen = connectable.cap_l,
          (false, false, true, false) => chosen = connectable.cap_r,
          (false, true, false, false) => chosen = connectable.cap_u,
          (true, false, false, false) => chosen = connectable.cap_d,
          (false, false, false, false) => chosen = connectable.base
        }
        return Some(chosen);
      },
      None => {}
    }
    None
  }
}
