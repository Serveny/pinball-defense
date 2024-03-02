use crate::prelude::*;

mod controls;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UiState {
    #[default]
    None,
    Controls,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UiState>()
            .add_systems(OnEnter(UiState::Controls), controls::spawn)
            .add_systems(
                Update,
                (controls::keys_to_pos).run_if(in_state(UiState::Controls)),
            )
            .add_systems(OnExit(UiState::Controls), controls::despawn);
    }
}
