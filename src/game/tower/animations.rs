use super::target::TargetPos;
use super::Tower;
use crate::prelude::*;
use crate::utils::RelEntity;

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
    mut q_rtt: Query<(&mut Transform, &RelEntity), With<RotateToTarget>>,
    q_parent: Query<(&Tower, &TargetPos)>,
) {
    for (mut rot_trans, rel_id) in q_rtt.iter_mut() {
        if let Ok((tower, target_pos)) = q_parent.get(rel_id.0) {
            if let Some(target_pos) = target_pos.0 {
                let direction = target_pos - tower.pos;
                rot_trans.look_at(direction, Vec3::Y);
            }
        }
    }
}
