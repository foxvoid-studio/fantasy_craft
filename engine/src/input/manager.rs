use std::collections::HashMap;
// We remove std::fs because it is not supported in WASM
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

    /// Loads bindings from a JSON string content.
    /// This method is platform-agnostic (works on PC and Web).
    /// The file reading must be done by the caller (App.rs) using load_string().
    pub fn load_from_string(&mut self, json_content: &str) {
        // We use match to handle errors gracefully without crashing the game
        let json_bindings: HashMap<String, Vec<String>> = match serde_json::from_str(json_content) {
            Ok(data) => data,
            Err(e) => {
                error!("InputManager: Failed to parse bindings JSON. Error: {}", e);
                return;
            }
        };

        for (action, inputs) in json_bindings {
            for input_str in inputs {
                if let Some(variant) = self.parse_input_string(&input_str) {
                    self.bind(&action, variant)
                }
                else {
                    // Use error!/warn! macro for better logging in WASM console
                    warn!("InputManager: Unknown input key '{}' for action '{}'", input_str, action);
                }
            }
        }

        info!("InputManager: Bindings loaded successfully.");
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
            
            // Common Keys
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
