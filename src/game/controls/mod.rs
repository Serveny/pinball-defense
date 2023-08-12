use self::gamepad::{gamepad_connections, gamepad_controls};
use self::mouse_keyboard::{key_system, mouse_btn_system};
use crate::game::flipper::{FlipperStatus, FlipperType};
use crate::game::GameState;
use crate::prelude::*;

pub mod gamepad;
mod mouse_keyboard;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (key_system, mouse_btn_system, gamepad_controls).run_if(in_state(GameState::Ingame)),
        )
        .add_systems(Update, gamepad_connections);
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
