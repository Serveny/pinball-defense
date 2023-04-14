use crate::prelude::*;
use crate::tower::TowerBase;

pub struct CollisionHandlerPlugin;

impl Plugin for CollisionHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TowerBaseCollisionStartEvent>()
            .add_system(collision_system);
    }
}

pub struct TowerBaseCollisionStartEvent(pub Entity);

fn collision_system(
    mut col_events: EventReader<CollisionEvent>,
    mut tbc_start_ev: EventWriter<TowerBaseCollisionStartEvent>,
    tower_base: Query<Entity, With<TowerBase>>,
) {
    for col_ev in col_events.iter() {
        if let CollisionEvent::Started(entity, _, _) = col_ev {
            if tower_base.contains(*entity) {
                tbc_start_ev.send(TowerBaseCollisionStartEvent(*entity));
            }
        }
    }
}
