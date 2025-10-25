use macroquad::prelude::*;
use crate::prelude::Context;
use crate::graphics::tiled_map::components::TileMapComponent;

pub fn tiled_map_render_system(ctx: &mut Context) {
    for (_, tileset_comp) in ctx.world.query::<&TileMapComponent>().iter() {
        if let Some(rendered_map) = ctx.asset_server.get_renderer_map(&tileset_comp.name) {
            draw_texture_ex(
                &rendered_map.texture.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(rendered_map.width, rendered_map.height)),
                    ..Default::default()
                }
            );
        }
    }
}
