use hecs::World;
use macroquad::prelude::*;
use crate::core::context::Context;
use crate::core::schedule::{Schedule, Stage, System};
use crate::core::asset_server::AssetServer;
use crate::core::plugins::Plugin;

pub struct App {
    pub context: Context,
    pub schedule: Schedule,
    pub window_conf: Conf
}

impl App {
    pub fn new(conf: Conf) -> Self {
        let world = World::new();
        let asset_server = AssetServer::new();

        App {
            context: Context {
                world,
                asset_server,
                dt: 0.0
            },
            schedule: Schedule::new(),
            window_conf: conf
        }
    }

    pub fn add_system(&mut self, stage: Stage, system: System) -> &mut Self {
        self.schedule.add_system(stage, system);
        self
    }

    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        plugin.build(self);
        self
    }

    pub async fn run(mut self) {
        self.context.asset_server
            .load_assets_from_file("resources/assets.json")
            .await
            .expect("Failed to load assets from JSON file");

        self.schedule.run_stage(Stage::StartUp, &mut self.context);

        let mut fps_timer: f32 = 0.0;
        let mut displayed_fps: i32 = get_fps();

        loop {
            self.context.dt = get_frame_time();
            fps_timer += self.context.dt;

            if fps_timer >= 1.0 {
                displayed_fps = get_fps();
                fps_timer = 0.0;
            }

            clear_background(LIGHTGRAY);

            self.schedule.run_stage(Stage::Update, &mut self.context);
            self.schedule.run_stage(Stage::PostUpdate, &mut self.context);
            self.schedule.run_stage(Stage::Render, &mut self.context);
            self.schedule.run_stage(Stage::PostRender, &mut self.context);

            set_default_camera();
            let fps_text = format!("FPS: {}", displayed_fps);
            draw_text(&fps_text, 10.0, 25.0, 30.0, BLACK);

            next_frame().await
        }
    }
}