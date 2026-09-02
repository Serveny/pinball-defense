use super::GameState;
use super::IngameTime;
use super::analog_counter::AnalogCounterSetEvent;
use super::ball_starter::BallStarterFireEndEvent;
use super::enemy::{Enemy, EnemyKind, SpawnEnemyEvent};
use crate::prelude::*;
use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::SmallRng;

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<WaveStartedEvent>()
            .add_systems(OnEnter(GameState::Init), init_resources)
            .add_systems(
                Update,
                (start_wave_system, wave_system, update_wave_counter_system)
                    .chain()
                    .run_if(in_state(GameState::Ingame)),
            );
    }
}

#[derive(Message, Clone, Copy)]
pub struct WaveStartedEvent {
    pub number: usize,
    pub kind: EnemyKind,
}

#[derive(Resource)]
pub struct WaveCounterId(pub Entity);

impl Default for WaveCounterId {
    fn default() -> Self {
        Self(Entity::from_bits(0))
    }
}

fn update_wave_counter_system(
    wave: Res<Wave>,
    wave_started_ev: MessageReader<WaveStartedEvent>,
    mut ac_set_ev: MessageWriter<AnalogCounterSetEvent>,
    wc_id: Res<WaveCounterId>,
) {
    if !wave_started_ev.is_empty() {
        ac_set_ev.write(AnalogCounterSetEvent::new(
            wc_id.0,
            u32::try_from(wave.number).unwrap_or(u32::MAX),
        ));
    }
}

fn init_resources(mut cmds: Commands) {
    cmds.insert_resource(Wave::default());
}

#[derive(Resource)]
struct Wave {
    number: usize,
    enemies_count: usize,
    next_enemy_spawn_time: f32,
    time_between_enemies: f32,
    started: bool,
    kind: EnemyKind,
    special_cooldown: usize,
    announce_pending: bool,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            number: 0,
            enemies_count: 0,
            next_enemy_spawn_time: 0.,
            time_between_enemies: 1.,
            started: false,
            kind: EnemyKind::Normal,
            special_cooldown: SPECIAL_COOLDOWN,
            announce_pending: false,
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
        SpawnEnemyEvent {
            wave: self.number,
            kind: spawn_kind(self.kind, self.number),
        }
    }

    fn prepare_next_wave(&mut self, now: f32) {
        self.number += 1;
        self.next_enemy_spawn_time = (now + TIME_BETWEEN_WAVES).round();
        self.time_between_enemies *= 0.999;
        self.roll_wave_kind();
        let count = self.number * 3 / 2;
        self.enemies_count = match self.kind {
            EnemyKind::Tank => count / 3,
            _ => count,
        };
        self.announce_pending = true;
        log!("🏄‍♂️ Wave end. Wait until {}", self.next_enemy_spawn_time);
    }

    fn roll_wave_kind(&mut self) {
        let mut rng = SmallRng::from_rng(&mut rand::rng());
        let (kind, special) = decide_wave_kind(self.number, self.special_cooldown, &mut rng);
        self.kind = kind;
        self.special_cooldown = if special {
            0
        } else {
            self.special_cooldown + 1
        };
        if special {
            log!("⚡ Special wave {}: {:?}", self.number, kind);
        }
    }
}

const SPECIAL_COOLDOWN: usize = 10;

fn decide_wave_kind<R: RngExt>(
    number: usize,
    special_cooldown: usize,
    rng: &mut R,
) -> (EnemyKind, bool) {
    let tank_ready = number >= 10;
    let speeder_ready = number >= 20;
    if (tank_ready || speeder_ready)
        && special_cooldown >= SPECIAL_COOLDOWN
        && rng.random_bool(0.25)
    {
        return if speeder_ready && (!tank_ready || rng.random_bool(0.5)) {
            (EnemyKind::Speeder, true)
        } else {
            (EnemyKind::Tank, true)
        };
    }
    (EnemyKind::Normal, false)
}

