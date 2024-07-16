use self::tween_completed::on_tween_completed_system;
use super::EventState;
use crate::prelude::*;
pub mod collision;
pub mod tween_completed;

// Captures general events from other plugins and sends them as specific events
pub struct PinballEventsPlugin;

impl Plugin for PinballEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (on_tween_completed_system).run_if(in_state(EventState::Active)),
        );
    }
}
