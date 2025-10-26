use crate::{core::plugins::Plugin, physics::systems::physics_system, prelude::{movement_system, Stage}};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut crate::prelude::App) {
        app
            .add_system(Stage::Update, movement_system)
            .add_system(Stage::PostUpdate, physics_system);
    }
}
