use super::enemy::SpawnEnemyEvent;
use super::GameState;
use super::IngameTime;
use crate::prelude::*;

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Wave>()
            .add_systems(Update, (wave_system).run_if(in_state(GameState::Ingame)));
    }
}

#[derive(Debug, States, Hash, PartialEq, Eq, Clone, Copy, Default)]
enum WaveStatus {
    #[default]
    None,
    Active,
}

#[derive(Resource)]
struct Wave {
    number: usize,
    enemies_count: usize,
    next_enemy_spawn_time: f32,
    time_between_enemies: f32,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            number: 0,
            enemies_count: 0,
            next_enemy_spawn_time: 0.,
            time_between_enemies: 1.,
        }
    }
}

impl Wave {
    fn is_time_to_spawn_enemy(&self, now: f32) -> bool {
        now >= self.next_enemy_spawn_time
    }

    fn is_wave_end(&self) -> bool {
        self.enemies_count == 0
    }

    fn next_enemy(&mut self, now: f32) -> SpawnEnemyEvent {
        self.enemies_count -= 1;
        self.next_enemy_spawn_time = now + self.time_between_enemies;
        SpawnEnemyEvent
    }

    fn prepare_next_wave(&mut self, now: f32) {
        self.number += 1;
        self.next_enemy_spawn_time = (now + TIME_BETWEEN_WAVES).round();
        self.enemies_count = (self.number as f32 * 1.5) as usize;
        self.time_between_enemies *= 0.999;
        log!("🏄‍♂️ Wave end. Wait until {}", self.next_enemy_spawn_time);
    }
}

#[derive(Event)]
struct SpawnNextWaveEvent;

const TIME_BETWEEN_WAVES: f32 = 8.;

fn wave_system(
    mut wave: ResMut<Wave>,
    mut spawn_enemy_ev: EventWriter<SpawnEnemyEvent>,
    ig_timer: Res<IngameTime>,
) {
    let now = **ig_timer;
    let wave = wave.as_mut();
    if wave.is_time_to_spawn_enemy(now) {
        match wave.is_wave_end() {
            true => wave.prepare_next_wave(now),
            false => spawn_enemy_ev.send(wave.next_enemy(now)),
        }
    }
}
