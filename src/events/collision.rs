use crate::ball::PinBall;
use crate::flipper::FlipperCollider;
use crate::pinball_menu::PinballMenuEvent;
use crate::prelude::*;
use crate::progress_bar::ProgressBarCountUpEvent;
use crate::tower::base::TowerBase;
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

pub(super) fn collision_system(
    mut col_events: EventReader<CollisionEvent>,
    mut light_on_ev: EventWriter<ContactLightOnEvent>,
    mut build_tower_ev: EventWriter<BuildTowerEvent>,
    mut pb_menu_ev: EventWriter<PinballMenuEvent>,
    mut prog_bar_ev: EventWriter<ProgressBarCountUpEvent>,
    q_light_on_coll: Query<Entity, With<LightOnCollision>>,
    q_tower_base: Query<Entity, With<TowerBase>>,
    q_tower_foundation: Query<Entity, With<TowerFoundation>>,
    q_menu_els: Query<(Entity, &TowerType), Without<TowerBase>>,
    q_ball: Query<With<PinBall>>,
    q_flipper_collider: Query<Entity, With<FlipperCollider>>,
) {
    for ev in col_events.iter() {
        if let CollisionEvent::Started(mut entity, entity_2, flag) = ev {
            // Workaround: Elements not always in the same order
            if q_ball.contains(entity) {
                entity = *entity_2;
            }
            //log!("⛷️ Ball collided with: {:?} - Flag: {:?}", entity, flag);

            // Sensors & Colliders
            if q_light_on_coll.contains(entity) {
                light_on_ev.send(ContactLightOnEvent(entity));
            }

            // Only Sensors
            if *flag == CollisionEventFlags::SENSOR {
                if q_tower_foundation.contains(entity) {
                    prog_bar_ev.send(ProgressBarCountUpEvent(entity, 0.5));
                    return;
                }
                if let Some((_, tower_type)) = q_menu_els.iter().find(|(id, _)| *id == entity) {
                    build_tower_ev.send(BuildTowerEvent(*tower_type));
                    return;
                }
                return;
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
    }
}
