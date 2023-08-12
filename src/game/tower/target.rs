use super::base::TowerSightSensor;
use crate::game::enemy::Enemy;
use crate::prelude::*;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

#[derive(Component)]
pub(super) struct TargetPos(pub Option<Vec3>);

#[derive(Component)]
pub(super) struct SightRadius(pub f32);

#[derive(Component)]
pub(super) struct AimFirstEnemy(pub Option<Entity>);

pub(super) fn aim_first_enemy_system(
    mut q_afe: Query<(&mut TargetPos, &AimFirstEnemy)>,
    q_enemy: Query<&Transform, With<Enemy>>,
) {
    for (mut target_pos, aim_enemy) in q_afe.iter_mut() {
        target_pos.0 = aim_enemy
            .0
            .and_then(|enemy_id| q_enemy.get(enemy_id).ok().map(|item| item.translation));
    }
}

pub(super) fn enemy_sight_system(
    mut col_events: EventReader<CollisionEvent>,
    mut q_afe: Query<&mut AimFirstEnemy>,
    q_tower_sight: Query<&Parent, With<TowerSightSensor>>,
) {
    for ev in col_events.iter() {
        match ev {
            CollisionEvent::Started(id_1, id_2, flag) => {
                if *flag == CollisionEventFlags::SENSOR {
                    set_aim_target(*id_1, *id_2, &mut q_afe, &q_tower_sight);
                }
            }
            CollisionEvent::Stopped(id_1, id_2, flag) => {
                if *flag == CollisionEventFlags::SENSOR {
                    unset_aim_target(*id_1, *id_2, &mut q_afe, &q_tower_sight);
                }
            }
        }
    }
}

fn set_aim_target(
    id_1: Entity,
    id_2: Entity,
    q_afe: &mut Query<&mut AimFirstEnemy>,
    q_tower_sight: &Query<&Parent, With<TowerSightSensor>>,
) {
    for (id_1, id_2) in [(id_1, id_2), (id_2, id_1)] {
        if let Ok(ts_parent) = q_tower_sight.get(id_1) {
            if let Ok(mut afe) = q_afe.get_mut(ts_parent.get()) {
                if afe.0.is_none() {
                    log!("😠 Set aim target {:?} to {:?}", id_1, id_2);
                    afe.0 = Some(id_2);
                }
                return;
            }
        }
    }
}

fn unset_aim_target(
    id_1: Entity,
    id_2: Entity,
    q_afe: &mut Query<&mut AimFirstEnemy>,
    q_tower_sight: &Query<&Parent, With<TowerSightSensor>>,
) {
    for id in [id_1, id_2] {
        if let Ok(ts_parent) = q_tower_sight.get(id) {
            if let Ok(mut afe) = q_afe.get_mut(ts_parent.get()) {
                if afe.0.is_some() {
                    afe.0 = None;
                    log!("😊 Unset aim target {:?}", afe.0);
                    return;
                }
            }
        }
    }
}
