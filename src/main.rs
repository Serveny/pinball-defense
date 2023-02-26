use ball::BallPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use prelude::*;
use world::WorldPlugin;

mod ball;
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
        .add_startup_system(setup_graphics)
        .run();
}

fn setup_graphics(mut cmds: Commands) {
    cmds.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 200., 200.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    cmds.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
}
