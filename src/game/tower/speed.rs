use super::target::AimFirstEnemy;
use crate::game::enemy::Enemy;
use crate::prelude::*;

#[derive(Component)]
pub(super) struct SlowDownFactor(pub f32);

pub(super) fn afe_slow_down_system(
    mut q_enemy: Query<&mut Enemy>,
    q_tower: Query<(&AimFirstEnemy, &SlowDownFactor)>,
) {
    for (target, slow_factor) in q_tower.iter() {
        if let Some(enemy_id) = target.0 {
            if let Ok(mut enemy) = q_enemy.get_mut(enemy_id) {
                enemy.slow_down(slow_factor.0);
            }
        }
    }
}
