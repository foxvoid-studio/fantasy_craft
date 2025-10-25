use crate::{gui::systems::gui_render_system, prelude::{Plugin, Stage}};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app
            .add_system(Stage::GuiRender, gui_render_system);
    }
}