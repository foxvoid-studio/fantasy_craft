use engine::prelude::*;

use crate::systems::{npc_behavior_system, player_update};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Stage::Update, player_update);
    }
}

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(Stage::Update, npc_behavior_system);
    }
}
