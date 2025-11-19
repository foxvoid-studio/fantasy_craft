use std::{collections::HashMap, fs};
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputVariant {
    Key(KeyCode),
    Mouse(MouseButton)
}

#[derive(Default)]
pub struct InputManager {
    bindings: HashMap<String, Vec<InputVariant>>
}

impl InputManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind(&mut self, action: &str, input: InputVariant) {
        self.bindings
            .entry(action.to_string())
            .or_insert_with(Vec::new)
            .push(input);
    }

    /// Loads bindings from a JSON file.
    /// Expected format: { "ActionName": ["KeyName", "MouseName"], ... }
    pub fn load_from_file(&mut self, path: &str) {
        let content = fs::read_to_string(path)
            .expect(&format!("Failed to read bindings file: {}", path));

        let json_bindings: HashMap<String, Vec<String>> = serde_json::from_str(&content)
            .expect("Failed to parse bindings JSON");

        for (action, inputs) in json_bindings {
            for input_str in inputs {
                if let Some(variant) = self.parse_input_string(&input_str) {
                    self.bind(&action, variant)
                }
                else {
                    eprintln!("Warning: Unknown input key '{}' for action '{}'", input_str, action);
                }
            }
        }

        println!("Input bindings loaded from {}", path);
    }

    pub fn is_action_down(&self, action: &str) -> bool {
        if let Some(inputs) = self.bindings.get(action) {
            for input in inputs {
                match input {
                    InputVariant::Key(k) => if is_key_down(*k) { return true; },
                    InputVariant::Mouse(b) => if is_mouse_button_down(*b) { return true; }
                }
            }
        }
        false
    }

    pub fn is_action_just_pressed(&self, action: &str) -> bool {
        if let Some(inputs) = self.bindings.get(action) {
            for input in inputs {
                match input {
                    InputVariant::Key(k) => if is_key_pressed(*k) { return true; },
                    InputVariant::Mouse(b) => if is_mouse_button_pressed(*b) { return true; }
                }
            }
        }
        false
    }

    fn parse_input_string(&self, s: &str) -> Option<InputVariant> {
        match s {
            // Mouse
            "MouseLeft" => Some(InputVariant::Mouse(MouseButton::Left)),
            "MouseRight" => Some(InputVariant::Mouse(MouseButton::Right)),
            "MouseMiddle" => Some(InputVariant::Mouse(MouseButton::Middle)),
            
            // Common Keys (Add more as needed)
            "Space" => Some(InputVariant::Key(KeyCode::Space)),
            "Escape" => Some(InputVariant::Key(KeyCode::Escape)),
            "Enter" => Some(InputVariant::Key(KeyCode::Enter)),
            "Tab" => Some(InputVariant::Key(KeyCode::Tab)),
            "LeftShift" => Some(InputVariant::Key(KeyCode::LeftShift)),
            "RightShift" => Some(InputVariant::Key(KeyCode::RightShift)),
            "LeftControl" => Some(InputVariant::Key(KeyCode::LeftControl)),
            
            // Arrows
            "Up" => Some(InputVariant::Key(KeyCode::Up)),
            "Down" => Some(InputVariant::Key(KeyCode::Down)),
            "Left" => Some(InputVariant::Key(KeyCode::Left)),
            "Right" => Some(InputVariant::Key(KeyCode::Right)),
            
            // Letters
            "A" => Some(InputVariant::Key(KeyCode::A)),
            "B" => Some(InputVariant::Key(KeyCode::B)),
            "C" => Some(InputVariant::Key(KeyCode::C)),
            "D" => Some(InputVariant::Key(KeyCode::D)),
            "E" => Some(InputVariant::Key(KeyCode::E)),
            "F" => Some(InputVariant::Key(KeyCode::F)),
            "G" => Some(InputVariant::Key(KeyCode::G)),
            "H" => Some(InputVariant::Key(KeyCode::H)),
            "I" => Some(InputVariant::Key(KeyCode::I)),
            "J" => Some(InputVariant::Key(KeyCode::J)),
            "K" => Some(InputVariant::Key(KeyCode::K)),
            "L" => Some(InputVariant::Key(KeyCode::L)),
            "M" => Some(InputVariant::Key(KeyCode::M)),
            "N" => Some(InputVariant::Key(KeyCode::N)),
            "O" => Some(InputVariant::Key(KeyCode::O)),
            "P" => Some(InputVariant::Key(KeyCode::P)),
            "Q" => Some(InputVariant::Key(KeyCode::Q)),
            "R" => Some(InputVariant::Key(KeyCode::R)),
            "S" => Some(InputVariant::Key(KeyCode::S)),
            "T" => Some(InputVariant::Key(KeyCode::T)),
            "U" => Some(InputVariant::Key(KeyCode::U)),
            "V" => Some(InputVariant::Key(KeyCode::V)),
            "W" => Some(InputVariant::Key(KeyCode::W)),
            "X" => Some(InputVariant::Key(KeyCode::X)),
            "Y" => Some(InputVariant::Key(KeyCode::Y)),
            "Z" => Some(InputVariant::Key(KeyCode::Z)),

            // Numbers
            "0" => Some(InputVariant::Key(KeyCode::Key0)),
            "1" => Some(InputVariant::Key(KeyCode::Key1)),
            "2" => Some(InputVariant::Key(KeyCode::Key2)),
            "3" => Some(InputVariant::Key(KeyCode::Key3)),
            "4" => Some(InputVariant::Key(KeyCode::Key4)),
            "5" => Some(InputVariant::Key(KeyCode::Key5)),
            "6" => Some(InputVariant::Key(KeyCode::Key6)),
            "7" => Some(InputVariant::Key(KeyCode::Key7)),
            "8" => Some(InputVariant::Key(KeyCode::Key8)),
            "9" => Some(InputVariant::Key(KeyCode::Key9)),

            _ => None,
        }
    }
}
