use assets::PinballDefenseAssets;
use ball::BallPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_window_title_diagnostics::WindowTitleLoggerDiagnosticsPlugin;
use controls::ControlsPlugin;
use fps_camera::FirstPersonCameraPlugin;
use prelude::*;
use world::WorldPlugin;

mod assets;
mod ball;
mod ball_starter;
mod controls;
mod flipper;
mod fps_camera;
mod prelude;
mod world;

#[derive(States, PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    Ingame,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Ingame),
        )
        .add_collection_to_loading_state::<_, PinballDefenseAssets>(GameState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        //.add_plugin(WindowTitleLoggerDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(FirstPersonCameraPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(ControlsPlugin)
        .add_startup_system(setup_graphics)
        .run();
}

#[derive(Component)]
struct Camera;

fn setup_graphics(mut cmds: Commands) {
    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
}
