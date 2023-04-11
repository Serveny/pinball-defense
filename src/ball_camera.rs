use crate::ball::Ball;
use crate::prelude::*;
use crate::CameraState;
pub struct BallCameraPlugin;

impl Plugin for BallCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(place_cam.in_schedule(OnEnter(CameraState::BallCamera)))
            .add_system(follow_ball.in_set(OnUpdate(CameraState::BallCamera)));
    }
}

fn place_cam(mut q_cam: Query<&mut Transform, (With<Camera>, Without<Ball>)>) {
    if let Ok(mut cam) = q_cam.get_single_mut() {
        cam.translation = Vec3::new(240., 120., -28.);
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
