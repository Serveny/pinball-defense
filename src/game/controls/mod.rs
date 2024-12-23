use self::mouse_keyboard::{key_system, mouse_btn_system, pause_key_system};
use crate::game::flipper::{FlipperStatus, FlipperType};
use crate::game::GameState;
use crate::prelude::*;

pub mod gamepad;
mod mouse_keyboard;

#[derive(Resource, Reflect)]
pub struct KeyboardControls {
    pub menu: KeyCode,
    pub flipper_left: KeyCode,
    pub flipper_right: KeyCode,
    pub charge_ball_starter: KeyCode,
    pub pause: KeyCode,
    pub toggle_key_ui: KeyCode,
}

impl Default for KeyboardControls {
    fn default() -> Self {
        Self {
            menu: KeyCode::Escape,
            flipper_left: KeyCode::KeyA,
            flipper_right: KeyCode::KeyD,
            charge_ball_starter: KeyCode::Space,
            pause: KeyCode::KeyP,
            toggle_key_ui: KeyCode::KeyK,
        }
    }
}

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyboardControls>()
            .add_systems(
                Update,
                (key_system, mouse_btn_system, gamepad::on_btn_changed)
                    .run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (pause_key_system, gamepad::pause_btn_changed).run_if(in_state(GameState::Pause)),
            );
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
