use super::set_flipper_status;
use crate::game::ball_starter::SpawnBallEvent;
use crate::game::{
    ball_starter::BallStarterState,
    flipper::{FlipperStatus, FlipperType},
};
use crate::game::{PauseGameEvent, ResumeGameEvent};
use crate::menu::MenuState;
use crate::prelude::*;
use bevy::input::gamepad::GamepadButtonChangedEvent;
use bevy::input::ButtonState;

pub(super) fn on_btn_changed(
    mut evr: EventReader<GamepadButtonChangedEvent>,
    mut spawn_ball_ev: EventWriter<SpawnBallEvent>,
    mut ball_starter_state: ResMut<NextState<BallStarterState>>,
    mut q_flipper: Query<(&mut FlipperStatus, &FlipperType)>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut pause_ev: EventWriter<PauseGameEvent>,
) {
    for ev in evr.read() {
        if ev.state != ButtonState::Pressed {
            return;
        }
        match ev.button {
            GamepadButton::East if ev.value > 0. => {
                spawn_ball_ev.write(SpawnBallEvent);
            }
            GamepadButton::South => ball_starter_state.set(match ev.value == 0. {
                true => BallStarterState::Fire,
                false => BallStarterState::Charge,
            }),
            GamepadButton::LeftTrigger => set_flipper_status(
                FlipperType::Left,
                FlipperStatus::by_value(ev.value),
                &mut q_flipper,
            ),
            GamepadButton::RightTrigger => set_flipper_status(
                FlipperType::Right,
                FlipperStatus::by_value(ev.value),
                &mut q_flipper,
            ),
            GamepadButton::Start => {
                pause_ev.write(PauseGameEvent);
                menu_state.set(MenuState::PauseMenu);
            }
            _ => {}
        }
    }
}

pub(super) fn pause_btn_changed(
    mut evr: EventReader<GamepadButtonChangedEvent>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut resume_ev: EventWriter<ResumeGameEvent>,
) {
    for ev in evr.read() {
        match ev.button {
            GamepadButton::Start => {
                menu_state.set(MenuState::None);
                resume_ev.write(ResumeGameEvent);
            }
            _ => {}
        }
    }
}
