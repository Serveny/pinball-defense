use super::set_flipper_status;
use crate::game::ball_starter::SpawnBallEvent;
use crate::game::{
    ball_starter::BallStarterState,
    flipper::{FlipperStatus, FlipperType},
};
use crate::prelude::*;
use bevy::input::gamepad::{GamepadButtonChangedEvent, GamepadConnectionEvent};

/// Simple resource to store the ID of the connected gamepad.
/// We need to know which gamepad to use for player input.
#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);

pub(super) fn gamepad_connections(
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

pub(super) fn gamepad_controls(
    mut evr: EventReader<GamepadButtonChangedEvent>,
    mut spawn_ball_ev: EventWriter<SpawnBallEvent>,
    mut ball_starter_state: ResMut<NextState<BallStarterState>>,
    mut q_flipper: Query<(&mut FlipperStatus, &FlipperType)>,
) {
    for ev in evr.iter() {
        match ev.button_type {
            GamepadButtonType::East if ev.value > 0. => {
                spawn_ball_ev.send(SpawnBallEvent);
            }

            GamepadButtonType::South => ball_starter_state.set(match ev.value == 0. {
                true => BallStarterState::Fire,
                false => BallStarterState::Charge,
            }),

            GamepadButtonType::LeftTrigger => set_flipper_status(
                FlipperType::Left,
                FlipperStatus::by_value(ev.value),
                &mut q_flipper,
            ),

            GamepadButtonType::RightTrigger => set_flipper_status(
                FlipperType::Right,
                FlipperStatus::by_value(ev.value),
                &mut q_flipper,
            ),
            // other events are irrelevant
            _ => {}
        }
    }
}
