use super::target::TargetPos;
use crate::prelude::*;

#[derive(Component)]
pub(super) struct RotateAlways;

pub(super) fn rotate_always_system(
    mut q_rha: Query<&mut Transform, With<RotateAlways>>,
    time: Res<Time>,
) {
    for mut trans in q_rha.iter_mut() {
        trans.rotate(Quat::from_rotation_y(time.delta_seconds()));
    }
}

#[derive(Component)]
pub(super) struct RotateToTarget;

pub(super) fn rotate_to_target_system(
    mut q_rtt: Query<(&mut Transform, &TargetPos), With<RotateToTarget>>,
) {
    for (mut trans, pos) in q_rtt.iter_mut() {
        if let Some(target_pos) = pos.0 {
            trans.look_at(target_pos, Vec3::Y);
            log!("Set target pos to {:?}", trans);
        }
    }
}
