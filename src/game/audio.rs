use super::{ball::PinBall, EventState, GameState};
use crate::prelude::*;
use crate::utils::Music;
use crate::{settings::SoundSettings, utils::Sound};
use bevy::audio::Volume;
use rand::Rng;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvent>()
            .add_systems(
                OnEnter(GameState::Init),
                (play_music, play_ball_rolling_sound),
            )
            .add_systems(OnEnter(GameState::Pause), pause_sounds)
            .add_systems(OnEnter(GameState::Ingame), resume_sounds)
            .add_systems(
                Update,
                (clean_up_sound_system, ball_rolling_sound_system)
                    .run_if(in_state(GameState::Ingame)),
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
    BallHitsFoundation,
    BallHitsWall,
    EnemyReachEnd,
    TowerBuild,
    TowerUpgradeRange,
    TowerUpgradeDamage,
    CounterTick,
    BallStarterCharge,
    BallStarterFire,
    PbMenuFadeIn,
    PbMenuFadeOut,
    PbMenuActive,
}

impl SoundEvent {
    fn sound_bundle<'a>(&self, assets: &'a PinballDefenseAudioAssets) -> (SoundHandle<'a>, f32) {
        use SoundEvent::*;
        let handle = match *self {
            BallSpawn => SoundHandle::Single(&assets.ball_release),
            FlipperPress => SoundHandle::Various(&assets.flipper_press),
            FlipperRelease => SoundHandle::Various(&assets.flipper_release),
            TowerHit => SoundHandle::Various(&assets.tower_hit),
            BallHitsEnd => SoundHandle::Single(&assets.ball_hits_end),
            BallHitsEnemy => SoundHandle::Single(&assets.ball_hits_enemy),
            BallHitsFoundation => SoundHandle::Various(&assets.ball_hits_foundation),
            BallHitsWall => SoundHandle::Various(&assets.ball_hits_wall),
            EnemyReachEnd => SoundHandle::Single(&assets.enemy_reach_end),
            TowerBuild => SoundHandle::Single(&assets.tower_build),
            TowerUpgradeRange => SoundHandle::Single(&assets.tower_upgrade_range),
            TowerUpgradeDamage => SoundHandle::Single(&assets.tower_upgrade_damage),
            CounterTick => SoundHandle::Various(&assets.analog_counter_tick),
            BallStarterCharge => SoundHandle::Single(&assets.ball_starter_charge),
            BallStarterFire => SoundHandle::Single(&assets.ball_starter_fire),
            PbMenuFadeIn => SoundHandle::Single(&assets.pb_menu_fade_in),
            PbMenuFadeOut => SoundHandle::Single(&assets.pb_menu_fade_out),
            PbMenuActive => SoundHandle::Single(&assets.pb_menu_active),
        };
        let speed = match handle {
            SoundHandle::Single(_) => 1.,
            SoundHandle::Various(_) => rand::thread_rng().gen_range(0.9..1.1),
        };
        (handle, speed)
    }
}

fn pause_sounds(q_sound: Query<&AudioSink, With<Sound>>) {
    for sound in &q_sound {
        sound.pause();
    }
}

fn resume_sounds(q_sound: Query<&AudioSink, With<Sound>>) {
    for sound in &q_sound {
        sound.play();
    }
}

fn on_play_sound_fx_system(
    mut cmds: Commands,
    mut evr: EventReader<SoundEvent>,
    assets: Res<PinballDefenseAudioAssets>,
    sound_sett: Res<SoundSettings>,
) {
    if sound_sett.fx_volume > 0. {
        for ev in evr.read() {
            let s = ev.sound_bundle(&assets);
            cmds.spawn(sound(s.0, sound_sett.fx_volume, s.1));
        }
    }
}

enum SoundHandle<'a> {
    Single(&'a Handle<AudioSource>),
    Various(&'a Handles<AudioSource>),
}

fn play_music(
    mut cmds: Commands,
    assets: Res<PinballDefenseAudioAssets>,
    sound_sett: Res<SoundSettings>,
) {
    cmds.spawn((
        AudioPlayer(assets.background_music.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::new(sound_sett.music_volume)),
        Music,
    ));
}

fn sound(handle: SoundHandle, vol: f32, speed: f32) -> impl Bundle {
    (
        Name::new("Sound"),
        Sound,
        AudioPlayer(match handle {
            SoundHandle::Single(handle) => handle.clone(),
            SoundHandle::Various(handles) => handles.choose().clone(),
        }),
        PlaybackSettings::ONCE
            .with_volume(Volume::new(vol))
            .with_speed(speed),
    )
}

fn clean_up_sound_system(mut cmds: Commands, q_sound: Query<(Entity, &AudioSink), With<Sound>>) {
    for (id, sound) in q_sound.iter() {
        if sound.empty() {
            cmds.entity(id).despawn();
        }
    }
}

#[derive(Component)]
struct BallRollingSound;

fn play_ball_rolling_sound(mut cmds: Commands, assets: Res<PinballDefenseAudioAssets>) {
    cmds.spawn((
        Name::new("Ball Rolling Sound"),
        AudioPlayer(assets.ball_rolling.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::new(0.)),
        BallRollingSound,
        Sound,
    ));
}

fn ball_rolling_sound_system(
    mut q_rolling_sound: Query<&AudioSink, With<BallRollingSound>>,
    q_ball: Query<&LinearVelocity, With<PinBall>>,
    sound_sett: Res<SoundSettings>,
) {
    if let Ok(sound) = q_rolling_sound.get_single_mut() {
        if let Some(vel) = q_ball.iter().next() {
            let linvel = vel.length().abs() / 12.;
            sound.set_volume(linvel * sound_sett.fx_volume);
            let speed = 0.9 + linvel / 2.;
            sound.set_speed(speed);
        } else {
            sound.set_volume(0.);
        }
    }
}
