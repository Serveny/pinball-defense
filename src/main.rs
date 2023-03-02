use ball::BallPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use controls::ControlsPlugin;
use prelude::*;
use world::WorldPlugin;

mod ball;
mod ball_starter;
mod controls;
mod prelude;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(ControlsPlugin)
        .add_startup_system(setup_graphics)
        .run();
}

#[derive(Component)]
struct Camera;

fn setup_graphics(mut cmds: Commands) {
    cmds.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 200., 200.),
        ..default()
    })
    .insert(Camera);
    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
}
