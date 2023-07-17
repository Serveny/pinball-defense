use crate::ball::Ball;
use crate::prelude::*;
use crate::CameraState;
pub struct BallCameraPlugin;

impl Plugin for BallCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            follow_ball.run_if(in_state(CameraState::BallCamera)),
        );
    }
}

fn place_cam(mut q_cam: Query<&mut Transform, (With<Camera>, Without<Ball>)>) {
    if let Ok(mut cam) = q_cam.get_single_mut() {
        cam.translation = Vec3::new(2.40, 1.20, -0.28);
        cam.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn follow_ball(
    mut q_cam: Query<&mut Transform, (With<Camera>, Without<Ball>)>,
    q_ball: Query<&Transform, With<Ball>>,
) {
    for ball_trans in q_ball.iter() {
        if let Ok(mut cam) = q_cam.get_single_mut() {
            cam.look_at(ball_trans.translation, Vec3::Y);
        }
    }
}
