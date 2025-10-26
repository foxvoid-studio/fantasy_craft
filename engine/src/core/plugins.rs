use crate::{core::app::App, hierarchy::systems::hierarchy_update_system, prelude::{collider_debug_render_system, AnimationPlugin, Camera2dPlugin, GuiPlugin, PhysicsPlugin, Stage, TiledMapPlugin}};

pub trait Plugin {
    fn build(&self, app: &mut App);
}

pub struct Default2dPlugin;

impl Plugin for Default2dPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PhysicsPlugin)
            .add_plugin(Camera2dPlugin)
            .add_system(Stage::Update, hierarchy_update_system)
            .add_plugin(TiledMapPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(GuiPlugin);
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Stage::PostRender, collider_debug_render_system);
    }
}
