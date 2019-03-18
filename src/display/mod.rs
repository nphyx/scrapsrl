use super::component::*;
use super::constants::*;
use super::resource::*;
use super::util::colors::*;
use super::util::{clamp, distance, Coord};
use tcod::console::Root;
use tcod::map::FovAlgorithm;
use tcod::{
    input::KeyPressFlags,
    input::{Key, KeyCode::*},
    BackgroundFlag, Console, FontLayout, FontType, Map, RootConsole, TextAlignment,
};

mod ui;

type TColor = tcod::colors::Color;

use super::constants::{
    DEFAULT_BG, DEFAULT_FG, MAP_HEIGHT, MAP_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
};

pub struct Display {
    pub root: Root,
    pub map: Map,
}

#[derive(SystemData)]
pub struct DisplayData<'a> {
    // I'll take one of everything
    characters: ReadStorage<'a, Character>,
    colors: ReadStorage<'a, Colors>,
    cursors: ReadStorage<'a, Cursor>,
    descriptions: ReadStorage<'a, Description>,
    icons: ReadStorage<'a, IconRef>,
    orientations: ReadStorage<'a, Orientation>,
    players: ReadStorage<'a, Player>,
    positions: ReadStorage<'a, Pos>,
    regions: ReadStorage<'a, Region>,
    solids: ReadStorage<'a, Solid>,
    state: Write<'a, GameState>,
    world: Read<'a, WorldState>,
    maps: Read<'a, RegionMaps>,
    assets: Read<'a, Assets>,
    collisions: Read<'a, CollisionMaps>,
    ui_queue: Read<'a, UIQueue>,
    input: Write<'a, UserInput>,
}

impl Display {
    /// initialize the display
    pub fn new() -> Display {
        let mut root = RootConsole::initializer()
            .font(SPRITE_MAP, FontLayout::AsciiInRow)
            .font_type(FontType::Greyscale)
            .font_dimensions(SPRITE_WIDTH, SPRITE_HEIGHT)
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("SCRAPS: Bug Hunter")
            .init();
        root.set_default_background(DEFAULT_BG);
        root.set_default_foreground(DEFAULT_FG);
        root.clear();

        use tcod::system::set_fps;
        set_fps(60);

        Display {
            root,
            map: Map::new(MAP_WIDTH as i32, MAP_HEIGHT as i32),
        }
    }
}

use specs::{Join, Read, ReadStorage, System, Write};
impl<'a> System<'a> for Display {
    type SystemData = DisplayData<'a>;

    /// the drawing the game megafunction, what a disaster area
    fn run(&mut self, mut data: Self::SystemData) {
        self.accept_input(&mut data);

        self.root.set_fullscreen(data.state.fullscreen);
        // wipe screen and prepare for new draw
        self.root.clear();
        self.root.set_default_background(DEFAULT_BG);
        self.root.set_default_foreground(DEFAULT_FG);

        let dot_dot_dot = ((data.state.frame / 15) % 4) as usize;
        match data.state.stage {
            GameStage::LoadingAssets => {
                self.render_splash("Loading Assets", dot_dot_dot);
                return;
            }
            GameStage::Initializing => {
                self.render_splash("Initializing", dot_dot_dot);
                self.accept_input(&mut data);
                return;
            }
            _ => match data.state.render_mode {
                RenderMode::Normal => {
                    self.render_map_normal(&data);
                    self.render_ui(&data);
                }
                RenderMode::Collision => {
                    self.render_map_collision(&data);
                    self.render_ui(&data);
                }
            },
        }
        self.root.flush();
    }
}

