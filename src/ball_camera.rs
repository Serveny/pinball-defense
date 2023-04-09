use crate::ball::Ball;
use crate::prelude::*;
use crate::CameraState;
pub struct BallCameraPlugin;

impl Plugin for BallCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(follow_ball.in_set(OnUpdate(CameraState::BallCamera)));
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
