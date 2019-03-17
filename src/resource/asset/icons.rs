//! # Icon Module
//! Manages icons and their location on the sprite sheet. Intended to work with
//! a bitmap font, uses unicode scalar values to look up icons on the sheet.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Copy, Clone, Serialize, Deserialize)]
#[allow(unused)]
pub enum DoorState {
    Closed,
    Open,
    Frame,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[allow(unused)]
pub enum DoorOrientation {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct CharPos([u32; 2]);

impl CharPos {
    pub fn to_char(self) -> char {
        char::from(self)
    }
}

impl std::ops::Add<[u32; 2]> for CharPos {
    type Output = CharPos;
    fn add(self, other: [u32; 2]) -> Self::Output {
        CharPos([self.0[0] + other[0], self.0[1] + other[1]])
    }
}

impl std::convert::From<[u32; 2]> for CharPos {
    fn from(pos: [u32; 2]) -> CharPos {
        CharPos(pos)
    }
}

use std::convert::TryFrom;
impl From<CharPos> for char {
    fn from(c_pos: CharPos) -> char {
        let code: u32 = (c_pos.0[1] * 16) + c_pos.0[0];
        char::try_from(code).unwrap_or('?')
    }
}

impl From<char> for CharPos {
    fn from(ch: char) -> CharPos {
        let digit = u32::from(ch);
        let y = digit / 16;
        let x = digit % 16;
        println!("GOT {}, {}, {}", digit, x, y);
        CharPos([x, y])
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConnectDir {
    CornerTopLeft,
    CornerTopRight,
    CornerBottomLeft,
    CornerBottomRight,
    Vertical,
    Horizontal,
    TeeUp,
    TeeDown,
    TeeLeft,
    TeeRight,
    Center,
    CapUp,
    CapDown,
    CapLeft,
    CapRight,
}

type IconConnectMap = HashMap<ConnectDir, Icon>;

type VariantList = Vec<Icon>;

#[derive(Clone, Serialize, Deserialize)]
pub struct Icon {
    /// base tile (no connections in any direction)
    base: CharPos,
    #[serde(default)]
    connections: IconConnectMap,
    #[serde(default)]
    /// variants on the base character which should be chosen at random and have no specific
    /// connections
    variants: VariantList,
}

/// An icon spec, loaded from a resource file. Contains the base icon and all its variants.
impl Default for Icon {
    fn default() -> Icon {
        Icon {
            base: CharPos::from('?'),
            connections: HashMap::new(),
            variants: Vec::new(),
        }
    }
}

impl Icon {
    pub fn new(base: CharPos) -> Icon {
        Icon {
            base,
            connections: HashMap::new(),
            variants: Vec::new(),
        }
    }
    /// get a char for a connected tile, given its connections in cardinal directions
    pub fn connected(&self, above: bool, below: bool, left: bool, right: bool) -> &Icon {
        match (above, below, left, right) {
            (true, true, false, false) => {
                self.connections.get(&ConnectDir::Vertical).unwrap_or(self)
            }
            (false, false, true, true) => self
                .connections
                .get(&ConnectDir::Horizontal)
                .unwrap_or(self),
            (false, true, false, true) => self
                .connections
                .get(&ConnectDir::CornerTopLeft)
                .unwrap_or(self),
            (false, true, true, false) => self
                .connections
                .get(&ConnectDir::CornerTopRight)
                .unwrap_or(self),
            (true, false, false, true) => self
                .connections
                .get(&ConnectDir::CornerBottomLeft)
                .unwrap_or(self),
            (true, false, true, false) => self
                .connections
                .get(&ConnectDir::CornerBottomRight)
                .unwrap_or(self),
            (true, true, true, false) => self.connections.get(&ConnectDir::TeeLeft).unwrap_or(self),
            (true, true, false, true) => {
                self.connections.get(&ConnectDir::TeeRight).unwrap_or(self)
            }
            (true, false, true, true) => self.connections.get(&ConnectDir::TeeUp).unwrap_or(self),
            (false, true, true, true) => self.connections.get(&ConnectDir::TeeDown).unwrap_or(self),
            (true, true, true, true) => self.connections.get(&ConnectDir::Center).unwrap_or(self),
            (false, false, false, true) => {
                self.connections.get(&ConnectDir::CapLeft).unwrap_or(self)
            }
            (false, false, true, false) => {
                self.connections.get(&ConnectDir::CapRight).unwrap_or(self)
            }
            (false, true, false, false) => self.connections.get(&ConnectDir::CapUp).unwrap_or(self),
            (true, false, false, false) => {
                self.connections.get(&ConnectDir::CapDown).unwrap_or(self)
            }
            (false, false, false, false) => self,
        }
    }

    /// get the char representation of the icon
    pub fn ch(&self) -> char {
        self.base.to_char()
    }

    /// choose one of the variant tiles (if available)
    pub fn variant(&self, sample: f32) -> &Icon {
        use crate::util::choose_ref;
        choose_ref(&self.variants, sample).unwrap_or(self)
    }
}

/// An intermediary structure that maps a standard 4x4 icon layout to
/// an Icon with Connect4 layout.
struct IconConnect4 {
    base: CharPos,
}

impl IconConnect4 {
    pub fn new(base: CharPos) -> IconConnect4 {
        IconConnect4 { base }
    }
}

impl From<char> for IconConnect4 {
    fn from(ch: char) -> IconConnect4 {
        IconConnect4 { base: ch.into() }
    }
}

impl From<IconConnect4> for Icon {
    fn from(ic4: IconConnect4) -> Icon {
        let mut map: IconConnectMap = HashMap::new();
        map.insert(ConnectDir::CapLeft, Icon::new(ic4.base + [1, 0]));
        map.insert(ConnectDir::Horizontal, Icon::new(ic4.base + [2, 0]));
        map.insert(ConnectDir::CapRight, Icon::new(ic4.base + [3, 0]));

        map.insert(ConnectDir::CapUp, Icon::new(ic4.base + [0, 1]));
        map.insert(ConnectDir::CornerTopLeft, Icon::new(ic4.base + [1, 1]));
        map.insert(ConnectDir::TeeDown, Icon::new(ic4.base + [2, 1]));
        map.insert(ConnectDir::CornerTopRight, Icon::new(ic4.base + [3, 1]));

        map.insert(ConnectDir::Vertical, Icon::new(ic4.base + [0, 2]));
        map.insert(ConnectDir::TeeRight, Icon::new(ic4.base + [1, 2]));
        map.insert(ConnectDir::Center, Icon::new(ic4.base + [2, 2]));
        map.insert(ConnectDir::TeeLeft, Icon::new(ic4.base + [3, 2]));

        map.insert(ConnectDir::CapDown, Icon::new(ic4.base + [0, 3]));
        map.insert(ConnectDir::CornerBottomLeft, Icon::new(ic4.base + [1, 3]));
        map.insert(ConnectDir::TeeUp, Icon::new(ic4.base + [2, 3]));
        map.insert(ConnectDir::CornerBottomRight, Icon::new(ic4.base + [3, 3]));

        Icon {
            base: ic4.base,
            connections: map,
            variants: Vec::new(),
        }
    }
}

/// Storage format for a set of icons.
#[derive(Serialize, Deserialize)]
pub struct IconSet {
    /// characters are a series of 4 icons facing north, south, east, and west
    characters: HashMap<String, [u32; 2]>,
    /// a 4-way connected tile with a standardized layout (see tiles.png for example)
    connected4: HashMap<String, [u32; 2]>,
    /// a simple tile with only one variant
    simple: HashMap<String, [u32; 2]>,
    /// an icon with a list of variants, the third array parameter is the variant count
    variant: HashMap<String, [u32; 3]>,
}

impl IconSet {
    pub fn process(&self) -> Vec<(String, Icon)> {
        let mut set: Vec<(String, Icon)> = Vec::new();
        for (name, pos) in &self.characters {
            let mut connections: IconConnectMap = HashMap::new();
            connections.insert(ConnectDir::CapDown, Icon::new(CharPos::from(*pos)));
            connections.insert(ConnectDir::CapUp, Icon::new(CharPos::from(*pos) + [1, 0]));
            connections.insert(ConnectDir::CapLeft, Icon::new(CharPos::from(*pos) + [2, 0]));
            connections.insert(
                ConnectDir::CapRight,
                Icon::new(CharPos::from(*pos) + [3, 0]),
            );
            let icon = Icon {
                base: CharPos::from(*pos),
                connections,
                variants: Vec::new(),
            };
            set.push((name.clone(), icon));
        }

        for (name, pos) in &self.connected4 {
            set.push((
                name.clone(),
                Icon::from(IconConnect4::new(CharPos::from(*pos))),
            ));
        }

        for (name, pos) in &self.simple {
            set.push((name.clone(), Icon::new(CharPos::from(*pos))));
        }

        for (name, pos) in &self.variant {
            let mut variants: VariantList = Vec::new();
            for i in 0..pos[2] {
                variants.push(Icon::new(CharPos::from([pos[0] + i, pos[1]])));
            }
            let icon = Icon {
                base: CharPos::from([pos[0], pos[1]]),
                variants,
                connections: HashMap::new(),
            };
            set.push((name.clone(), icon));
        }

        set
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn icon_from_char() {
        let mut p: CharPos;
        p = CharPos::from('#');
        assert_eq!(p.0[0], 3);
        assert_eq!(p.0[1], 2);
        assert_eq!(u32::from('\u{1ff}'), 0x1ff);
        assert_eq!(0x1ff, 511);
        p = CharPos::from('\u{10}');
        assert_eq!(p.0[0], 0);
        assert_eq!(p.0[1], 1);
        p = CharPos::from('\u{ff}');
        assert_eq!(p.0[0], 15);
        assert_eq!(p.0[1], 15);
    }
    #[test]
    fn icon_to_char() {
        assert_eq!(CharPos::from('#').to_char(), '#');
        assert_eq!(CharPos::from('\u{f0}').to_char(), '\u{f0}')
    }
    #[test]
    fn icon_to_from() {
        // just a couple tests here
        assert_eq!(char::from(CharPos::from('?')), '?');
        assert_eq!(char::from(CharPos::from('1')), '1');
        assert_eq!(char::from(CharPos::from('^')), '^');
    }
    #[test]
    /// this sort of tests the connected function too by default
    fn icon_connect_4_to_icon() {
        // using a fixed position it should be easy to test
        let icon = Icon::from(IconConnect4::from('0'));
        assert_eq!(icon.connected(false, false, false, false).ch(), '0');

        assert_eq!(icon.connected(true, true, false, false).ch(), 'P');
        assert_eq!(icon.connected(false, true, false, false).ch(), '@');
        assert_eq!(icon.connected(true, false, false, false).ch(), '`');

        assert_eq!(icon.connected(false, false, true, true).ch(), '2');
        assert_eq!(icon.connected(false, false, false, true).ch(), '1');
        assert_eq!(icon.connected(false, false, true, false).ch(), '3');

        assert_eq!(icon.connected(false, true, false, true).ch(), 'A');
        assert_eq!(icon.connected(false, true, true, true).ch(), 'B');
        assert_eq!(icon.connected(false, true, true, false).ch(), 'C');

        assert_eq!(icon.connected(true, true, false, true).ch(), 'Q');
        assert_eq!(icon.connected(true, true, true, true).ch(), 'R');
        assert_eq!(icon.connected(true, true, true, false).ch(), 'S');

        assert_eq!(icon.connected(true, false, false, true).ch(), 'a');
        assert_eq!(icon.connected(true, false, true, true).ch(), 'b');
        assert_eq!(icon.connected(true, false, true, false).ch(), 'c');
    }
}