fn spawn_kind(wave_kind: EnemyKind, number: usize) -> EnemyKind {
    if wave_kind != EnemyKind::Normal {
        return wave_kind;
    }
    let tank_unlocked = number >= 10;
    let speeder_unlocked = number >= 20;
    if !tank_unlocked && !speeder_unlocked {
        return EnemyKind::Normal;
    }
    let mut rng = SmallRng::from_rng(&mut rand::rng());
    if speeder_unlocked && rng.random_bool(0.05) {
        EnemyKind::Speeder
    } else if rng.random_bool(0.2) {
        EnemyKind::Tank
    } else {
        EnemyKind::Normal
    }
}

const TIME_BETWEEN_WAVES: f32 = 12.;

fn start_wave_system(
    mut wave: ResMut<Wave>,
    mut fire_end_ev: MessageReader<BallStarterFireEndEvent>,
    ig_timer: Res<IngameTime>,
) {
    if wave.started || fire_end_ev.read().next().is_none() {
        return;
    }
    wave.started = true;
    wave.prepare_next_wave(**ig_timer);
}

fn wave_system(
    mut wave: ResMut<Wave>,
    mut spawn_enemy_ev: MessageWriter<SpawnEnemyEvent>,
    mut wave_started_ev: MessageWriter<WaveStartedEvent>,
    ig_timer: Res<IngameTime>,
    q_enemy: Query<(), With<Enemy>>,
) {
    let now = **ig_timer;
    let wave = wave.as_mut();
    if wave.started && wave.is_time_to_spawn_enemy(now) {
        if wave.is_wave_end() {
            if q_enemy.is_empty() {
                wave.prepare_next_wave(now);
            } else {
                wave.next_enemy_spawn_time = now + 1.;
            }
        } else {
            if wave.announce_pending {
                wave_started_ev.write(WaveStartedEvent {
                    number: wave.number,
                    kind: wave.kind,
                });
                wave.announce_pending = false;
            }
            spawn_enemy_ev.write(wave.next_enemy(now));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;

    fn kinds<R: RngExt>(number: usize, cooldown: usize, rng: &mut R) -> EnemyKind {
        decide_wave_kind(number, cooldown, rng).0
    }

    #[test]
    fn no_specials_before_wave_ten() {
        let mut rng = StdRng::seed_from_u64(42);
        assert_eq!(kinds(9, 50, &mut rng), EnemyKind::Normal);
        assert_eq!(kinds(1, 50, &mut rng), EnemyKind::Normal);
        assert_ne!(kinds(19, 50, &mut rng), EnemyKind::Speeder);
    }

    #[test]
    fn cooldown_blocks_specials() {
        let mut rng = StdRng::seed_from_u64(42);
        assert_eq!(
            kinds(100, SPECIAL_COOLDOWN - 1, &mut rng),
            EnemyKind::Normal
        );
    }

    #[test]
    fn no_speeder_special_before_twenty() {
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..200 {
            let (kind, special) = decide_wave_kind(15, 50, &mut rng);
            assert!(!(special && kind == EnemyKind::Speeder));
        }
    }

    #[test]
    fn specials_only_after_cooldown_with_probability() {
        let mut rng = StdRng::seed_from_u64(3);
        let mut specials = 0;
        for _ in 0..1000 {
            let (_, special) = decide_wave_kind(50, 50, &mut rng);
            specials += usize::from(special);
        }
        assert!((200..400).contains(&specials), "specials: {specials}");
    }

    #[test]
    fn mixed_waves_respect_unlocks() {
        for n in 0..10 {
            for _ in 0..50 {
                assert_eq!(spawn_kind(EnemyKind::Normal, n), EnemyKind::Normal);
            }
        }
        for _ in 0..200 {
            assert_ne!(spawn_kind(EnemyKind::Normal, 12), EnemyKind::Speeder);
            assert_eq!(spawn_kind(EnemyKind::Tank, 5), EnemyKind::Tank);
            assert_eq!(spawn_kind(EnemyKind::Speeder, 5), EnemyKind::Speeder);
        }
        let mut has_tank = false;
        let mut has_speeder = false;
        for _ in 0..2000 {
            match spawn_kind(EnemyKind::Normal, 25) {
                EnemyKind::Tank => has_tank = true,
                EnemyKind::Speeder => has_speeder = true,
                EnemyKind::Normal => {}
            }
        }
        assert!(has_tank && has_speeder);
    }
}
