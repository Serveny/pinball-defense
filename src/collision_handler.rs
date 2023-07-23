use crate::prelude::*;
use crate::tower::{TowerBase, TowerFoundation};

pub struct CollisionHandlerPlugin;

impl Plugin for CollisionHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TowerBaseCollisionStartEvent>()
            .add_event::<TowerFoundationCollisionStartEvent>()
            .add_systems(Update, collision_system);
    }
}

#[derive(Event)]
pub struct TowerBaseCollisionStartEvent(pub Entity);

#[derive(Event)]
pub struct TowerFoundationCollisionStartEvent(pub Entity);

fn collision_system(
    mut col_events: EventReader<CollisionEvent>,
    mut tbc_start_ev: EventWriter<TowerBaseCollisionStartEvent>,
    mut tfc_start_ev: EventWriter<TowerFoundationCollisionStartEvent>,
    tower_base: Query<Entity, With<TowerBase>>,
    tower_foundation: Query<Entity, With<TowerFoundation>>,
) {
    for col_ev in col_events.iter() {
        if let CollisionEvent::Started(entity, _, _) = col_ev {
            if tower_base.contains(*entity) {
                tbc_start_ev.send(TowerBaseCollisionStartEvent(*entity));
            } else if tower_foundation.contains(*entity) {
                tfc_start_ev.send(TowerFoundationCollisionStartEvent(*entity));
            }
        }
    }
}
