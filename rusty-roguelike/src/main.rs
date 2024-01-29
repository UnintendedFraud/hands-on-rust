mod map;
mod player;
mod map_builder;
mod camera;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

const FONT_PATH: &str = "dungeonfont.png";

use prelude::*;

struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mb = MapBuilder::new(&mut rng);

        return Self {
            map: mb.map,
            player: Player::new(mb.player_start),
            camera: Camera::new(mb.player_start),
        };
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        self.player.update(ctx, &self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font(FONT_PATH, 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_PATH)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, FONT_PATH)
        .build()?;

    return main_loop(context, State::new());
}
