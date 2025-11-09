use engine::prelude::*;

#[derive(Debug, Clone)]
pub struct FilesMenuButton;

pub struct FilesMenuButtonLoader;

impl ComponentLoader for FilesMenuButtonLoader {
    fn load(&self, ctx: &mut Context, entity: hecs::Entity, _data: &serde_json::Value) {
        ctx.world.insert_one(entity, FilesMenuButton).expect("Failed to insert FilesMenuButton");
    }
}

pub fn files_menu_button_clicked(ctx: &mut Context) {
    for (_, (gui_button, _)) in ctx.world.query::<(&GuiButton, &FilesMenuButton)>().iter() {
        if gui_button.just_clicked {
            println!("Files Menu Opening");
        }
    }
}

pub struct FilesMenuPlugin;

impl Plugin for FilesMenuPlugin {
    fn build(&self, app: &mut App) {
        app.scene_loader
            .register("FilesMenuButton", Box::new(FilesMenuButtonLoader));

        app.
            add_system(Stage::Update, files_menu_button_clicked);
    }
}
