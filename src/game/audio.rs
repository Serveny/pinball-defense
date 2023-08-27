use super::GameState;
use crate::prelude::*;
use bevy::audio::VolumeLevel;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySoundEvent>()
            .add_systems(OnEnter(GameState::Ingame), play_music)
            .add_systems(
                Update,
                play_sound_system.run_if(in_state(GameState::Ingame)),
            );
    }
}

fn play_music(mut cmds: Commands, assets: Res<PinballDefenseAudioAssets>) {
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

#[derive(Event)]
pub enum PlaySoundEvent {
    BallSpawn,
}

fn play_sound_system(mut evr: EventReader<PlaySoundEvent>) {
    for ev in evr.iter() {
        use PlaySoundEvent::*;
        match *ev {
            BallSpawn => (),
        }
    }
}
