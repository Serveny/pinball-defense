use super::TowerSightSensor;
use crate::game::enemy::{Enemy, OnEnemyDespawnEvent};
use crate::prelude::*;
use bevy::platform::collections::HashSet;

#[derive(Component)]
pub(super) struct SightRadius(pub f32);

#[derive(Component)]
pub(super) struct TargetPos(pub Option<Vec3>);

pub(super) fn target_pos_by_afe_system(
    mut q_afe: Query<(&mut TargetPos, &AimFirstEnemy)>,
    q_enemy: Query<&Transform, With<Enemy>>,
) {
    for (mut target_pos, afe) in q_afe.iter_mut() {
        target_pos.0 = afe
            .0
            .and_then(|enemy_id| q_enemy.get(enemy_id).ok().map(|item| item.translation));
    }
}

#[derive(Component)]
pub(super) struct AimFirstEnemy(pub Option<Entity>);

pub(super) fn aim_first_enemy_system(mut q_afe: Query<(&mut AimFirstEnemy, &EnemiesWithinReach)>) {
    for (mut aim_enemy, ewr) in q_afe.iter_mut() {
        match aim_enemy.0 {
            Some(enemy_id) => {
                if !ewr.0.contains(&enemy_id) {
                    aim_enemy.0 = None;
                }
            }
            None => aim_enemy.0 = ewr.0.iter().next().copied(),
        }
    }
}

#[derive(Component, Default)]
pub(super) struct EnemiesWithinReach(pub HashSet<Entity>);

pub(super) fn on_enemy_enter_reach_system(
    mut evr: EventReader<CollisionStarted>,
    mut q_ewr: Query<&mut EnemiesWithinReach>,
    q_tower_sight: Query<&ChildOf, With<TowerSightSensor>>,
) {
    for ev in evr.read() {
        // if *flag == CollisionEventFlags::SENSOR {
        edit_eir(ev.0, ev.1, &mut q_ewr, &q_tower_sight, |eir, enemy_id| {
            //log!("Insert: {enemy_id:?}");
            eir.0.insert(enemy_id);
        });
    }
}

pub(super) fn on_enemy_leave_reach_system(
    mut evr: EventReader<CollisionEnded>,
    mut q_ewr: Query<&mut EnemiesWithinReach>,
    q_tower_sight: Query<&ChildOf, With<TowerSightSensor>>,
) {
    for ev in evr.read() {
        // if *flag == CollisionEventFlags::SENSOR {
        edit_eir(ev.0, ev.1, &mut q_ewr, &q_tower_sight, |eir, enemy_id| {
            //log!("Remove: {enemy_id:?}");
            eir.0.remove(&enemy_id);
        });
    }
}

fn edit_eir<F: FnOnce(&mut EnemiesWithinReach, Entity)>(
    id_1: Entity,
    id_2: Entity,
    q_eir: &mut Query<&mut EnemiesWithinReach>,
    q_tower_sight: &Query<&ChildOf, With<TowerSightSensor>>,
    f: F,
) {
    for (tower_sight_id, enemy_id) in [(id_1, id_2), (id_2, id_1)] {
        if let Ok(ts_child_of) = q_tower_sight.get(tower_sight_id) {
            if let Ok(mut eir) = q_eir.get_mut(ts_child_of.parent()) {
                f(&mut eir, enemy_id);
                return;
            }
            panic!("😭 No 'EnemiesWithinReach' component for tower sight found.");
        }
    }
}

pub(super) fn on_remove_despawned_enemies_from_ewr_system(
    mut evr: EventReader<OnEnemyDespawnEvent>,
    mut q_ewr: Query<&mut EnemiesWithinReach>,
) {
    for ev in evr.read() {
        for mut ewr in q_ewr.iter_mut() {
            ewr.0.remove(&ev.0);
        }
    }
}
