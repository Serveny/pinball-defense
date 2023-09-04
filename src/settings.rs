use crate::prelude::*;

#[derive(Resource, Reflect, Default)]
pub struct GraphicsSettings {
    pub is_shadows: bool,
    pub is_hdr: bool,
    pub bloom_intensity: f32,
}

impl GraphicsSettings {
    #[allow(dead_code)]
    pub fn low() -> Self {
        Self {
            is_shadows: false,
            is_hdr: false,
            bloom_intensity: 0.,
        }
    }

    #[allow(dead_code)]
    pub fn high() -> Self {
        Self {
            is_shadows: true,
            is_hdr: true,
            bloom_intensity: 0.01,
        }
    }
}

#[derive(Resource, Reflect, Default)]
pub struct SoundSettings {
    pub music_volume: f32,
    pub fx_volume: f32,
}

//#[derive(Resource)]
//pub struct AudioSettings {
//volume_background_music: f32,
//}
