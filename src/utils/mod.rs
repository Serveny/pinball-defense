use self::collision_events::collision_system;
use self::tween_completed_events::tween_completed_system;
use crate::prelude::*;
use crate::GameState;
pub(crate) mod collision_events;
pub(crate) mod tween_completed_events;
use collision_events::*;

pub struct PinballUtilsPlugin;

impl Plugin for PinballUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TowerBaseCollisionStartEvent>()
            .add_event::<TowerFoundationCollisionStartEvent>()
            .add_event::<TowerMenuElementCollisionStartEvent>()
            .add_event::<ContactLightOnEvent>()
            .add_event::<BuildTowerEvent>()
            .add_systems(
                Update,
                (collision_system, tween_completed_system).run_if(in_state(GameState::Ingame)),
            );
    }
}

#[derive(Component)]
pub struct RelParent(pub Entity);
