use super::TowerSightSensor;
use crate::game::enemy::{Enemy, OnEnemyDespawnEvent};
use crate::prelude::*;
use crate::utils::RelEntity;
use bevy::platform::collections::HashSet;

#[derive(Component, Reflect)]
#[reflect(Component)]
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

#[derive(Component, Default)]
pub(super) struct Targets(pub HashSet<Entity>);

pub(super) fn single_target_selection_system(
    mut q_single: Query<(&mut Targets, &AimFirstEnemy), Without<ConeFov>>,
) {
    for (mut targets, afe) in q_single.iter_mut() {
        targets.0.clear();
        if let Some(enemy_id) = afe.0 {
            targets.0.insert(enemy_id);
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TargetAllInReach;

pub(super) fn reach_selection_system(
    mut q_reach: Query<(&mut Targets, &EnemiesWithinReach), With<TargetAllInReach>>,
) {
    for (mut targets, reach) in q_reach.iter_mut() {
        targets.0.clone_from(&reach.0);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ConeFov(pub f32);

#[derive(Component)]
pub struct ConeAim(pub Vec3);

pub(super) fn cone_selection_system(
    mut q_cone: Query<(
        Entity,
        &mut Targets,
        &ConeFov,
        Option<&AimFirstEnemy>,
        &EnemiesWithinReach,
    )>,
    q_enemy: Query<&Transform, With<Enemy>>,
    q_aim_child: Query<(&GlobalTransform, &ConeAim, &RelEntity)>,
) {
    for (tower_id, mut targets, fov, afe, reach) in q_cone.iter_mut() {
        targets.0.clear();
        let active = match afe {
            None => true,
            Some(afe) => afe.0.is_some(),
        };
        if !active {
            continue;
        }
        let Some((aim_global, aim, _)) = q_aim_child
            .iter()
            .find(|(_, _, rel_id)| rel_id.0 == tower_id)
        else {
            continue;
        };
        let (_, rotation, _) = aim_global.to_scale_rotation_translation();
        let beam = rotation * aim.0;
        let apex = aim_global.translation();
        let half_angle = fov.0 / 2.;
        for enemy_id in &reach.0 {
            if let Ok(enemy_tf) = q_enemy.get(*enemy_id)
                && is_in_cone(beam, enemy_tf.translation - apex, half_angle)
            {
                targets.0.insert(*enemy_id);
            }
        }
    }
}

pub(super) fn is_in_cone(beam: Vec3, to_enemy: Vec3, half_angle: f32) -> bool {
    beam != Vec3::ZERO && to_enemy != Vec3::ZERO && beam.angle_between(to_enemy) <= half_angle
}

pub(super) fn on_enemy_enter_reach_system(
    mut evr: MessageReader<CollisionStart>,
    mut q_ewr: Query<&mut EnemiesWithinReach>,
    q_tower_sight: Query<&ChildOf, With<TowerSightSensor>>,
) {
    for ev in evr.read() {
        // if *flag == CollisionEventFlags::SENSOR {
        edit_eir(
            ev.collider1,
            ev.collider2,
            &mut q_ewr,
            &q_tower_sight,
            |eir, enemy_id| {
                //log!("Insert: {enemy_id:?}");
                eir.0.insert(enemy_id);
            },
        );
    }
}

pub(super) fn on_enemy_leave_reach_system(
    mut evr: MessageReader<CollisionEnd>,
    mut q_ewr: Query<&mut EnemiesWithinReach>,
    q_tower_sight: Query<&ChildOf, With<TowerSightSensor>>,
) {
    for ev in evr.read() {
        // if *flag == CollisionEventFlags::SENSOR {
        edit_eir(
            ev.collider1,
            ev.collider2,
            &mut q_ewr,
            &q_tower_sight,
            |eir, enemy_id| {
                //log!("Remove: {enemy_id:?}");
                eir.0.remove(&enemy_id);
            },
        );
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
        if let Ok(ts_child_of) = q_tower_sight.get(tower_sight_id)
            && let Ok(mut eir) = q_eir.get_mut(ts_child_of.parent())
        {
            f(&mut eir, enemy_id);
            return;
        }
    }
}

pub(super) fn on_remove_despawned_enemies_from_ewr_system(
    mut evr: MessageReader<OnEnemyDespawnEvent>,
    mut q_ewr: Query<&mut EnemiesWithinReach>,
) {
    for ev in evr.read() {
        for mut ewr in q_ewr.iter_mut() {
            ewr.0.remove(&ev.0);
        }
    }
}

#[cfg(test)]
mod cone_tests {
    use super::*;

    #[test]
    fn enemy_inside_full_cone_width_is_hit() {
        let beam = Vec3::NEG_Z;
        let to_enemy = Vec3::new(0.1, 0., -0.3);
        assert!(is_in_cone(beam, to_enemy, 45_f32.to_radians() / 2.));
    }

    #[test]
    fn enemy_outside_cone_is_missed() {
        let beam = Vec3::NEG_Z;
        let to_enemy = Vec3::new(0.2, 0., -0.1);
        assert!(!is_in_cone(beam, to_enemy, 45_f32.to_radians() / 2.));
    }

    #[test]
    fn boundary_enemy_is_hit() {
        let beam = Vec3::NEG_Z;
        let to_enemy = beam + Vec3::X * (45_f32.to_radians() / 2. - 1e-4).tan();
        assert!(is_in_cone(beam, to_enemy, 45_f32.to_radians() / 2.));
    }

    #[test]
    fn zero_beam_or_target_never_matches() {
        assert!(!is_in_cone(Vec3::ZERO, Vec3::NEG_Z, 1.));
        assert!(!is_in_cone(Vec3::NEG_Z, Vec3::ZERO, 1.));
    }
}
