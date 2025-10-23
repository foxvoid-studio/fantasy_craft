use std::{collections::HashMap, sync::Arc};
use macroquad::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct FrameSequenceData {
    row: u32,
    count: u32,
}

#[derive(Deserialize)]
struct SpritesheetData {
    id: String,
    path: String,
    columns: u32,
    rows: u32,
}

#[derive(Deserialize)]
struct AnimationData {
    id: String,
    spritesheet_id: String,
    frame_sequence: FrameSequenceData,
    #[serde(default)] 
    flip: Option<bool>, 
}

#[derive(Deserialize)]
struct AssetFileData {
    spritesheets: Vec<SpritesheetData>,
    animations: Vec<AnimationData>,
}

use crate::assets::{Animation, AnimationKeyFrame, Spritesheet};

// --- AssetServer ---
pub struct AssetServer {
    animations: HashMap<String, Animation>,
    spritesheets: HashMap<String, Arc<Spritesheet>>
}

#[allow(dead_code)]
impl AssetServer {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            spritesheets: HashMap::new()
        }
    }

    pub fn add_spritesheet(&mut self, name: String, spritesheet: Spritesheet) {
        let arc_spritesheet = Arc::new(spritesheet);
        self.spritesheets.insert(name, arc_spritesheet);
    }

    pub fn add_animation(&mut self, name: String, animation: Animation) {
        self.animations.insert(name, animation);
    }

    pub fn get_spritesheet(&self, name: &str) -> Option<&Arc<Spritesheet>> {
        self.spritesheets.get(name)
    }

    pub fn get_animation(&self, name: &str) -> Option<&Animation> {
        self.animations.get(name)
    }

    pub fn get_animation_mut(&mut self, name: &str) -> Option<&mut Animation> {
        self.animations.get_mut(name)
    }
    
    // --- Logique de chargement ---
    pub async fn load_assets_from_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_content = std::fs::read_to_string(path)?;
        let asset_data: AssetFileData = serde_json::from_str(&json_content)?;

        // 1. Chargement des Spritesheets
        for ss_data in asset_data.spritesheets {
            let texture = load_texture(&ss_data.path).await?;

            texture.set_filter(FilterMode::Nearest);
            
            // Note: La taille des sprites est déduite de la taille totale de l'image
            let sprite_width = texture.width() / ss_data.columns as f32;
            let sprite_height = texture.height() / ss_data.rows as f32;

            let spritesheet = Spritesheet::new(texture, sprite_width, sprite_height);
            self.add_spritesheet(ss_data.id, spritesheet);
        }

        // 2. Création des Animations
        for anim_data in asset_data.animations {
            let spritesheet_arc = self.spritesheets.get(&anim_data.spritesheet_id)
                .ok_or_else(|| format!("Spritesheet '{}' not found for animation '{}'", 
                                       anim_data.spritesheet_id, anim_data.id))?
                .clone();

            let frames: Vec<AnimationKeyFrame> = (0..anim_data.frame_sequence.count)
                .map(|col| AnimationKeyFrame::new(col, anim_data.frame_sequence.row))
                .collect();

            let speed = 6.0; 
            let flip_x = anim_data.flip.unwrap_or(false);

            let animation = Animation::new(
                spritesheet_arc, 
                frames, 
                speed,
                flip_x
            );
            
            self.add_animation(anim_data.id, animation);
        }

        Ok(())
    }
}
