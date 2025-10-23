use hecs::World;
use macroquad::prelude::*;

mod assets;
mod asset_server;
mod camera;
mod components;
mod systems;

use crate::asset_server::AssetServer;
use crate::camera::Camera;
use crate::components::{AnimationComponent, Behavior, BehaviorComponent, DirectionComponent, NpcTag, PlayerTag, Speed, StateComponent, Transform, Velocity};
use crate::components::{Direction, State};
use systems::*;

#[macroquad::main("Fantasy Craft")]
async fn main() {
    let mut asset_server = AssetServer::new();

    asset_server.load_assets_from_file("resources/assets.json")
        .await
        .expect("Failed to load assets from JSON file");

    let mut world = World::new();
    let initial_position = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
    world.spawn((
        Transform {
            position: initial_position,
            ..Default::default()
        },
        Velocity(Vec2::ZERO),
        Speed(100.0),
        DirectionComponent(Direction::Down),
        StateComponent(State::Idle),
        AnimationComponent("player_base_idle_down".to_string()),
        PlayerTag
    ));

    world.spawn((
        Transform {
            position: Vec2::new(100.0, 100.0),
            ..Default::default()
        },
        Speed(100.0),
        DirectionComponent(Direction::Down),
        StateComponent(State::Idle),
        AnimationComponent("farmer_idle_down".to_string()),
        BehaviorComponent(Behavior::Wander),
        NpcTag { name: "farmer".to_string(), wander_time: 0.0, wander_target_duration: 0.0 }
    ));

    let mut game_camera = Camera::new(initial_position);

    loop {
        let dt = get_frame_time();

        clear_background(LIGHTGRAY);

        player_update(&mut world);
        npc_behavior_system(&mut world, dt);
        movement_system(&mut world, dt);
        update_animations(&mut world, dt, &mut asset_server);

        if let Some((_, transform)) = world.query::<&Transform>().with::<&PlayerTag>().iter().next() {
            game_camera.update(transform.position, dt);
            game_camera.set_camera();
        }

        render_system(&mut world, &mut asset_server);

        game_camera.unset_camera();

        // draw gui here

        next_frame().await
    }
}
