use crate::prelude::{Plugin, Stage, tilemap_layer_render_system};
use crate::graphics::tiled_map::systems::tilemap_render_system;

pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app
            .add_system(Stage::Render, tilemap_layer_render_system)
            .add_system(Stage::Render, tilemap_render_system);
    }
}