impl Display {
    fn accept_input(&mut self, data: &mut DisplayData) {
        let key_input = self.root.check_for_keypress(KeyPressFlags::all());
        match key_input {
            // we don't match modifier keys as an input
            Some(Key { code: Control, .. })
            | Some(Key { code: Alt, .. })
            | Some(Key { code: Shift, .. }) => {}
            // only match when pressed = on, tcod fires on down + up
            Some(Key { pressed: true, .. }) => {
                data.input.set(key_input);
                if data.input.get().is_some() {
                    loop {
                        if self.root.check_for_keypress(KeyPressFlags::all()).is_none() {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
        data.state.close_game = self.root.window_closed() || data.state.close_game;
    }

    fn render_splash(&mut self, title: &str, dot_dot_dot: usize) {
        self.root.set_alignment(TextAlignment::Left);
        self.root.print_rect(
            SCREEN_WIDTH / 2 - 7,
            SCREEN_HEIGHT / 2,
            SCREEN_WIDTH,
            1,
            format!("{}{:.<2$}", title, "", dot_dot_dot),
        );
        self.root.flush();
    }

    fn render_map_normal<'a>(&mut self, data: &DisplayData) {
        let mut player_pos: Pos = Pos::default();
        let mut player_region: Region = Region::default();

        for (region, pos, character, _player) in (
            &data.regions,
            &data.positions,
            &data.characters,
            &data.players,
        )
            .join()
        {
            ui::draw_sidebar_frame(&self.root, &data.assets);
            ui::draw_stats(&self.root, &data.assets, character);
            ui::draw_status_bar(&self.root, character, &data.state);
            player_pos = *pos;
            player_region = *region;
            ui::draw_worldmap(
                &self.root,
                &data.assets,
                player_region,
                &data.world,
                &data.state,
            );
        }
        let map = data.maps.get(player_region);

        // update fov map before computing fov
        for (pos, tile) in map.iter() {
            self.map
                .set(pos.x as i32, pos.y as i32, tile.transparent, tile.walkable)
        }

        // Compute the FOV
        self.map.compute_fov(
            player_pos.x as i32,
            player_pos.y as i32,
            SCREEN_WIDTH,
            true,
            FovAlgorithm::Basic,
        );

        // draw all tiles
        for (pos, tile) in map.iter() {
            self.root.put_char_ex(
                pos.x as i32,
                pos.y as i32,
                tile.icon,
                TColor::from(tile.fg),
                TColor::from(tile.bg),
            );
        }

        // draw all npcs, also snag the one under the cursor if applicable
        for (region, pos, icon, color, ..) in (
            &data.regions,
            &data.positions,
            &data.icons,
            &data.colors,
            !&data.players,
        )
            .join()
        {
            let ipos: Coord<i32> = (*pos).into();
            if self.map.is_in_fov(ipos.x, ipos.y) && *region == player_region {
                self.root.put_char(
                    ipos.x,
                    ipos.y,
                    data.assets.get_icon(&icon.name).ch(),
                    BackgroundFlag::None,
                );
                self.root
                    .set_char_foreground(ipos.x, ipos.y, TColor::from(color.fg));
            }
        }

        self.draw_player(&data); //orientations, positions, icons, colors, players, assets);

        // TODO compute time of day adjustment, sunset gradient, and moon phase :D
        let time_of_day_rel = data.world.time_relative();

        let light = Color::new(178, 162, 72);
        let day_ambient = Color::new(225, 255, 225);
        let evening_ambient = Color::new(255, 92, 92);
        let night_ambient = Color::new(138, 128, 255);
        let ambient = if time_of_day_rel > 0.5 {
            lerp(
                day_ambient,
                evening_ambient,
                1.0 - ((time_of_day_rel - 0.5) * 2.0).powf(2.0),
            )
        } else {
            lerp(
                night_ambient,
                evening_ambient,
                (time_of_day_rel * 2.0).powf(2.0),
            )
        };
        let night = Color::new(12, 12, 12);

        // lighting pass
        for (pos, _) in map.iter() {
            let ipos: Coord<i32> = pos.into();
            let orig_fg = Color::from(self.root.get_char_foreground(ipos.x, ipos.y));
            let orig_bg = Color::from(self.root.get_char_background(ipos.x, ipos.y));
            let mut fg = orig_fg;
            let mut bg = orig_bg;
            let dist = distance(player_pos, pos);
            let light_radius = 20.0;
            // this figures out the intensity and radius of the player-emitted light area
            let rel_dist =
                (clamp(0.000_001, light_radius, light_radius - dist) / light_radius).powf(2.0);

            // apply night time shading
            fg = lerp(multiply(fg, night), fg, 0.1 + time_of_day_rel);
            bg = lerp(multiply(bg, night), bg, 0.1 + time_of_day_rel);
            // ambient light enhancement
            fg = soft_light(fg, ambient, 1.0);
            bg = soft_light(bg, ambient, 0.5);
            if self.map.is_in_fov(ipos.x, ipos.y) {
                // apply light
                if time_of_day_rel < 0.5 {
                    fg = lerp(fg, color_dodge(orig_fg, light), rel_dist);
                    bg = lerp(bg, color_dodge(orig_bg, light), rel_dist);
                }
            } else {
                // desaturate areas that are out of fov
                bg = desaturate(bg, 0.65);
                fg = desaturate(fg, 0.65);
            }
            self.root
                .set_char_foreground(ipos.x, ipos.y, TColor::from(fg));
            self.root
                .set_char_background(ipos.x, ipos.y, TColor::from(bg), BackgroundFlag::Set);
        }

        // draw in the cursor highlight
        for (pos, ..) in (&data.positions, &data.cursors).join() {
            self.root.set_char_background(
                pos.x as i32,
                pos.y as i32,
                TColor::new(110, 180, 144),
                BackgroundFlag::Overlay,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    /// render in collision mode, showing solids in a lighter background color
    /// (also ignores fov so it's a wallhack)
    fn render_map_collision<'a>(&mut self, data: &DisplayData) {
        let mut player_region: Region = Region::default();

        for (region, character, _player) in (&data.regions, &data.characters, &data.players).join()
        {
            ui::draw_sidebar_frame(&self.root, &data.assets);
            ui::draw_stats(&self.root, &data.assets, character);
            ui::draw_status_bar(&self.root, character, &data.state);
            player_region = *region;
        }
        let map = &data.maps.get(player_region);

        // draw all tiles
        for (pos, tile) in map.iter() {
            let collision = &data.collisions.get(player_region, pos);
            let bg: Color = if !collision && tile.walkable {
                Color::new(32, 32, 32)
            } else {
                Color::new(180, 180, 180)
            };
            self.root.put_char_ex(
                pos.x as i32,
                pos.y as i32,
                tile.icon,
                TColor::from(tile.fg),
                TColor::from(bg),
            );
        }

        // draw all npcs, also snag the one under the cursor if applicable
        for (region, pos, icon, color, ..) in (
            &data.regions,
            &data.positions,
            &data.icons,
            &data.colors,
            !&data.players,
        )
            .join()
        {
            if *region == player_region {
                self.root.put_char(
                    pos.x as i32,
                    pos.y as i32,
                    data.assets.get_icon(&icon.name).ch(),
                    BackgroundFlag::None,
                );
                self.root
                    .set_char_foreground(pos.x as i32, pos.y as i32, TColor::from(color.fg));
            }
        }

        for (region, pos, _solid, ..) in (&data.regions, &data.positions, &data.solids).join() {
            if *region == player_region {
                self.root.set_char_background(
                    pos.x as i32,
                    pos.y as i32,
                    TColor::from(Color::new(180, 180, 180)),
                    BackgroundFlag::Set,
                );
            }
        }

        self.draw_player(&data);

        // draw in the cursor highlight
        for (pos, ..) in (&data.positions, &data.cursors).join() {
            self.root.set_char_background(
                pos.x as i32,
                pos.y as i32,
                TColor::new(110, 180, 144),
                BackgroundFlag::Overlay,
            );
        }
    }

    fn draw_player(&mut self, data: &DisplayData) {
        // draw player, make sure it ends up on top
        for (orientation, pos, icon, color, ..) in (
            &data.orientations,
            &data.positions,
            &data.icons,
            &data.colors,
            &data.players,
        )
            .join()
        {
            let mut north = false;
            let mut south = false;
            let mut east = false;
            let mut west = false;
            match orientation.dir {
                Direction::North => {
                    north = true;
                }
                Direction::South => {
                    south = true;
                }
                Direction::East => {
                    east = true;
                }
                Direction::West => {
                    west = true;
                }
            }
            self.root.put_char(
                pos.x as i32,
                pos.y as i32,
                data.assets
                    .get_icon(&icon.name)
                    .connected(north, south, east, west)
                    .ch(),
                BackgroundFlag::None,
            );
            self.root
                .set_char_foreground(pos.x as i32, pos.y as i32, TColor::from(color.fg))
        }
    }

    /// renders UI elements, done after map draw passes
    fn render_ui<'a>(&mut self, data: &DisplayData) {
        let mut cursor_pos: Pos = Pos::default();
        let mut player_region: Region = Region::default();
        let mut has_cursor: bool = false;

        for (region, character, _player) in (&data.regions, &data.characters, &data.players).join()
        {
            ui::draw_sidebar_frame(&self.root, &data.assets);
            ui::draw_stats(&self.root, &data.assets, character);
            ui::draw_status_bar(&self.root, character, &data.state);
            player_region = *region;
        }

        // find the cursor position
        for (pos, _cursor) in (&data.positions, &data.cursors).join() {
            cursor_pos.x = pos.x;
            cursor_pos.y = pos.y;
            has_cursor = true;
        }

        // get the current map
        let map = &data.maps.get(player_region);

        // find an entity under the cursor, if it exists
        if has_cursor && self.map.is_in_fov(cursor_pos.x as i32, cursor_pos.y as i32) {
            let mut found_entity = false;
            for (region, pos, icon, color, desc) in (
                &data.regions,
                &data.positions,
                &data.icons,
                &data.colors,
                &data.descriptions,
            )
                .join()
            {
                if *pos == cursor_pos && *region == player_region {
                    ui::draw_entity_info(
                        &self.root,
                        data.assets.get_icon(&icon.name).ch(),
                        *color,
                        desc,
                    );
                    found_entity = true;
                }
            }
            if !found_entity {
                ui::draw_tile_info(&self.root, map.get(cursor_pos).unwrap());
            }
        }

        if let Some(widget) = data.ui_queue.get() {
            ui::draw_centered_dialog(&self.root, widget);
        }
    }
}
