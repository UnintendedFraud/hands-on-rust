mod player_input;
mod map_render;
mod entity_render;

pub use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    return Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build();
}
