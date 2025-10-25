use crate::prelude::{Plugin, Stage};
use crate::graphics::tiled_map::systems::tiled_map_render_system;

pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app
            .add_system(Stage::Render, tiled_map_render_system);
    }
}
