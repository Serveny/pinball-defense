use crate::prelude::*;
use bevy::core_pipeline::bloom::{BloomCompositeMode, BloomSettings};

#[derive(Resource)]
pub struct GraphicsSettings {
    pub is_shadows: bool,
    pub is_hdr: bool,
    pub bloom: BloomSettings,
}

impl GraphicsSettings {
    #[allow(dead_code)]
    pub fn low() -> Self {
        Self {
            is_shadows: false,
            is_hdr: false,
            bloom: BloomSettings::default(),
        }
    }

    #[allow(dead_code)]
    pub fn high() -> Self {
        Self {
            is_shadows: true,
            is_hdr: true,
            bloom: BloomSettings {
                intensity: 0.01,
                composite_mode: BloomCompositeMode::Additive,
                ..default()
            },
        }
    }
}

#[derive(Resource, Default)]
pub struct SoundSettings {
    pub music_volume: f32,
    pub fx_volume: f32,
}

//#[derive(Resource)]
//pub struct AudioSettings {
//volume_background_music: f32,
//}
