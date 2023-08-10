use super::set_flipper_status;
use crate::ball_starter::{BallStarterState, SpawnBallEvent};
use crate::flipper::{FlipperStatus, FlipperType};
use crate::prelude::*;
use crate::CameraState;
use bevy::window::{CursorGrabMode, PrimaryWindow};

#[allow(clippy::too_many_arguments)]
pub(super) fn key_system(
    key: Res<Input<KeyCode>>,
    mut spawn_ball_ev: EventWriter<SpawnBallEvent>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    mut cam_state: ResMut<NextState<CameraState>>,
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
        spawn_ball_ev.send(SpawnBallEvent);
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
