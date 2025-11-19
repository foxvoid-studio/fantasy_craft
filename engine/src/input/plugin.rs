use crate::{input::{focus::InputFocus, manager::InputManager}, prelude::{App, Plugin}};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.context.insert_resource(InputFocus::default());
        app.context.insert_resource(InputManager::new());
    }
}
