use crate::{core::app::App, prelude::{AnimationPlugin, Camera2dPlugin, GuiPlugin, PhysicsPlugin, TiledMapPlugin}};

pub trait Plugin {
    fn build(&self, app: &mut App);
}

pub struct Default2dPlugin;

impl Plugin for Default2dPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PhysicsPlugin)
            .add_plugin(Camera2dPlugin)
            .add_plugin(TiledMapPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(GuiPlugin);
    }
}
