use crate::ball::PinBall;
use crate::flipper::FlipperCollider;
use crate::pinball_menu::PinballMenuEvent;
use crate::prelude::*;
use crate::tower::base::TowerBase;
use crate::tower::foundation::TowerFoundation;
use crate::tower::light::LightOnCollision;
use crate::tower::TowerType;

#[derive(Event)]
pub struct TowerBaseCollisionStartEvent(pub Entity);

#[derive(Event)]
pub struct TowerFoundationCollisionStartEvent(pub Entity);

#[derive(Event)]
pub struct TowerMenuElementCollisionStartEvent(pub Entity);

#[derive(Event)]
pub struct LightOnEvent(pub Entity);

#[derive(Event)]
pub struct BuildTowerEvent(pub TowerType);

pub(super) fn collision_system(
    mut col_events: EventReader<CollisionEvent>,
    mut light_on_ev: EventWriter<LightOnEvent>,
    mut tbc_start_ev: EventWriter<TowerBaseCollisionStartEvent>,
    mut tfc_start_ev: EventWriter<TowerFoundationCollisionStartEvent>,
    mut build_tower_ev: EventWriter<BuildTowerEvent>,
    mut pb_menu_ev: EventWriter<PinballMenuEvent>,
    q_light_on_coll: Query<Entity, With<LightOnCollision>>,
    q_tower_base: Query<Entity, With<TowerBase>>,
    q_tower_foundation: Query<Entity, With<TowerFoundation>>,
    q_menu_elements: Query<(Entity, &TowerType), Without<TowerBase>>,
    q_ball: Query<Entity, With<PinBall>>,
    q_flipper_collider: Query<Entity, With<FlipperCollider>>,
) {
    for ev in col_events.iter() {
        if let CollisionEvent::Started(mut entity, entity_2, _) = ev {
            //println!("🥳 Event geworfen: {:?}", ev);
            // Workaround: Elements not always in the same order
            if q_ball.contains(entity) {
                entity = *entity_2;
            }
            if q_flipper_collider.contains(entity) {
                pb_menu_ev.send(PinballMenuEvent::Activate);
                return;
            }
            if let Some((_, tower_type)) = q_menu_elements.iter().find(|(id, _)| *id == entity) {
                build_tower_ev.send(BuildTowerEvent(*tower_type));
                return;
            }
            if q_light_on_coll.contains(entity) {
                light_on_ev.send(LightOnEvent(entity));
            }
            if q_tower_base.contains(entity) {
                tbc_start_ev.send(TowerBaseCollisionStartEvent(entity));
            } else if q_tower_foundation.contains(entity) {
                tfc_start_ev.send(TowerFoundationCollisionStartEvent(entity));
            }
        }
    }
}
