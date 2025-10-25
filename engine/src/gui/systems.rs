use macroquad::prelude::*;
use crate::core::context::Context;
use crate::gui::components::TextDisplay;
use crate::physics::components::Transform;

pub fn gui_render_system(ctx: &mut Context) {
    for (_, (text_display, transform)) in ctx.world.query::<(&TextDisplay, &Transform)>().iter() {
        if text_display.screen_space {
            draw_text(
                &text_display.text,
                transform.position.x,
                transform.position.y,
                text_display.font_size,
                text_display.color
            );
        }
    }
}
