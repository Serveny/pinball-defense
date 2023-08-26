use super::GameState;
use crate::prelude::*;
use bevy::audio::VolumeLevel;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Ingame), play_music);
    }
}

fn play_music(mut cmds: Commands, assets: Res<PinballDefenseAssets>) {
    cmds.spawn(AudioBundle {
        source: assets.background_music.clone(),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.2)),
            speed: 1.,
            paused: false,
        },
    });
}

//#[derive(Component)]
//struct TowerHitSound;

//fn spawn_tower_hit_sound(mut cmds: Commands, assets: Res<PinballDefenseAssets>) {
//cmds.spawn((
//AudioBundle {
//source: assets.tower_hit_sound.clone(),
//settings: PlaybackSettings {
//mode: bevy::audio::PlaybackMode::Once,
//volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.2)),
//speed: 1.,
//paused: true,
//},
//},
//TowerHitSound,
//));
//}

//fn play_tower_hit_system(on_tower_hit: EventReader<)
