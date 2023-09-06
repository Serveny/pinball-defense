use self::gamepad::{on_gamepad_btn_changed, on_gamepad_connections};
use self::mouse_keyboard::{key_system, mouse_btn_system, pause_key_system};
use crate::game::flipper::{FlipperStatus, FlipperType};
use crate::game::GameState;
use crate::prelude::*;

pub mod gamepad;
mod mouse_keyboard;

#[derive(Resource, Reflect)]
pub struct KeyboardControls {
    flipper_left: KeyCode,
    flipper_right: KeyCode,
    charge_ball_starter: KeyCode,
    pause: KeyCode,
}

impl Default for KeyboardControls {
    fn default() -> Self {
        Self {
            flipper_left: KeyCode::Y,
            flipper_right: KeyCode::C,
            charge_ball_starter: KeyCode::Space,
            pause: KeyCode::P,
        }
    }
}

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyboardControls>()
            .add_systems(
                Update,
                (key_system, mouse_btn_system, on_gamepad_btn_changed)
                    .run_if(in_state(GameState::Ingame)),
            )
            .add_systems(Update, on_gamepad_connections)
            .add_systems(Update, pause_key_system.run_if(in_state(GameState::Pause)));
    }
}

fn set_flipper_status(
    flipper_type: FlipperType,
    status: FlipperStatus,
    q_flipper: &mut Query<(&mut FlipperStatus, &FlipperType)>,
) {
    for (mut f_status, f_type) in q_flipper.iter_mut() {
        if *f_type == flipper_type {
            *f_status = status;
            return;
        }
    }
}
