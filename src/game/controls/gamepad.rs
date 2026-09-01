use super::set_flipper_status;
use crate::game::ball_starter::SpawnBallEvent;
use crate::game::{GameState, PauseGameEvent, ResumeGameEvent};
use crate::game::{
    ball_starter::BallStarterState,
    flipper::{FlipperStatus, FlipperType},
};
use crate::menu::MenuState;
use crate::prelude::*;
use bevy::input::ButtonState;
use bevy::input::gamepad::GamepadButtonChangedEvent;

pub(super) fn on_btn_changed(
    mut evr: MessageReader<GamepadButtonChangedEvent>,
    mut spawn_ball_ev: MessageWriter<SpawnBallEvent>,
    mut ball_starter_state: ResMut<NextState<BallStarterState>>,
    mut q_flipper: Query<(&mut FlipperStatus, &FlipperType)>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut pause_ev: MessageWriter<PauseGameEvent>,
    game_state: Res<State<GameState>>,
    ball_starter: Res<State<BallStarterState>>,
) {
    if game_state.is_changed() {
        evr.clear();
    }
    for ev in evr.read() {
        match ev.button {
            GamepadButton::LeftTrigger => {
                set_flipper_status(
                    FlipperType::Left,
                    FlipperStatus::by_value(ev.value),
                    &mut q_flipper,
                );
                continue;
            }
            GamepadButton::RightTrigger => {
                set_flipper_status(
                    FlipperType::Right,
                    FlipperStatus::by_value(ev.value),
                    &mut q_flipper,
                );
                continue;
            }
            GamepadButton::South => {
                if ev.state == ButtonState::Pressed {
                    ball_starter_state.set(BallStarterState::Charge);
                } else if *ball_starter.get() == BallStarterState::Charge {
                    ball_starter_state.set(BallStarterState::Fire);
                }
                continue;
            }
            _ => {}
        }
        if ev.state != ButtonState::Pressed {
            continue;
        }
        match ev.button {
            GamepadButton::East if ev.value > 0. => {
                spawn_ball_ev.write(SpawnBallEvent);
            }
            GamepadButton::Start if *game_state.get() == GameState::Ingame => {
                pause_ev.write(PauseGameEvent);
                menu_state.set(MenuState::PauseMenu);
            }
            _ => {}
        }
    }
}

pub(super) fn pause_btn_changed(
    mut evr: MessageReader<GamepadButtonChangedEvent>,
    mut resume_ev: MessageWriter<ResumeGameEvent>,
    mut menu_state: ResMut<NextState<MenuState>>,
    game_state: Res<State<GameState>>,
) {
    if game_state.is_changed() {
        evr.clear();
    }
    for ev in evr.read() {
        if ev.button == GamepadButton::Start
            && ev.state == ButtonState::Pressed
            && *game_state.get() == GameState::Pause
        {
            menu_state.set(MenuState::None);
            resume_ev.write(ResumeGameEvent);
        }
    }
}
