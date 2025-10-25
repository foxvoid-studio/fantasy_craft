use macroquad::prelude::*;
use parry2d::shape::{SharedShape, Cuboid};
use parry2d::na::Vector2;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: Vec2,
    pub scale: Vec2
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            rotation: Vec2::new(0.0, 0.0),
            scale: Vec2::new(1.0, 1.0)
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum BodyType {
    Static,
    Dynamic,
    Kinematic
}

#[derive(Debug)]
pub struct RigidBody {
    pub body_type: BodyType,
    pub velocity: Vec2
}

impl RigidBody {
    pub fn new(body_type: BodyType) -> Self {
        Self {
            body_type,
            velocity: Vec2::ZERO
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Collider {
    pub shape: SharedShape,
    pub half_extents: Vec2
}

impl Collider {
    pub fn new_box(width: f32, height: f32) -> Self {
        Self {
            shape: SharedShape::new(Cuboid::new(Vector2::new(width / 2.0, height / 2.0))),
            half_extents: vec2(width / 2.0, height / 2.0)
        }
    }
}

#[derive(Debug)]
pub struct Velocity(pub Vec2);

#[derive(Debug)]
pub struct Speed(pub f32);
