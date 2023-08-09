use crate::enemy::SpawnEnemyEvent;
use crate::prelude::*;
use crate::GameState;
use crate::IngameTime;

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WaveNo>()
            .init_resource::<EnemiesToSpawn>()
            .init_resource::<NextEnemySpawnTime>()
            .init_resource::<EnemiesToSpawn>()
            .add_systems(Update, (wave_system).run_if(in_state(GameState::Ingame)));
    }
}

#[derive(Debug, States, Hash, PartialEq, Eq, Clone, Copy, Default)]
enum WaveStatus {
    #[default]
    None,
    Active,
}

#[derive(Event)]
struct SpawnNextWaveEvent;

#[derive(Resource, Default, Deref, DerefMut)]
struct WaveNo(usize);

#[derive(Resource, Default, Deref, DerefMut)]
struct EnemiesToSpawn(usize);

#[derive(Resource, Deref, DerefMut, Default)]
struct NextEnemySpawnTime(f32);

const TIME_BETWEEN_WAVES: f32 = 4.;

fn wave_system(
    mut spawn_enemy_ev: EventWriter<SpawnEnemyEvent>,
    mut wave_no: ResMut<WaveNo>,
    mut enemies_count: ResMut<EnemiesToSpawn>,
    mut next_enemy_spawn: ResMut<NextEnemySpawnTime>,
    ig_timer: Res<IngameTime>,
) {
    let now = **ig_timer;
    if now >= **next_enemy_spawn {
        if **enemies_count == 0 {
            **wave_no += 1;
            **next_enemy_spawn = (now + TIME_BETWEEN_WAVES).round();
            log!("🏄‍♂️ Wave end. Wait until {}", **next_enemy_spawn);
            **enemies_count = **wave_no;
        } else {
            **enemies_count -= 1;
            **next_enemy_spawn = now + 1.;
            spawn_enemy_ev.send(SpawnEnemyEvent);
        }
    }
}
