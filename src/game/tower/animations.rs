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
    mut q_rtt: Query<(&mut Transform, &Parent), With<RotateToTarget>>,
    q_parent: Query<(&Transform, &TargetPos), Without<RotateToTarget>>,
) {
    for (mut rot_trans, parent) in q_rtt.iter_mut() {
        if let Ok((tower_trans, target_pos)) = q_parent.get(parent.get()) {
            let tower_pos = tower_trans.translation;
            if let Some(target_pos) = target_pos.0 {
                let direction = target_pos - tower_pos;
                rot_trans.look_at(direction, Vec3::Y);
                rot_trans.rotate_y(f32::to_radians(90.));
            }
        }
    }
}
