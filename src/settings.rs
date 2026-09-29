use crate::prelude::*;

#[derive(Resource, Reflect, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct GraphicsSettings {
    pub is_shadows: bool,
    pub is_hdr: bool,
    pub is_bloom: bool,
    pub is_fullscreen: bool,
}

pub const BLOOM_INTENSITY: f32 = 0.1;

impl GraphicsSettings {
    pub fn bloom_intensity(&self) -> f32 {
        if self.is_bloom { BLOOM_INTENSITY } else { 0. }
    }

    #[allow(dead_code)]
    pub fn low() -> Self {
        Self {
            is_shadows: false,
            is_hdr: true,
            is_bloom: true,
            is_fullscreen: false,
        }
    }

    #[allow(dead_code)]
    pub fn high() -> Self {
        Self {
            is_shadows: true,
            is_hdr: true,
            is_bloom: true,
            is_fullscreen: false,
        }
    }
}

#[derive(Resource, Reflect, Default)]
pub struct SoundSettings {
    pub music_volume: f32,
    pub fx_volume: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn bloom_checkbox_controls_fixed_intensity() {
        let mut graphics = GraphicsSettings::low();
        assert!(graphics.is_hdr);
        assert_eq!(graphics.bloom_intensity(), BLOOM_INTENSITY);
        graphics.is_bloom = false;
        assert_eq!(graphics.bloom_intensity(), 0.);
    }
}
