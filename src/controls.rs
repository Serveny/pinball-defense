use crate::ball::{spawn_ball, BallSpawn};
use crate::ball_starter::BallStarterState;
use crate::fps_camera::CameraState;
use crate::prelude::*;
use crate::GameState;
use bevy::input::gamepad::{GamepadButtonChangedEvent, GamepadConnectionEvent};
use bevy::window::{CursorGrabMode, PrimaryWindow};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (cursor_grab_system, gamepad_connections, gamepad_controls)
                .in_set(OnUpdate(GameState::Ingame)),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn cursor_grab_system(
    mut cmds: Commands,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    ball_spawn: Res<BallSpawn>,
    mut cam_state: ResMut<NextState<CameraState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ball_starter_state: ResMut<NextState<BallStarterState>>,
) {
    let mut window = q_window.get_single_mut().unwrap();
    if btn.just_pressed(MouseButton::Right) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        cam_state.set(CameraState::Active);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        cam_state.set(CameraState::Inactive);
    }

    if key.just_pressed(KeyCode::LControl) {
        spawn_ball(&mut cmds, &mut meshes, &mut materials, ball_spawn.0);
    }

    if key.just_pressed(KeyCode::Space) {
        ball_starter_state.set(BallStarterState::Charge);
    }

    if key.just_released(KeyCode::Space) {
        ball_starter_state.set(BallStarterState::Fire);
    }
}

/// Simple resource to store the ID of the connected gamepad.
/// We need to know which gamepad to use for player input.
#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);

fn gamepad_connections(
    mut cmds: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadConnectionEvent>,
) {
    for ev in gamepad_evr.iter() {
        let id = ev.gamepad;
        match ev.connected() {
            true => {
                println!("New gamepad connected with ID: {id:?}");

                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    cmds.insert_resource(MyGamepad(id));
                }
            }
            false => {
                println!("Lost gamepad connection with ID: {id:?}");

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == id {
                        cmds.remove_resource::<MyGamepad>();
                    }
                }
            }
        }
    }
}

fn gamepad_controls(
    mut cmds: Commands,
    mut evr: EventReader<GamepadButtonChangedEvent>,
    ball_spawn: Res<BallSpawn>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ball_starter_state: ResMut<NextState<BallStarterState>>,
) {
    for ev in evr.iter() {
        match ev.button_type {
            GamepadButtonType::East if ev.value > 0. => {
                spawn_ball(&mut cmds, &mut meshes, &mut materials, ball_spawn.0)
            }

            GamepadButtonType::South => ball_starter_state.set(match ev.value == 0. {
                true => BallStarterState::Charge,
                false => BallStarterState::Fire,
            }),

            // other events are irrelevant
            _ => {}
        }
    }
}
