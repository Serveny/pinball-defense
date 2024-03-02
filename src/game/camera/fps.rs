use crate::game::controls::gamepad::MyGamepad;
use crate::prelude::*;
use bevy::input::mouse::MouseMotion;

#[derive(Component)]
pub(super) struct LookDirection {
    yaw: f32,
    pitch: f32,
}

impl Default for LookDirection {
    fn default() -> Self {
        LookDirection {
            yaw: -90.0,
            pitch: 0.0,
        }
    }
}

#[derive(Resource)]
pub(super) struct FpsCamSettings {
    pub move_speed: f32,
    pub mouse_sensitivity: f32,
    pub stick_sensitivity: f32,
}

impl Default for FpsCamSettings {
    fn default() -> Self {
        FpsCamSettings {
            move_speed: 1.,
            mouse_sensitivity: 0.1,
            stick_sensitivity: 1.,
        }
    }
}

pub(super) fn on_keyboard_mouse_motion_system(
    mut evr: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut LookDirection)>,
    key_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    settings: Res<FpsCamSettings>,
) {
    // Handle mouse movement to rotate the camera
    let mut delta_look = Vec2::default();
    evr.read()
        .for_each(|ev| delta_look += Vec2::new(ev.delta.x, ev.delta.y));

    // Handle keyboard input to move the camera
    let delta_move = Vec2::new(
        (key_input.pressed(KeyCode::KeyW) as i8 - key_input.pressed(KeyCode::KeyS) as i8) as f32,
        (key_input.pressed(KeyCode::KeyA) as i8 - key_input.pressed(KeyCode::KeyD) as i8) as f32,
    );

    query.iter_mut().for_each(|(mut transform, mut cam)| {
        look_and_move_in_direction(
            &mut transform,
            &mut cam,
            delta_look,
            delta_move,
            time.delta_seconds(),
            settings.mouse_sensitivity,
            settings.move_speed,
        )
    });
}

pub(super) fn gamepad_input(
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut query: Query<(&mut Transform, &mut LookDirection)>,
    time: Res<Time>,
    settings: Res<FpsCamSettings>,
) {
    if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        let gamepad = gp.0;

        // Rotate
        if let (Some(lx), Some(ly), Some(rx), Some(ry)) = (
            axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX)),
            axes.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY)),
            axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX)),
            axes.get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY)),
        ) {
            query.iter_mut().for_each(|(mut transform, mut cam)| {
                look_and_move_in_direction(
                    &mut transform,
                    &mut cam,
                    Vec2::new(rx, -ry),
                    Vec2::new(ly, -lx),
                    time.delta_seconds(),
                    settings.stick_sensitivity,
                    settings.move_speed,
                )
            });
        }
    }
}

fn look_and_move_in_direction(
    transform: &mut Transform,
    direction: &mut LookDirection,
    delta_look: Vec2,
    delta_move: Vec2,
    delta_seconds: f32,
    sensitivity: f32,
    move_speed: f32,
) {
    // Handle mouse movement to rotate the camera
    direction.yaw -= delta_look.x * sensitivity;
    direction.pitch += (delta_look.y * sensitivity).clamp(-89., 89.);

    // Rotate the camera using yaw and pitch
    transform.rotation = Quat::from_axis_angle(Vec3::Z, direction.yaw.to_radians())
        * Quat::from_axis_angle(-Vec3::X, direction.pitch.to_radians());

    // Handle keyboard input to move the camera
    if delta_move.x != 0. || delta_move.y != 0. {
        let direction =
            transform.rotation * Vec3::new(-delta_move.y, 0., -delta_move.x).normalize();
        transform.translation += direction * move_speed * delta_seconds;
    }
}
