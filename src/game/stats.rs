use super::enemy::Enemy;
use super::health::ChangeHealthEvent;
use super::pinball_menu::UpgradeMenuExecuteEvent;
use super::tower::SpawnTowerEvent;
use super::wave::WaveStartedEvent;
use super::{EventState, GameState};
use crate::prelude::*;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Init), init_stats)
            .add_systems(
                Update,
                track_stats_system.run_if(in_state(EventState::Active)),
            );
    }
}

#[derive(Resource, Default)]
pub struct GameStats {
    pub damage_dealt: f32,
    pub towers_built: u32,
    pub upgrades_performed: u32,
    pub wave_number: usize,
}

fn init_stats(mut cmds: Commands) {
    cmds.insert_resource(GameStats::default());
}

#[allow(clippy::cast_possible_truncation)]
fn track_stats_system(
    mut stats: ResMut<GameStats>,
    mut health_evr: MessageReader<ChangeHealthEvent>,
    mut tower_evr: MessageReader<SpawnTowerEvent>,
    mut upgrade_evr: MessageReader<UpgradeMenuExecuteEvent>,
    mut wave_evr: MessageReader<WaveStartedEvent>,
    q_enemy: Query<(), With<Enemy>>,
) {
    for ev in health_evr.read() {
        if ev.amount < 0.0 && q_enemy.contains(ev.health_id) {
            stats.damage_dealt += -ev.amount;
        }
    }
    stats.towers_built += tower_evr.read().count() as u32;
    stats.upgrades_performed += upgrade_evr.read().count() as u32;
    stats.wave_number += wave_evr.read().count();
}
