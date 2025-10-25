use macroquad::prelude::*;

#[derive(Debug)]
pub struct CameraComponent {
    pub lerp_factor: f32,
    pub zoom: f32
}

#[derive(Debug)]
pub struct MainCamera;

#[derive(Debug)]
pub struct CameraTarget;