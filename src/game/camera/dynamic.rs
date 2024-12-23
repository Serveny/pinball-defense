use super::{CamTransformLens, PinballCamera, CAM_LOW_POS, LOOK_POS};
use crate::game::ball_starter::{
    BallSpawn, BallStarterChargeStartedEvent, BallStarterFireEndEvent,
};
use crate::prelude::*;
use bevy_tweening::{Animator, EaseMethod, Tween};

pub(super) fn on_ball_start_cam_system(
    mut cmds: Commands,
    q_cam: Query<Entity, With<PinballCamera>>,
    on_charge_start: EventReader<BallStarterChargeStartedEvent>,
    on_fire_end: EventReader<BallStarterFireEndEvent>,
    ball_spawn: Res<BallSpawn>,
) {
    if !on_charge_start.is_empty() {
        cmds.entity(q_cam.single())
            .insert(Animator::new(ball_start_tracking_shot(false, ball_spawn.0)));
    } else if !on_fire_end.is_empty() {
        cmds.entity(q_cam.single())
            .insert(Animator::new(ball_start_tracking_shot(true, ball_spawn.0)));
    }
}

const CAM_BALL_START_POS: Vec3 = Vec3::new(1.7, 0.9, 1.4);

fn ball_start_tracking_shot(is_back: bool, look_at: Vec3) -> Tween<Transform> {
    let mut start = CAM_LOW_POS;
    let mut end = CAM_BALL_START_POS;
    let mut look_at_start = LOOK_POS;
    let mut look_at_end = look_at;
    if is_back {
        std::mem::swap(&mut start, &mut end);
        std::mem::swap(&mut look_at_start, &mut look_at_end);
    }
    Tween::new(
        EaseMethod::EaseFunction(EaseFunction::CubicOut),
        std::time::Duration::from_secs(1),
        CamTransformLens::new(start, end, look_at_start, look_at_end),
    )
}
