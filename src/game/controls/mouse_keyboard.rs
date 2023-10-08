use super::{set_flipper_status, KeyboardControls};
use crate::game::ball_starter::{BallStarterState, SpawnBallEvent};
use crate::game::camera::CameraState;
use crate::game::flipper::{FlipperStatus, FlipperType};
use crate::game::{GameState, PauseGameEvent, ResumeGameEvent};
use crate::menu::MenuState;
use crate::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

#[allow(clippy::too_many_arguments)]
pub(super) fn key_system(
    key: Res<Input<KeyCode>>,
    controls: Res<KeyboardControls>,
    mut spawn_ball_ev: EventWriter<SpawnBallEvent>,
    mut pause_ev: EventWriter<PauseGameEvent>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    mut cam_state: ResMut<NextState<CameraState>>,
    mut ball_starter_state: ResMut<NextState<BallStarterState>>,
    mut q_flipper: Query<(&mut FlipperStatus, &FlipperType)>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if key.just_pressed(KeyCode::U) {
        game_state.set(GameState::GameOver);
    }

    if key.just_pressed(KeyCode::Escape) {
        let mut window = q_window.get_single_mut().unwrap();
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        cam_state.set(CameraState::Dynamic);
        pause_ev.send(PauseGameEvent);
        menu_state.set(MenuState::PauseMenu);
    }

    if key.just_pressed(KeyCode::ControlLeft) {
        spawn_ball_ev.send(SpawnBallEvent);
    }

    if key.just_pressed(controls.charge_ball_starter) {
        ball_starter_state.set(BallStarterState::Charge);
    }

    if key.just_released(controls.charge_ball_starter) {
        ball_starter_state.set(BallStarterState::Fire);
    }

    if key.just_pressed(controls.flipper_left) {
        set_flipper_status(FlipperType::Left, FlipperStatus::Pushed, &mut q_flipper);
    }
    if key.just_pressed(controls.flipper_right) {
        set_flipper_status(FlipperType::Right, FlipperStatus::Pushed, &mut q_flipper);
    }
    if key.just_released(controls.flipper_left) {
        set_flipper_status(FlipperType::Left, FlipperStatus::Idle, &mut q_flipper);
    }
    if key.just_released(controls.flipper_right) {
        set_flipper_status(FlipperType::Right, FlipperStatus::Idle, &mut q_flipper);
    }
    if key.just_pressed(controls.pause) {
        pause_ev.send(PauseGameEvent);
    }
}

pub(super) fn pause_key_system(
    key: Res<Input<KeyCode>>,
    mut q_flipper: Query<(&mut FlipperStatus, &FlipperType)>,
    mut ball_starter_state: ResMut<NextState<BallStarterState>>,
    mut resume_ev: EventWriter<ResumeGameEvent>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    if key.just_released(KeyCode::Y) {
        set_flipper_status(FlipperType::Left, FlipperStatus::Idle, &mut q_flipper);
    }
    if key.just_released(KeyCode::C) {
        set_flipper_status(FlipperType::Right, FlipperStatus::Idle, &mut q_flipper);
    }
    if key.just_released(KeyCode::Space) {
        ball_starter_state.set(BallStarterState::Fire);
    }
    if key.just_pressed(KeyCode::Escape) || key.just_pressed(KeyCode::P) {
        menu_state.set(MenuState::None);
        resume_ev.send(ResumeGameEvent);
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
