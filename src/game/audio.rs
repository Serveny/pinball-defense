use super::{EventState, GameState};
use crate::prelude::*;
use crate::settings::SoundSettings;
use bevy::audio::{PlaybackMode, Volume, VolumeLevel};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvent>()
            .add_systems(OnEnter(GameState::Init), play_music)
            .add_systems(
                Update,
                (clean_up_sound_system).run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (on_play_sound_fx_system).run_if(in_state(EventState::Active)),
            );
    }
}

#[derive(Event)]
pub enum SoundEvent {
    BallSpawn,
    FlipperPress,
    FlipperRelease,
    TowerHit,
    BallHitsEnd,
    BallHitsEnemy,
    EnemyReachEnd,
    TowerBuild,
    TowerUpgradeRange,
    TowerUpgradeDamage,
}

impl SoundEvent {
    fn sound_bundle<'a>(&self, assets: &'a PinballDefenseAudioAssets) -> SoundHandle<'a> {
        use SoundEvent::*;
        match *self {
            BallSpawn => SoundHandle::Single(&assets.ball_release),
            FlipperPress => SoundHandle::Various(&assets.flipper_press),
            FlipperRelease => SoundHandle::Various(&assets.flipper_release),
            TowerHit => SoundHandle::Various(&assets.tower_hit),
            BallHitsEnd => SoundHandle::Single(&assets.ball_hits_end),
            BallHitsEnemy => SoundHandle::Single(&assets.ball_hits_enemy),
            EnemyReachEnd => SoundHandle::Single(&assets.enemy_reach_end),
            TowerBuild => SoundHandle::Single(&assets.tower_build),
            TowerUpgradeRange => SoundHandle::Single(&assets.tower_upgrade_range),
            TowerUpgradeDamage => SoundHandle::Single(&assets.tower_upgrade_damage),
        }
    }
}

fn on_play_sound_fx_system(
    mut cmds: Commands,
    mut evr: EventReader<SoundEvent>,
    assets: Res<PinballDefenseAudioAssets>,
    sound_sett: Res<SoundSettings>,
) {
    if sound_sett.fx_volume > 0. {
        for ev in evr.iter() {
            cmds.spawn(sound(ev.sound_bundle(&assets), sound_sett.fx_volume));
        }
    }
}

enum SoundHandle<'a> {
    Single(&'a Handle<AudioSource>),
    Various(&'a Handles<AudioSource>),
}

#[derive(Component)]
pub struct Music;

fn play_music(
    mut cmds: Commands,
    assets: Res<PinballDefenseAudioAssets>,
    sound_sett: Res<SoundSettings>,
) {
    cmds.spawn((
        AudioBundle {
            source: assets.background_music.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::Absolute(VolumeLevel::new(sound_sett.music_volume)),
                speed: 1.,
                paused: false,
            },
        },
        Music,
    ));
}

#[derive(Component)]
pub struct Sound;

fn sound(handle: SoundHandle, vol: f32) -> impl Bundle {
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
                volume: Volume::Absolute(VolumeLevel::new(vol)),
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
