use crate::ball::{spawn_ball, Ball, BallSpawn};
use crate::prelude::*;
use bevy::input::mouse::MouseMotion;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(mouse_input)
            .add_system(gamepad_connections)
            .add_system(gamepad_input);
    }
}

fn mouse_input(
    buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut q_cam: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    if buttons.pressed(MouseButton::Right) {
        for ev in motion_evr.iter() {
            rotate_cam_around_middle(&mut q_cam, ev.delta, time.delta_seconds());
        }
    }
}

fn rotate_cam_around_middle(
    q_cam: &mut Query<&mut Transform, With<Camera>>,
    delta: Vec2,
    delta_seconds: f32,
) {
    let mut transform = q_cam.single_mut();
    // Order is important to prevent unintended roll
    transform.rotate_local(
        Quat::from_axis_angle(Vec3::Y, (-4. * delta_seconds * delta.x).to_radians())
            * Quat::from_axis_angle(Vec3::X, (4. * delta_seconds * delta.y).to_radians()),
    );
    transform.rotation.z = 0.;
    println!("{}", transform.rotation.xyz());
}

fn cam_walk(q_cam: &mut Query<&mut Transform, With<Camera>>, delta: Vec2, delta_seconds: f32) {
    let mut transform = q_cam.single_mut();
    transform.translation.x += delta.x * delta_seconds;
    transform.translation.z += delta.y * delta_seconds;
}

/// Simple resource to store the ID of the connected gamepad.
/// We need to know which gamepad to use for player input.
#[derive(Resource)]
struct MyGamepad(Gamepad);

fn gamepad_connections(
    mut cmds: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
    ball_spawn: Res<BallSpawn>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_ball: Query<&Ball>,
) {
    let mut is_ball_spawned = false;
    for ev in gamepad_evr.iter() {
        let id = ev.gamepad;
        match &ev.event_type {
            GamepadEventType::Connected(info) => {
                println!(
                    "New gamepad connected with ID: {:?}, name: {}",
                    id, info.name
                );

                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    cmds.insert_resource(MyGamepad(id));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == id {
                        cmds.remove_resource::<MyGamepad>();
                    }
                }
            }
            GamepadEventType::ButtonChanged(GamepadButtonType::South, z) => {
                if !is_ball_spawned && *z > 0. && q_ball.is_empty() {
                    println!("South pressed: {z} at {}", ball_spawn.0);
                    spawn_ball(&mut cmds, &mut meshes, &mut materials, ball_spawn.0);
                    is_ball_spawned = true;
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}

fn gamepad_input(
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut q_cam: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        let gamepad = gp.0;

        // The joysticks are represented using a separate axis for X and Y
        let axis_rx = GamepadAxis::new(gamepad, GamepadAxisType::RightStickX);
        let axis_ry = GamepadAxis::new(gamepad, GamepadAxisType::RightStickY);

        // Rotate
        if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
            rotate_cam_around_middle(
                &mut q_cam,
                Vec2::new(x * 20., y * 20.),
                time.delta_seconds(),
            );
        }

        let axis_lx = GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX);
        let axis_ly = GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY);

        if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            cam_walk(
                &mut q_cam,
                Vec2::new(x * 100., y * 100.),
                time.delta_seconds(),
            );
        }
    }
}
