use hecs::World;
use macroquad::math::Vec2;
use crate::{core::{asset_server::AssetServer, focus::InputFocus}, graphics::splash_screen::SplashScreenData, prelude::CollisionEvent};

pub struct Context {
    pub world: World,
    pub asset_server: AssetServer,
    pub dt: f32,
    pub collision_events: Vec<CollisionEvent>,
    pub prev_mouse_pos: Vec2,
    pub input_focus: InputFocus,
    pub splash_screen_data: Option<SplashScreenData>
}
