use hecs::Bundle;
use macroquad::prelude::*;
use crate::physics::components::Transform;
use crate::gui::components::{GuiBox, GuiButton, GuiCheckbox, GuiSlider};

#[derive(Bundle, Debug)]
pub struct CheckboxBundle {
    pub transform: Transform,
    pub gui_box: GuiBox,
    pub button: GuiButton,
    pub checkbox: GuiCheckbox
}

impl Default for CheckboxBundle {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            gui_box: GuiBox {
                width: 25.0,
                height:  25.0,
                border_radius: 4.0,
                ..Default::default()
            },
            button: GuiButton  {
                normal_color: Color::new(0.9, 0.9, 0.9, 1.0),
                hovered_color: Color::new(1.0, 1.0, 1.0, 1.0),
                pressed_color: Color::new(0.8, 0.8, 0.8, 1.0),
                ..Default::default()
            },
            checkbox: GuiCheckbox {
                is_checked: false
            }
        }
    }
}

#[derive(Bundle, Debug)]
pub struct SliderBundle {
    pub transform: Transform,
    pub gui_box: GuiBox,
    pub slider: GuiSlider
}

impl Default for SliderBundle {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            gui_box: GuiBox {
                width: 200.0,
                height: 20.0,
                border_radius: 10.0,
                color: Color::new(0.3, 0.3, 0.3, 1.0),
                ..Default::default()
            },
            slider: GuiSlider::default()
        }
    }
}
