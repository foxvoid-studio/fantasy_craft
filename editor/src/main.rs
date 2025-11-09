use macroquad::prelude::*;
use engine::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Editor".to_owned(),
        window_width: 3840,
        window_height: 2160,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new(window_conf());

    app
        .with_splash_screen_enabled(true)
        .add_plugin(Default2dPlugin);

    app.run().await;
}
