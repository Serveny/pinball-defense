use crate::game::ball::PinBall;
use crate::prelude::*;

//fn place_cam(mut q_cam: Query<&mut Transform, (With<Camera>, Without<Ball>)>) {
//if let Ok(mut cam) = q_cam.single_mut() {
//cam.translation = Vec3::new(2.40, 0.28, 1.20);
//cam.look_at(Vec3::ZERO, Vec3::Z);
//}
//}

pub(super) fn follow_ball(
    mut q_cam: Query<&mut Transform, (With<Camera>, Without<PinBall>)>,
    q_ball: Query<&Transform, With<PinBall>>,
) {
    for ball_trans in q_ball.iter() {
        if let Ok(mut cam) = q_cam.single_mut() {
            cam.look_at(ball_trans.translation, Vec3::Z);
        }
    }
}
