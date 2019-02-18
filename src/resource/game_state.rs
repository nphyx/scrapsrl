use crate::component::Region;

#[derive(PartialEq, Eq)]
pub enum GameStage {
    LoadingAssets,
    Initializing,
    Playing,
}

#[derive(PartialEq, Eq)]
pub enum RenderMode {
    Normal,
    Collision,
}

/// game state collects a bunch of globally needed data and coordination info in one
/// place. Doesn't make a lot of sense to make these individual resources, causes too
/// much boilerplate in Systems
pub struct GameState {
    pub stage: GameStage,
    /// if true, game will close on next game loop pass
    pub close_game: bool,
    /// frame counter
    pub frame: u32,
    /// tick counter
    pub tick: u32,
    /// fullscreen state
    pub fullscreen: bool,
    /// the game is completely paused. Nothing happens except system-level input
    /// (main menu, close game, etc)
    pub paused: bool,
    /// the game is being fast-forwarded, and will not accept player motion
    pub fast_forward: bool,

    /// when the game is not ticking, AI doesn't take its turn and time doesn't advance
    /// but some player actions can still be taken (navigate menus, inventory, look around)
    pub ticking: bool,
    /// player is using a cursor to look around
    pub looking: bool,
    /// game controls enabled (does not affect system-level input: fullscreen, quit, etc)
    pub input_enabled: bool,

    /// tracks the X,Y offset of the current map from 0, 0
    pub region: Region,

    /// rendering modes
    pub render_mode: RenderMode,
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            stage: GameStage::LoadingAssets,
            close_game: false,
            frame: 0,
            tick: 0,

            fullscreen: true,
            paused: false,
            fast_forward: false,
            input_enabled: false,
            ticking: true,

            looking: false,

            region: Region::default(),

            render_mode: RenderMode::Normal,
        }
    }
}

impl GameState {
    pub fn change_region(&mut self, x_change: i32, y_change: i32) {
        self.region.x += x_change;
        self.region.y += y_change;
        println!("changed region to {:?}", self.region);
    }
}
