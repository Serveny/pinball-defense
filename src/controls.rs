use crate::prelude::*;
use bevy::input::mouse::MouseMotion;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_controls)
            .add_system(mouse_input)
            .add_system(gamepad_connections)
            .add_system(gamepad_input);
    }
}

fn setup_controls(cmds: Commands) {}

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
    transform.rotate_around(
        Vec3::ZERO,
        Quat::from_euler(
            EulerRot::XYZ,
            (4. * delta.y * delta_seconds).to_radians(),
            (4. * delta.x * delta_seconds).to_radians(),
            (-4. * delta.y * delta_seconds).to_radians(),
        ),
    );
}

/// Simple resource to store the ID of the connected gamepad.
/// We need to know which gamepad to use for player input.
#[derive(Resource)]
struct MyGamepad(Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
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
                    commands.insert_resource(MyGamepad(id));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == id {
                        commands.remove_resource::<MyGamepad>();
                    }
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
    let gamepad = if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        gp.0
    } else {
        // no gamepad is connected
        return;
    };

    // The joysticks are represented using a separate axis for X and Y
    let axis_lx = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::RightStickX,
    };
    let axis_ly = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::RightStickY,
    };

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        rotate_cam_around_middle(
            &mut q_cam,
            Vec2::new(x * 20., y * 20.),
            time.delta_seconds(),
        );
    }
}
