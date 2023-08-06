use super::set_flipper_status;
use crate::ball::{spawn_ball, BallSpawn};
use crate::ball_starter::BallStarterState;
use crate::flipper::{FlipperStatus, FlipperType};
use crate::prelude::*;
use crate::CameraState;
use bevy::window::{CursorGrabMode, PrimaryWindow};

#[allow(clippy::too_many_arguments)]
pub(super) fn key_system(
    mut cmds: Commands,
    key: Res<Input<KeyCode>>,
    ball_spawn: Res<BallSpawn>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    mut cam_state: ResMut<NextState<CameraState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ball_starter_state: ResMut<NextState<BallStarterState>>,
    mut q_flipper: Query<(&mut FlipperStatus, &FlipperType)>,
) {
    if key.just_pressed(KeyCode::Escape) {
        let mut window = q_window.get_single_mut().unwrap();
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        cam_state.set(CameraState::None);
    }

    if key.just_pressed(KeyCode::ControlLeft) {
        spawn_ball(&mut cmds, &mut meshes, &mut materials, ball_spawn.0);
    }

    if key.just_pressed(KeyCode::Space) {
        ball_starter_state.set(BallStarterState::Charge);
    }

    if key.just_released(KeyCode::Space) {
        ball_starter_state.set(BallStarterState::Fire);
    }

    if key.just_pressed(KeyCode::Y) {
        set_flipper_status(FlipperType::Left, FlipperStatus::Pushed, &mut q_flipper);
    }
    if key.just_pressed(KeyCode::C) {
        set_flipper_status(FlipperType::Right, FlipperStatus::Pushed, &mut q_flipper);
    }
    if key.just_released(KeyCode::Y) {
        set_flipper_status(FlipperType::Left, FlipperStatus::Idle, &mut q_flipper);
    }
    if key.just_released(KeyCode::C) {
        set_flipper_status(FlipperType::Right, FlipperStatus::Idle, &mut q_flipper);
    }
}

pub(super) fn mouse_btn_system(
    btn: Res<Input<MouseButton>>,
    mut cam_state: ResMut<NextState<CameraState>>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if btn.just_pressed(MouseButton::Right) {
        let mut window = q_window.get_single_mut().unwrap();
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        cam_state.set(CameraState::FpsCamera);
    }
}
