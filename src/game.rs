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
pub use self::save::{
    list_saves, load_game, next_save_path, save_game, spawn_save_screenshot, thumbnail_path,
};
use self::ui::UiState;
use self::world::{PinballWorld, spawn_pinball_world};
use crate::AppState;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use controls::ControlsPlugin;
pub use controls::KeyboardControls;
use enemy::EnemyPlugin;
use events::PinballEventsPlugin;
use pinball_menu::PinballMenuPlugin;
use player_life::PlayerLifePlugin;
use progress::ProgressPlugin;
use save::SavePlugin;
use stats::StatsPlugin;
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
mod save;
mod stats;
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
            .add_message::<PauseGameEvent>()
            .add_message::<ResumeGameEvent>()
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
                SavePlugin,
                StatsPlugin,
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
            .add_systems(
                Update,
                save_and_exit_system.run_if(in_state(GameState::Ingame)),
            )
            .add_systems(OnEnter(AppState::Game), init_game)
            .add_systems(OnExit(AppState::Game), clean_up_on_exit_game)
            .add_systems(
                OnEnter(GameState::GameOver),
                (game_over::spawn, pause_on_game_over),
            )
            .add_systems(
                Update,
                (
                    crate::menu::tools::menu_btn::system,
                    crate::menu::tools::menu_btn::focus_system,
                    crate::menu::tools::hover_focus_system,
                    crate::menu::gamepad::navigation_system,
                    crate::menu::gamepad::activate_system,
                    crate::menu::focus_first_widget,
                    game_over::action_handler,
                )
                    .run_if(in_state(GameState::GameOver)),
            )
            .add_systems(OnExit(GameState::GameOver), reset);
    }
}

fn init_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Init);
}

fn save_and_exit_system(
    mut cmds: Commands,
    args: Res<crate::CliArgs>,
    mut exit_ev: MessageWriter<AppExit>,
) {
    let Some(path) = &args.save else {
        return;
    };
    save_game(&mut cmds, path.clone());
    exit_ev.write(AppExit::Success);
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
    cmds.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
        affects_lightmapped_meshes: true,
    });
    // directional 'sun' light
    cmds.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadow_maps_enabled: g_sett.is_shadows,
            ..default()
        },
        Transform::from_xyz(0.0, -0.0, 2.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
    ));
}

#[derive(Message)]
pub struct PauseGameEvent;

fn on_set_pause_system(
    evr: MessageReader<PauseGameEvent>,
    mut set_game_state: ResMut<NextState<GameState>>,
    mut physics_time: ResMut<Time<Physics>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    if !evr.is_empty() {
        log!("⏸️ Pause Game");
        set_game_state.set(GameState::Pause);
        pause_times(&mut physics_time, &mut virtual_time);
    }
}

#[derive(Message)]
pub struct ResumeGameEvent;

fn on_resume_game_system(
    evr: MessageReader<ResumeGameEvent>,
    mut set_game_state: ResMut<NextState<GameState>>,
    mut physics_time: ResMut<Time<Physics>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    if !evr.is_empty() {
        log!("️⏯️ Resume Game");
        set_game_state.set(GameState::Ingame);
        resume_times(&mut physics_time, &mut virtual_time);
    }
}

fn pause_times(physics_time: &mut Time<Physics>, virtual_time: &mut Time<Virtual>) {
    physics_time.pause();
    virtual_time.pause();
}

fn resume_times(physics_time: &mut Time<Physics>, virtual_time: &mut Time<Virtual>) {
    physics_time.unpause();
    virtual_time.unpause();
}

fn pause_on_game_over(
    mut physics_time: ResMut<Time<Physics>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    pause_times(&mut physics_time, &mut virtual_time);
}

fn reset(
    mut cmds: Commands,
    mut physics_time: ResMut<Time<Physics>>,
    mut virtual_time: ResMut<Time<Virtual>>,
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
    resume_times(&mut physics_time, &mut virtual_time);
}

fn clean_up_on_exit_game(
    mut cmds: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    mut physics_time: ResMut<Time<Physics>>,
    mut virtual_time: ResMut<Time<Virtual>>,
    q_game: Query<
        Entity,
        Or<(
            With<PinBall>,
            With<PinballWorld>,
            With<Camera>,
            With<DirectionalLight>,
        )>,
    >,
) {
    game_state.set(GameState::None);
    resume_times(&mut physics_time, &mut virtual_time);
    for entity in q_game.iter() {
        cmds.entity(entity).despawn();
    }
}
