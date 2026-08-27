use super::set_flipper_status;
use crate::game::ball_starter::SpawnBallEvent;
use crate::game::{
    ball_starter::BallStarterState,
    flipper::{FlipperStatus, FlipperType},
};
use crate::game::{GameState, PauseGameEvent, ResumeGameEvent};
use crate::menu::MenuState;
use crate::prelude::*;
use bevy::input::gamepad::GamepadButtonChangedEvent;
use bevy::input::ButtonState;

pub(super) fn on_btn_changed(
    mut evr: MessageReader<GamepadButtonChangedEvent>,
    mut spawn_ball_ev: MessageWriter<SpawnBallEvent>,
    mut ball_starter_state: ResMut<NextState<BallStarterState>>,
    mut q_flipper: Query<(&mut FlipperStatus, &FlipperType)>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut pause_ev: MessageWriter<PauseGameEvent>,
    mut resume_ev: MessageWriter<ResumeGameEvent>,
    game_state: Res<State<GameState>>,
) {
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
                ball_starter_state.set(match ev.state {
                    ButtonState::Pressed => BallStarterState::Charge,
                    ButtonState::Released => BallStarterState::Fire,
                });
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
            GamepadButton::Start => match *game_state.get() {
                GameState::Ingame => {
                    pause_ev.write(PauseGameEvent);
                    menu_state.set(MenuState::PauseMenu);
                }
                GameState::Pause => {
                    menu_state.set(MenuState::None);
                    resume_ev.write(ResumeGameEvent);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
