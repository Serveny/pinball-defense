use ball::BallPlugin;
//#[cfg(debug_assertions)]
//use bevy_debug_grid::*;
use self::analog_counter::AnalogCounterPlugin;
use self::audio::AudioPlugin;
use self::ball::PinBall;
use self::ball_starter::BallStarterPlugin;
use self::camera::PinballCameraPlugin;
use self::flipper::FlipperPlugin;
use self::game_over::GameOverScreen;
use self::health::HealthPlugin;
use self::level::LevelPlugin;
use self::light::LightPlugin;
use self::ui::UiState;
use self::world::{spawn_pinball_world, PinballWorld};
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::AppState;
use controls::ControlsPlugin;
pub use controls::KeyboardControls;
use enemy::EnemyPlugin;
use events::PinballEventsPlugin;
use pinball_menu::PinballMenuPlugin;
use player_life::PlayerLifePlugin;
use progress::ProgressPlugin;
use std::f32::consts::PI;
use tower::TowerPlugin;
use wave::WavePlugin;

mod analog_counter;
mod audio;
mod ball;
mod ball_starter;
mod camera;
mod cfg;
mod controls;
mod enemy;
mod events;
mod flipper;
mod game_over;
mod health;
mod level;
mod light;
mod pinball_menu;
mod player_life;
mod progress;
mod road;
mod tower;
mod ui;
mod wave;
mod world;

#[derive(States, PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
enum GameState {
    #[default]
    None,
    Init,
    Ingame,
    Pause,
    GameOver,
}

#[derive(States, PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
enum EventState {
    #[default]
    Inactive,
    Active,
}
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_state::<EventState>()
            .add_event::<PauseGameEvent>()
            .add_event::<ResumeGameEvent>()
            .init_resource::<IngameTime>()
            .add_plugins((
                AssetsPlugin,
                BallPlugin,
                PinballCameraPlugin,
                TowerPlugin,
                ControlsPlugin,
                PinballMenuPlugin,
                PinballEventsPlugin,
                ProgressPlugin,
                EnemyPlugin,
                WavePlugin,
                LevelPlugin,
                AnalogCounterPlugin,
                AudioPlugin,
            ))
            .add_plugins((
                HealthPlugin,
                PlayerLifePlugin,
                LightPlugin,
                FlipperPlugin,
                BallStarterPlugin,
                self::ui::UiPlugin,
            ))
            .add_systems(
                OnEnter(GameState::Init),
                (setup_ambient_lights, spawn_pinball_world, start_game),
            )
            .add_systems(
                Update,
                (tick_ingame_timer_system, on_set_pause_system).run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (on_resume_game_system).run_if(in_state(GameState::Pause)),
            )
            .add_systems(OnEnter(AppState::Game), init_game)
            .add_systems(OnEnter(GameState::GameOver), game_over::spawn)
            .add_systems(
                Update,
                (game_over::btn_system).run_if(in_state(GameState::GameOver)),
            )
            .add_systems(OnExit(GameState::GameOver), reset);
    }
}

fn init_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Init);
}

fn start_game(
    mut cmds: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    mut ev_state: ResMut<NextState<EventState>>,
    mut ui_state: ResMut<NextState<UiState>>,
) {
    game_state.set(GameState::Ingame);
    ev_state.set(EventState::Active);
    ui_state.set(UiState::Controls);
    cmds.insert_resource(IngameTime::default());
}

#[derive(Resource, Deref, DerefMut, Default)]
struct IngameTime(f32);

fn tick_ingame_timer_system(mut ig_time: ResMut<IngameTime>, time: Res<Time>) {
    **ig_time += time.delta_secs();
}

fn setup_ambient_lights(mut cmds: Commands, g_sett: Res<GraphicsSettings>) {
    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
        affects_lightmapped_meshes: true,
    });
    // directional 'sun' light
    cmds.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: g_sett.is_shadows,
            ..default()
        },
        Transform::from_xyz(0.0, -0.0, 2.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
    ));
}

#[derive(Event)]
pub struct PauseGameEvent;

fn on_set_pause_system(
    evr: EventReader<PauseGameEvent>,
    mut set_game_state: ResMut<NextState<GameState>>,
    mut rapier_cfg: Query<&mut RapierConfiguration>,
) {
    if let Ok(mut rapier_cfg) = rapier_cfg.single_mut() {
        if !evr.is_empty() {
            log!("⏸️ Pause Game");
            set_game_state.set(GameState::Pause);
            rapier_cfg.physics_pipeline_active = false;
        }
    }
}

#[derive(Event)]
pub struct ResumeGameEvent;

fn on_resume_game_system(
    evr: EventReader<ResumeGameEvent>,
    mut set_game_state: ResMut<NextState<GameState>>,
    mut rapier_cfg: Query<&mut RapierConfiguration>,
) {
    if let Ok(mut rapier_cfg) = rapier_cfg.single_mut() {
        if !evr.is_empty() {
            log!("️⏯️ Resume Game");
            set_game_state.set(GameState::Ingame);
            rapier_cfg.physics_pipeline_active = true;
        }
    }
}

fn reset(
    mut cmds: Commands,
    q_game_over_screen: Query<
        Entity,
        Or<(
            With<GameOverScreen>,
            With<PinBall>,
            With<PinballWorld>,
            With<Camera>,
            With<DirectionalLight>,
        )>,
    >,
) {
    q_game_over_screen
        .iter()
        .for_each(|entity| cmds.entity(entity).despawn());
}
