use self::{
    collision::{
        collision_system, BuildTowerEvent, ContactLightOnEvent, PinballEnemyHitEvent,
        TowerMenuElementCollisionStartEvent,
    },
    progress_bar::on_progress_bar_full_system,
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
            .add_systems(
                Update,
                (
                    collision_system,
                    tween_completed_system,
                    on_progress_bar_full_system,
                )
                    .run_if(in_state(GameState::Ingame)),
            );
    }
}
