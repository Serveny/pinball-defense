use super::GameState;
use crate::prelude::*;

mod ball;
mod fps;

pub struct PinballCameraPlugin;

impl Plugin for PinballCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CameraState>()
            .add_systems(OnEnter(GameState::Init), fps::setup_camera)
            .add_systems(
                Update,
                ball::follow_ball.run_if(in_state(CameraState::BallCamera)),
            )
            .add_systems(
                Update,
                (fps::on_keyboard_mouse_motion_system, fps::gamepad_input)
                    .run_if(in_state(CameraState::FpsCamera)),
            );
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum CameraState {
    #[default]
    None,
    BallCamera,
    FpsCamera,
}
