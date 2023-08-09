use crate::ball::PinBall;
use crate::enemy::Enemy;
use crate::flipper::FlipperCollider;
use crate::pinball_menu::PinballMenuEvent;
use crate::prelude::*;
use crate::progress_bar::ProgressBarCountUpEvent;
use crate::tower::base::{TowerBase, TowerSightSensor};
use crate::tower::foundation::TowerFoundation;
use crate::tower::light::LightOnCollision;
use crate::tower::TowerType;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

#[derive(Event)]
pub struct TowerMenuElementCollisionStartEvent(pub Entity);

#[derive(Event)]
pub struct ContactLightOnEvent(pub Entity);

#[derive(Event)]
pub struct BuildTowerEvent(pub TowerType);

#[derive(Event)]
pub struct PinballEnemyHitEvent(pub Entity);

#[derive(Event, Debug)]
pub struct EnemyInRangeOfTowerEvent {
    enemy_id: Entity,
    tower_id: Entity,
}

impl EnemyInRangeOfTowerEvent {
    pub fn new(enemy_id: Entity, tower_id: Entity) -> Self {
        Self { enemy_id, tower_id }
    }
}

#[derive(Event, Debug)]
pub struct EnemyOutRangeOfTowerEvent {
    enemy_id: Entity,
    tower_id: Entity,
}

impl EnemyOutRangeOfTowerEvent {
    pub fn new(enemy_id: Entity, tower_id: Entity) -> Self {
        Self { enemy_id, tower_id }
    }
}

// Collision Groups
pub const BALL: Group = Group::GROUP_1;

pub const INTERACT_WITH_BALL: Group = Group::GROUP_2;

pub fn collider_only_interact_with_ball() -> CollisionGroups {
    CollisionGroups::new(INTERACT_WITH_BALL, BALL)
}

pub const ENEMY: Group = Group::GROUP_3;
pub const INTERACT_WITH_ENEMY: Group = Group::GROUP_4;

pub fn collider_only_interact_with_enemy() -> CollisionGroups {
    CollisionGroups::new(INTERACT_WITH_ENEMY, ENEMY)
}

pub(super) fn collision_system(
    mut col_events: EventReader<CollisionEvent>,
    mut light_on_ev: EventWriter<ContactLightOnEvent>,
    mut build_tower_ev: EventWriter<BuildTowerEvent>,
    mut pb_menu_ev: EventWriter<PinballMenuEvent>,
    mut prog_bar_ev: EventWriter<ProgressBarCountUpEvent>,
    mut enemy_hit_ev: EventWriter<PinballEnemyHitEvent>,
    mut enemy_in_range_ev: EventWriter<EnemyInRangeOfTowerEvent>,
    mut enemy_out_range_ev: EventWriter<EnemyOutRangeOfTowerEvent>,
    q_light_on_coll: Query<Entity, With<LightOnCollision>>,
    q_tower_base: Query<Entity, With<TowerBase>>,
    q_tower_foundation: Query<Entity, With<TowerFoundation>>,
    q_menu_els: Query<(Entity, &TowerType), Without<TowerBase>>,
    q_ball: Query<With<PinBall>>,
    q_flipper_collider: Query<Entity, With<FlipperCollider>>,
    q_enemy: Query<With<Enemy>>,
    q_tower_sight: Query<With<TowerSightSensor>>,
) {
    for ev in col_events.iter() {
        match ev {
            CollisionEvent::Started(mut entity, mut entity_2, flag) => {
                log!(
                    "😊 Collision detected: {:?} - {:?} | Flag: {:?}",
                    entity,
                    entity_2,
                    flag
                );
                if q_tower_sight.contains(entity) {
                    log!("Tower sight 1");
                    enemy_in_range_ev.send(EnemyInRangeOfTowerEvent::new(entity_2, entity));
                    continue;
                }
                if q_tower_sight.contains(entity_2) {
                    log!("Tower sight 1");
                    enemy_in_range_ev.send(EnemyInRangeOfTowerEvent::new(entity, entity_2));
                    continue;
                }

                log!("⛷️ Ball collided with: {:?} - Flag: {:?}", entity, flag);

                // Workaround: Elements not always in the same order
                if q_ball.contains(entity) {
                    log!("⚠ Order new");
                    std::mem::swap(&mut entity, &mut entity_2);
                }

                // Sensors & Colliders
                if q_light_on_coll.contains(entity) {
                    light_on_ev.send(ContactLightOnEvent(entity));
                }

                // Only Sensors
                if *flag == CollisionEventFlags::SENSOR {
                    if q_tower_foundation.contains(entity) {
                        prog_bar_ev.send(ProgressBarCountUpEvent(entity, 1.));
                        return;
                    }
                    if let Some((_, tower_type)) = q_menu_els.iter().find(|(id, _)| *id == entity) {
                        build_tower_ev.send(BuildTowerEvent(*tower_type));
                        return;
                    }
                    if q_enemy.contains(entity) {
                        enemy_hit_ev.send(PinballEnemyHitEvent(entity));
                    }
                }

                // Only Colliders
                if q_flipper_collider.contains(entity) {
                    pb_menu_ev.send(PinballMenuEvent::Activate);
                    return;
                }

                if q_tower_base.contains(entity) {
                    prog_bar_ev.send(ProgressBarCountUpEvent(entity, 0.05));
                }
            }
            CollisionEvent::Stopped(entity, entity_2, flag) => {
                let entity = *entity;
                let entity_2 = *entity_2;

                if *flag == CollisionEventFlags::SENSOR {
                    if q_tower_sight.contains(entity) {
                        enemy_out_range_ev.send(EnemyOutRangeOfTowerEvent::new(entity_2, entity));
                        continue;
                    }

                    if q_tower_sight.contains(entity_2) {
                        enemy_out_range_ev.send(EnemyOutRangeOfTowerEvent::new(entity, entity_2));
                        continue;
                    }
                }
            }
        }
    }
}

pub fn test_system_1(mut evr: EventReader<EnemyInRangeOfTowerEvent>) {
    for ev in evr.iter() {
        log!("Test1: {:?}", ev);
    }
}

pub fn test_system_2(mut evr: EventReader<EnemyInRangeOfTowerEvent>) {
    for ev in evr.iter() {
        log!("Test2: {:?}", ev);
    }
}
