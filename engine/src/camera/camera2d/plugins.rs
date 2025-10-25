use crate::{core::plugins::Plugin, prelude::{update_camera, Stage}};

pub struct Camera2dPlugin;

impl Plugin for Camera2dPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app
            .add_system(Stage::PostUpdate, update_camera);
    }
}