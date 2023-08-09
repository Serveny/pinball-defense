use self::{
    collision::{
        collision_system, test_system_1, test_system_2, BuildTowerEvent, ContactLightOnEvent,
        EnemyInRangeOfTowerEvent, EnemyOutRangeOfTowerEvent, PinballEnemyHitEvent,
        TowerMenuElementCollisionStartEvent,
    },
    progress_bar::{on_progress_bar_empty_system, on_progress_bar_full_system},
    tween_completed::tween_completed_system,
};
use crate::prelude::*;
use crate::GameState;
pub mod collision;
pub mod progress_bar;
pub mod tween_completed;

// Captures general events from other plugins and sends them as specific events
pub struct PinballEventsPlugin;

impl Plugin for PinballEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TowerMenuElementCollisionStartEvent>()
            .add_event::<ContactLightOnEvent>()
            .add_event::<BuildTowerEvent>()
            .add_event::<PinballEnemyHitEvent>()
            .add_event::<EnemyInRangeOfTowerEvent>()
            .add_event::<EnemyOutRangeOfTowerEvent>()
            .add_systems(
                Update,
                (
                    collision_system,
                    tween_completed_system,
                    on_progress_bar_full_system,
                    on_progress_bar_empty_system,
                    test_system_1,
                    test_system_2,
                )
                    .run_if(in_state(GameState::Ingame)),
            );
    }
}
