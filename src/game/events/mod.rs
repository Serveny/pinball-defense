use self::{
    collision::{PinballEnemyHitEvent, TowerMenuElementCollisionStartEvent},
    tween_completed::tween_completed_system,
};
use crate::game::GameState;
use crate::prelude::*;
pub mod collision;
pub mod tween_completed;

// Captures general events from other plugins and sends them as specific events
pub struct PinballEventsPlugin;

impl Plugin for PinballEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TowerMenuElementCollisionStartEvent>()
            .add_event::<PinballEnemyHitEvent>()
            .add_systems(
                Update,
                (tween_completed_system).run_if(in_state(GameState::Ingame)),
            );
    }
}
