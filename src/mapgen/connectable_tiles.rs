use std::collections::HashMap;

pub struct ConnectableChar {
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

pub fn build_connectables() -> HashMap<char, ConnectableChar> { 
  let mut connectables = HashMap::new();
  connectables.insert('+', ConnectableChar{
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
  connectables.insert('\u{256c}', ConnectableChar{
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
  return connectables;
}
