use ball::BallPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_window_title_diagnostics::WindowTitleLoggerDiagnosticsPlugin;
use controls::ControlsPlugin;
use fps_camera::FirstPersonCameraPlugin;
use prelude::*;
use world::WorldPlugin;

mod ball;
mod ball_starter;
mod controls;
mod fps_camera;
mod prelude;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(WindowTitleLoggerDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin)
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
