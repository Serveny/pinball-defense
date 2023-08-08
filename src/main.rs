use assets::PinballDefenseAssets;
use ball::BallPlugin;
use ball_camera::BallCameraPlugin;
use bevy::{app::PluginGroupBuilder, diagnostic::FrameTimeDiagnosticsPlugin};
//#[cfg(debug_assertions)]
//use bevy_debug_grid::*;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use bevy_window_title_diagnostics::WindowTitleLoggerDiagnosticsPlugin;
use controls::ControlsPlugin;
use enemy::EnemyPlugin;
use events::PinballEventsPlugin;
use fps_camera::FirstPersonCameraPlugin;
use loading::LoadingScreenPlugin;
use pinball_menu::PinballMenuPlugin;
use player_life::PlayerLifePlugin;
use prelude::*;
use progress_bar::ProgressBarPlugin;
use settings::GraphicsSettings;
use std::f32::consts::PI;
use tower::TowerPlugin;
use wave::WavePlugin;
use world::WorldPlugin;

mod assets;
mod ball;
mod ball_camera;
mod ball_starter;
mod controls;
mod enemy;
mod events;
mod flipper;
mod fps_camera;
mod loading;
mod pinball_menu;
mod player_life;
mod prelude;
mod progress_bar;
mod road;
mod settings;
mod tower;
mod utils;
mod wave;
mod world;

#[derive(States, PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    Ingame,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum CameraState {
    #[default]
    None,
    BallCamera,
    FpsCamera,
}

fn main() {
    let mut app = App::new();

    app.add_state::<GameState>()
        .add_state::<CameraState>()
        .init_resource::<IngameTime>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Ingame),
        )
        .add_collection_to_loading_state::<_, PinballDefenseAssets>(GameState::Loading)
        .add_plugins(DefaultPlugins);

    // Only show debug data in debug mode
    #[cfg(debug_assertions)]
    add_debug_plugins(&mut app);

    #[cfg(debug_assertions)]
    app.insert_resource(GraphicsSettings::low());

    #[cfg(not(debug_assertions))]
    app.insert_resource(GraphicsSettings::high());

    app.add_plugins((
        FrameTimeDiagnosticsPlugin,
        TweeningPlugin,
        WindowTitleLoggerDiagnosticsPlugin::default(),
        PinballDefensePlugins,
    ));

    add_rapier(&mut app);
    app.add_systems(Startup, setup_ambient_lights)
        .add_systems(
            Update,
            tick_ingame_timer_system.run_if(in_state(GameState::Ingame)),
        )
        .run();
}

struct PinballDefensePlugins;
impl PluginGroup for PinballDefensePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LoadingScreenPlugin)
            .add(FirstPersonCameraPlugin)
            .add(WorldPlugin)
            .add(BallPlugin)
            .add(BallCameraPlugin)
            .add(TowerPlugin)
            .add(ControlsPlugin)
            .add(PinballMenuPlugin)
            .add(PinballEventsPlugin)
            .add(ProgressBarPlugin)
            .add(EnemyPlugin)
            .add(WavePlugin)
            .add(PlayerLifePlugin)
    }
}

fn add_rapier(app: &mut App) {
    let rapier_cfg = RapierConfiguration {
        //timestep_mode: TimestepMode::Variable {
        //max_dt: 1. / 128.,
        //time_scale: 1.,
        //substeps: 1,
        //},
        //timestep_mode: TimestepMode::Fixed {
        //dt: 1. / 64.,
        //substeps: 4,
        //},
        ..default()
    };
    app.insert_resource(rapier_cfg)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct IngameTime(f32);

fn tick_ingame_timer_system(mut ig_time: ResMut<IngameTime>, time: Res<Time>) {
    **ig_time += time.delta_seconds();
}

#[cfg(debug_assertions)]
fn add_debug_plugins(app: &mut App) {
    app.add_plugins((RapierDebugRenderPlugin::default(), WorldInspectorPlugin::default()))
        //.add_plugin(DebugGridPlugin::with_floor_grid())
        ;
}

#[derive(Component)]
struct Camera;

fn setup_ambient_lights(mut cmds: Commands, g_sett: Res<GraphicsSettings>) {
    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
    // directional 'sun' light
    cmds.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 16000.0,
            shadows_enabled: g_sett.is_shadows,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
        ..default()
    });
}
