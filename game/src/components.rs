use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Behavior {
    Stand,
    Wander
}

#[derive(Debug)]
pub struct BehaviorComponent(pub Behavior);

#[derive(Debug)]
pub struct PlayerTag;

#[derive(Debug)]
pub struct NpcTag {
    pub name: String,
    pub wander_time: f32,
    pub wander_target_duration: f32
}
