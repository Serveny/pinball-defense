use assets::PinballDefenseAssets;
use ball::BallPlugin;
use ball_camera::BallCameraPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_window_title_diagnostics::WindowTitleLoggerDiagnosticsPlugin;
use controls::ControlsPlugin;
use fps_camera::FirstPersonCameraPlugin;
use prelude::*;
use tower::TowerPlugin;
use world::WorldPlugin;

mod assets;
mod ball;
mod ball_camera;
mod ball_starter;
mod controls;
mod flipper;
mod fps_camera;
mod prelude;
mod tower;
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
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Ingame),
        )
        .add_collection_to_loading_state::<_, PinballDefenseAssets>(GameState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(WindowTitleLoggerDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(FirstPersonCameraPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(BallCameraPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(ControlsPlugin);
    add_rapier(&mut app);
    app.add_startup_system(setup_graphics).run();
}

fn add_rapier(app: &mut App) {
    let rapier_cfg = RapierConfiguration {
        timestep_mode: TimestepMode::Variable {
            max_dt: 1. / 128.,
            time_scale: 1.,
            substeps: 1,
        },
        ..default()
    };
    app.insert_resource(rapier_cfg)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default());

    // Only show debug data in debug mode
    #[cfg(debug_assertions)]
    app.add_plugin(RapierDebugRenderPlugin::default());
}

#[derive(Component)]
struct Camera;

fn setup_graphics(mut cmds: Commands) {
    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
}
