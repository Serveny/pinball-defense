use super::GameState;
use crate::prelude::*;
use bevy::audio::{PlaybackMode, Volume, VolumeLevel};

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

#[derive(Event)]
pub enum PlaySoundEvent {
    BallSpawn,
    FlipperPress,
    FlipperRelease,
    TowerHit,
    BallHitsEnd,
    EnemyDeath,
    EnemyReachEnd,
}

impl PlaySoundEvent {
    fn sound_bundle<'a>(&self, assets: &'a PinballDefenseAudioAssets) -> SoundHandle<'a> {
        use PlaySoundEvent::*;
        match *self {
            BallSpawn => SoundHandle::Single(&assets.ball_release),
            FlipperPress => SoundHandle::Various(&assets.flipper_press),
            FlipperRelease => SoundHandle::Various(&assets.flipper_release),
            TowerHit => SoundHandle::Various(&assets.tower_hit),
            BallHitsEnd => SoundHandle::Single(&assets.ball_hits_end),
            EnemyDeath => SoundHandle::Single(&assets.enemy_death),
            EnemyReachEnd => SoundHandle::Single(&assets.enemy_reach_end),
        }
    }
}

fn play_sound_system(
    mut cmds: Commands,
    mut evr: EventReader<PlaySoundEvent>,
    assets: Res<PinballDefenseAudioAssets>,
) {
    for ev in evr.iter() {
        cmds.spawn(sound(ev.sound_bundle(&assets)));
    }
}

enum SoundHandle<'a> {
    Single(&'a Handle<AudioSource>),
    Various(&'a Handles<AudioSource>),
}

const VOLUME: f32 = 0.6;

fn play_music(mut cmds: Commands, assets: Res<PinballDefenseAudioAssets>) {
    cmds.spawn(AudioBundle {
        source: assets.background_music.clone(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Absolute(VolumeLevel::new(VOLUME)),
            speed: 1.,
            paused: false,
        },
    });
}

#[derive(Component)]
struct Sound;

fn sound(handle: SoundHandle) -> impl Bundle {
    (
        Name::new("Sound"),
        Sound,
        AudioBundle {
            source: match handle {
                SoundHandle::Single(handle) => handle.clone(),
                SoundHandle::Various(handles) => handles.choose().clone(),
            },
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                volume: Volume::Absolute(VolumeLevel::new(VOLUME)),
                speed: 1.,
                paused: false,
            },
        },
    )
}

fn clean_up_sound_system(mut cmds: Commands, q_sound: Query<(Entity, &AudioSink), With<Sound>>) {
    for (id, sound) in q_sound.iter() {
        if sound.empty() {
            cmds.entity(id).despawn();
        }
    }
}
