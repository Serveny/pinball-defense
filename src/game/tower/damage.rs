use super::TowerReady;
use super::target::Targets;
use crate::game::health::ChangeHealthEvent;
use crate::prelude::*;

pub(super) type DamagePerSecond = f32;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(super) struct DamageOverTime(pub DamagePerSecond);

pub(super) fn damage_over_time_system(
    time: Res<Time>,
    q_tower: Query<(Entity, &Targets, &DamageOverTime), With<TowerReady>>,
    mut health_ev: MessageWriter<ChangeHealthEvent>,
) {
    for (tower_id, targets, damage) in q_tower.iter() {
        for enemy_id in &targets.0 {
            health_ev.write(ChangeHealthEvent::new(
                *enemy_id,
                -damage.0 * time.delta_secs(),
                Some(tower_id),
            ));
        }
    }
}
