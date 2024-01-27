mod map;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::map::*;
    pub use crate::player::*;
    
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        return Self {
            map: Map::new(),
            player: Player::new(Point { x: SCREEN_WIDTH / 2, y: SCREEN_HEIGHT / 2 }),
        };
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Dungeon Crawler").with_fps_cap(30.0).build()?;

    return main_loop(context, State::new());
}