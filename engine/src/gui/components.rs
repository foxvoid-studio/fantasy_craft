use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct TextDisplay {
    pub text: String,
    pub font_size: f32,
    pub color: Color,
    pub screen_space: bool
}

impl Default for TextDisplay {
    fn default() -> Self {
        Self {
            text: String::new(),
            font_size: 30.0,
            color: BLACK,
            screen_space: true
        }
    }
}
