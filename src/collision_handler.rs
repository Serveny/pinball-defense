use crate::prelude::*;
use crate::tower::{LightOnCollision, TowerBase, TowerFoundation, TowerType};
use crate::world::MenuElement;

pub struct CollisionHandlerPlugin;

impl Plugin for CollisionHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TowerBaseCollisionStartEvent>()
            .add_event::<TowerFoundationCollisionStartEvent>()
            .add_event::<LightOnEvent>()
            .add_event::<BuildTowerEvent>()
            .add_systems(Update, collision_system);
    }
}

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

#[allow(clippy::too_many_arguments)]
fn collision_system(
    mut col_events: EventReader<CollisionEvent>,
    mut light_on_ev: EventWriter<LightOnEvent>,
    mut tbc_start_ev: EventWriter<TowerBaseCollisionStartEvent>,
    mut tfc_start_ev: EventWriter<TowerFoundationCollisionStartEvent>,
    mut build_tower_ev: EventWriter<BuildTowerEvent>,
    q_light_on_coll: Query<Entity, With<LightOnCollision>>,
    q_tower_base: Query<Entity, With<TowerBase>>,
    q_tower_foundation: Query<Entity, With<TowerFoundation>>,
    q_menu_elements: Query<(Entity, &TowerType), With<MenuElement>>,
) {
    for col_ev in col_events.iter() {
        if let CollisionEvent::Started(entity, _, _) = col_ev {
            if let Some((_, tower_type)) = q_menu_elements.iter().find(|(id, _)| id == entity) {
                build_tower_ev.send(BuildTowerEvent(*tower_type));
                return;
            }
            if q_light_on_coll.contains(*entity) {
                light_on_ev.send(LightOnEvent(*entity));
            }
            if q_tower_base.contains(*entity) {
                tbc_start_ev.send(TowerBaseCollisionStartEvent(*entity));
            } else if q_tower_foundation.contains(*entity) {
                tfc_start_ev.send(TowerFoundationCollisionStartEvent(*entity));
            }
        }
    }
}
