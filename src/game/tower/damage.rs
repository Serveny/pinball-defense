use super::target::{AimFirstEnemy, EnemiesWithinReach};
use crate::game::health::ChangeHealthEvent;
use crate::prelude::*;

pub(super) type DamagePerSecond = f32;

#[derive(Component)]
pub(super) struct DamageOverTime(pub DamagePerSecond);

pub(super) fn afe_damage_over_time_system(
    time: Res<Time>,
    q_tower: Query<(Entity, &AimFirstEnemy, &DamageOverTime)>,
    mut health_ev: EventWriter<ChangeHealthEvent>,
) {
    for (tower_id, target, damage) in q_tower.iter() {
        if let Some(enemy_id) = target.0 {
            health_ev.send(ChangeHealthEvent::new(
                enemy_id,
                -damage.0 * time.delta_seconds(),
                Some(tower_id),
            ));
        }
    }
}

#[derive(Component)]
pub struct DamageAllTargetsInReach;

pub(super) fn datir_damage_over_time_system(
    time: Res<Time>,
    q_tower: Query<(Entity, &EnemiesWithinReach, &DamageOverTime), With<DamageAllTargetsInReach>>,
    mut health_ev: EventWriter<ChangeHealthEvent>,
) {
    for (tower_id, targets, damage) in q_tower.iter() {
        for enemy_id in targets.0.iter() {
            health_ev.send(ChangeHealthEvent::new(
                *enemy_id,
                -damage.0 * time.delta_seconds(),
                Some(tower_id),
            ));
        }
    }
}
