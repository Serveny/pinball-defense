use super::audio::{Music, Sound};
use super::GameState;
use crate::prelude::*;
use crate::settings::{GraphicsSettings, SoundSettings};
use bevy::core_pipeline::bloom::BloomSettings;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (on_changed_sound_settings, on_changed_graphics_settings)
                .run_if(in_state(GameState::Pause)),
        );
    }
}

fn on_changed_sound_settings(
    sound_sett: Res<SoundSettings>,
    mut q_sound: Query<&mut AudioSink, (With<Sound>, Without<Music>)>,
    mut q_music: Query<&mut AudioSink, (With<Music>, Without<Sound>)>,
) {
    if sound_sett.is_changed() {
        for sound in q_sound.iter_mut() {
            sound.set_volume(sound_sett.fx_volume);
        }
        for music in q_music.iter_mut() {
            music.set_volume(sound_sett.music_volume);
        }
    }
}

fn on_changed_graphics_settings(
    g_sett: Res<GraphicsSettings>,
    mut q_spot: Query<&mut SpotLight>,
    mut q_point: Query<&mut PointLight>,
    mut q_cam: Query<&mut Camera>,
    mut q_bloom: Query<&mut BloomSettings>,
) {
    if g_sett.is_changed() {
        q_point.for_each_mut(|mut light| light.shadows_enabled = g_sett.is_shadows);
        q_spot.for_each_mut(|mut light| light.shadows_enabled = g_sett.is_shadows);
        q_cam.for_each_mut(|mut cam| cam.hdr = g_sett.is_hdr);
        q_bloom.for_each_mut(|mut bloom_sett| *bloom_sett = g_sett.bloom.clone());
    }
}
