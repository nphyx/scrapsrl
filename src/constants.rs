use tcod::colors::Color;

pub const TILE_HEIGHT: i32 = 16;
pub const TILE_WIDTH: i32 = 16;

pub const SIDEBAR_WIDTH: i32 = 25;
pub const DIALOG_WIDTH: i32 = 32;

pub const SCREEN_HEIGHT: i32 = 1080 / TILE_HEIGHT - 1;
pub const SCREEN_WIDTH: i32 = 1920 / TILE_WIDTH;

pub const MAP_WIDTH: usize = (SCREEN_WIDTH - SIDEBAR_WIDTH) as usize;
pub const MAP_HEIGHT: usize = (SCREEN_HEIGHT - 1) as usize;

pub const DEFAULT_BG: Color = Color { r: 0, g: 12, b: 9 };
pub const DEFAULT_FG: Color = Color {
    r: 225,
    g: 255,
    b: 232,
};

pub const CHUNK_RADIUS: u8 = 2;

pub const TEMPLATE_DIR: &str = "./resource/template";
pub const ICON_DIR: &str = "./resource";
pub const SPRITE_MAP: &str = "./tiles.png";
pub const SPRITE_WIDTH: i32 = 16;
pub const SPRITE_HEIGHT: i32 = 64;

/// world is always square, this is the length of a side
pub const WORLD_SIZE: usize = 250;
