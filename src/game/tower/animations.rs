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
        trans.rotate(Quat::from_rotation_z(time.delta_secs()));
    }
}

#[derive(Component)]
pub(super) struct RotateToTarget;

pub(super) fn rotate_to_target_system(
    mut q_rtt: Query<(&mut Transform, &RelEntity), With<RotateToTarget>>,
    q_spawner: Query<(&Tower, &TargetPos)>,
) {
    for (mut rot_trans, rel_id) in q_rtt.iter_mut() {
        if let Ok((tower, target_pos)) = q_spawner.get(rel_id.0) {
            if let Some(target_pos) = target_pos.0 {
                let direction = target_pos.truncate() - tower.pos.truncate();
                let angle = direction.angle_to(Vec2::Y);
                rot_trans.rotation = Quat::from_rotation_z(-angle);
            }
        }
    }
}
