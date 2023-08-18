use super::enemy::Enemy;
use super::progress_bar::ProgressBarCountUpEvent;
use super::world::QueryWorld;
use super::GameState;
use crate::prelude::*;
use crate::utils::RelEntity;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Update, ().run_if(in_state(GameState::Ingame)));
    }
}
