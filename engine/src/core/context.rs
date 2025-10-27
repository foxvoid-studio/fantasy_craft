use hecs::World;
use macroquad::math::Vec2;
use crate::{core::asset_server::AssetServer, prelude::CollisionEvent};

pub struct Context {
    pub world: World,
    pub asset_server: AssetServer,
    pub dt: f32,
    pub collision_events: Vec<CollisionEvent>,
    pub prev_mouse_pos: Vec2
}
