use hecs::World;
use macroquad::prelude::*;
use futures::{FutureExt, future::BoxFuture};
use crate::core::context::Context;
use crate::core::focus::InputFocus;
use crate::core::schedule::{Schedule, Stage, System};
use crate::core::asset_server::AssetServer;
use crate::core::plugins::Plugin;
use crate::graphics::splash_screen::{animate_splash_screen, despawn_splash_screen, setup_splash_screen};
use crate::prelude::Spritesheet;

pub struct App {
    pub context: Context,
    pub schedule: Schedule,
    pub window_conf: Conf,
    show_splash_screen: bool
}

impl App {
    pub fn new(conf: Conf) -> Self {
        let world = World::new();
        let asset_server = AssetServer::new();

        App {
            context: Context {
                world,
                asset_server,
                dt: 0.0,
                collision_events: Vec::new(),
                prev_mouse_pos: Vec2::ZERO,
                input_focus: InputFocus::default()
            },
            schedule: Schedule::new(),
            window_conf: conf,
            show_splash_screen: true
        }
    }

    pub fn with_splash_screen(&mut self, enabled: bool) -> &mut Self {
        self.show_splash_screen = enabled;
        self
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
        const SPLASH_MIN_DURATION: f64 = 3.0;

        let mut maybe_asset_server: Option<AssetServer> = None;

        if self.show_splash_screen {
            // --- Splash setup ---
            self.context.asset_server.add_spritesheet(
                "logo_engine".to_string(),
                Spritesheet::new(
                    load_texture("resources/textures/logo_engine.png").await.unwrap(),
                    1024.0,
                    1024.0,
                ),
            );

            setup_splash_screen(&mut self.context);
            let start_time = get_time();

            // --- Création d'une future pour le chargement ---
            let mut loading_asset_server = AssetServer::new();

            let mut load_future: BoxFuture<'static, (AssetServer, Result<(), Box<dyn std::error::Error>>)> =
                Box::pin(async move {
                    let result = loading_asset_server.load_assets_from_file("resources/assets.json").await;
                    (loading_asset_server, result)
                });

            let mut assets_loaded = false;

            // --- Boucle du splash ---
            loop {
                self.context.dt = get_frame_time();
                clear_background(BLACK);

                // Animation + rendu
                animate_splash_screen(&mut self.context);
                self.schedule.run_stage(Stage::Render, &mut self.context);
                self.schedule.run_stage(Stage::PostRender, &mut self.context);
                set_default_camera();
                self.schedule.run_stage(Stage::GuiRender, &mut self.context);

                next_frame().await;

                let elapsed = get_time() - start_time;
                let duration_done = elapsed >= SPLASH_MIN_DURATION;

                if !assets_loaded {
                    if let Some((loaded_server, result)) = load_future.as_mut().now_or_never() {
                        result.expect("Failed to load assets from JSON file");
                        maybe_asset_server = Some(loaded_server);
                        assets_loaded = true;
                    }
                }

                if assets_loaded && duration_done {
                    break;
                }
            }

            despawn_splash_screen(&mut self.context);
        } else {
            // --- Pas de splash : on charge directement les assets ---
            let mut asset_server = AssetServer::new();
            asset_server
                .load_assets_from_file("resources/assets.json")
                .await
                .expect("Failed to load assets from JSON file");

            maybe_asset_server = Some(asset_server);
        }

        // Réinjection du AssetServer chargé
        if let Some(loaded_server) = maybe_asset_server {
            self.context.asset_server.merge(loaded_server);
            self.context.asset_server.finalize_textures().await;
        }

        self.context.asset_server.prepare_loaded_tiledmap().await;

        // --- Démarrage du jeu ---
        self.schedule.run_stage(Stage::StartUp, &mut self.context);

        loop {
            self.context.dt = get_frame_time();
            clear_background(LIGHTGRAY);

            self.schedule.run_stage(Stage::Update, &mut self.context);
            self.schedule.run_stage(Stage::PostUpdate, &mut self.context);
            self.schedule.run_stage(Stage::Render, &mut self.context);
            self.schedule.run_stage(Stage::PostRender, &mut self.context);

            set_default_camera();
            self.schedule.run_stage(Stage::GuiRender, &mut self.context);

            self.context.collision_events.clear();
            self.context.prev_mouse_pos = mouse_position().into();

            next_frame().await;
        }
    }
}
