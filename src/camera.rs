use macroquad::prelude::*;

pub struct Camera {
    current_position: Vec2,
    pub lerp_factor: f32,
    pub zoom: f32
}

impl Camera {
    pub fn new(initial_position: Vec2) -> Self {
        Self {
            current_position: initial_position,
            lerp_factor: 8.0,
            zoom: 1.0
        }
    }

    pub fn update(&mut self, target_position: Vec2, dt: f32) {
        let lerp_speed = 1.0 - (-self.lerp_factor * dt).exp();
        self.current_position = self.current_position.lerp(target_position, lerp_speed);
    }
    
    pub fn set_camera(&self) {
        let camera = Camera2D {
            target: self.current_position,
            rotation: 0.0,
            zoom: vec2(self.zoom * 2.0 / screen_width(), self.zoom * 2.0 / screen_height()),
            ..Default::default()
        };
        set_camera(&camera);
    }
    
    pub fn unset_camera(&self) {
        set_default_camera();
    }
}
