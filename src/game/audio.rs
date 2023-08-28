use super::GameState;
use crate::prelude::*;
use bevy::{audio::VolumeLevel, utils::HashMap};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySoundEvent>()
            .add_systems(OnEnter(GameState::Ingame), play_music)
            .add_systems(
                Update,
                (play_sound_system, clean_up_sound_system).run_if(in_state(GameState::Ingame)),
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

#[derive(Resource, Default)]
struct AudioIds(pub HashMap<String, Vec<Entity>>);

#[derive(Component)]
struct Sound;

fn sound(name: &str, audio_srcs: &PinballDefenseAudioSources) -> impl Bundle {
    (
        Name::new(format!("{name} FX")),
        Sound,
        AudioBundle {
            source: audio_srcs
                .0
                .get(name)
                .unwrap_or_else(|| panic!("😥 No audio src with name {name}"))
                .choose()
                .clone(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Once,
                volume: bevy::audio::Volume::Absolute(VolumeLevel::new(0.2)),
                speed: 1.,
                paused: false,
            },
        },
    )
}

#[derive(Event)]
pub enum PlaySoundEvent {
    BallSpawn,
    FlipperPress,
    FlipperRelease,
}

fn play_sound_system(
    mut cmds: Commands,
    mut evr: EventReader<PlaySoundEvent>,
    audio_srcs: Res<PinballDefenseAudioSources>,
) {
    for ev in evr.iter() {
        use PlaySoundEvent::*;
        match *ev {
            BallSpawn => cmds.spawn(sound("flipper_press", &audio_srcs)),
            FlipperPress => cmds.spawn(sound("flipper_press", &audio_srcs)),
            FlipperRelease => cmds.spawn(sound("flipper_release", &audio_srcs)),
        };
    }
}

fn clean_up_sound_system(mut cmds: Commands, q_sound: Query<(Entity, &AudioSink), With<Sound>>) {
    for (id, sound) in q_sound.iter() {
        if sound.empty() {
            cmds.entity(id).despawn();
        }
    }
}
