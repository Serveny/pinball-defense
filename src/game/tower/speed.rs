use super::TowerReady;
use super::target::Targets;
use crate::game::enemy::Enemy;
use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(super) struct SlowDownFactor(pub f32);

pub(super) fn slow_down_system(
    mut q_enemy: Query<&mut Enemy>,
    q_tower: Query<(&Targets, &SlowDownFactor), With<TowerReady>>,
) {
    for (targets, slow_factor) in q_tower.iter() {
        for enemy_id in &targets.0 {
            if let Ok(mut enemy) = q_enemy.get_mut(*enemy_id) {
                enemy.slow_down(slow_factor.0);
            }
        }
    }
}
