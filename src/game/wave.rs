use super::GameState;
use super::IngameTime;
use super::ball_starter::BallStarterFireEndEvent;
use super::enemy::{Enemy, EnemyKind, SpawnEnemyEvent};
use crate::prelude::*;
use moonshine_save::load::Loaded;
use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::SmallRng;

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<WaveStartedEvent>()
            .register_type::<Wave>()
            .add_observer(on_loaded_sync_wave)
            .add_systems(OnEnter(GameState::Init), init_resources)
            .add_systems(
                Update,
                (start_wave_system, wave_system)
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

fn init_resources(mut cmds: Commands) {
    cmds.insert_resource(Wave::default());
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct Wave {
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
    pub fn number(&self) -> usize {
        self.number
    }

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
        self.next_enemy_spawn_time = now + TIME_BETWEEN_WAVES;
        self.time_between_enemies = (BASE_TIME_BETWEEN_ENEMIES
            * 0.97f32.powi(i32::try_from(self.number).unwrap_or(i32::MAX)))
        .max(MIN_TIME_BETWEEN_ENEMIES);
        self.roll_wave_kind();
        self.enemies_count = match self.kind {
            EnemyKind::Tank => enemies_per_wave(self.number) / 3,
            _ => enemies_per_wave(self.number),
        };
        self.announce_pending = true;
        log!("🏄‍♂️ Wave end. Wait until {}", self.next_enemy_spawn_time);
    }

    fn sync_to_loaded_state(&mut self, now: f32) {
        if self.number > 0 {
            self.started = true;
            self.announce_pending = self.enemies_count > 0;
            self.next_enemy_spawn_time = now + TIME_BETWEEN_WAVES;
        }
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
    let speeder_ready = number >= 10;
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
    let speeder_unlocked = number >= 10;
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

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
fn enemies_per_wave(wave: usize) -> usize {
    (f32::from(u16::try_from(wave).unwrap_or(u16::MAX)).powf(1.25)) as usize
}

const TIME_BETWEEN_WAVES: f32 = 4.;
const BASE_TIME_BETWEEN_ENEMIES: f32 = 1.;
const MIN_TIME_BETWEEN_ENEMIES: f32 = 0.3;

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

fn on_loaded_sync_wave(_: On<Loaded>, mut wave: ResMut<Wave>, ig_timer: Res<IngameTime>) {
    wave.sync_to_loaded_state(**ig_timer);
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
    if wave.started {
        if wave.is_wave_end()
            && !wave.announce_pending
            && wave.is_time_to_spawn_enemy(now)
            && q_enemy.is_empty()
        {
            wave.prepare_next_wave(now);
        } else if wave.is_time_to_spawn_enemy(now) && wave.announce_pending {
            if q_enemy.is_empty() {
                wave_started_ev.write(WaveStartedEvent {
                    number: wave.number,
                    kind: wave.kind,
                });
                wave.announce_pending = false;
                spawn_enemy_ev.write(wave.next_enemy(now));
            } else {
                wave.next_enemy_spawn_time = now + 1.;
            }
        } else if wave.is_time_to_spawn_enemy(now) && !wave.is_wave_end() {
            spawn_enemy_ev.write(wave.next_enemy(now));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;

    #[test]
    fn next_level_waits_four_seconds_after_last_enemy() {
        let mut app = App::new();
        app.add_message::<SpawnEnemyEvent>()
            .add_message::<WaveStartedEvent>()
            .insert_resource(IngameTime(10.))
            .insert_resource(Wave::default())
            .add_systems(Update, wave_system);
        {
            let mut wave = app.world_mut().resource_mut::<Wave>();
            wave.started = true;
            wave.number = 1;
            wave.next_enemy_spawn_time = 11.;
        }
        app.update();
        assert_eq!(app.world().resource::<Wave>().number, 1);
        app.world_mut().resource_mut::<IngameTime>().0 = 11.;
        app.update();
        let wave = app.world().resource::<Wave>();
        assert_eq!(wave.number, 2);
        assert!((wave.next_enemy_spawn_time - 15.).abs() < 0.0001);
        app.world_mut().resource_mut::<IngameTime>().0 = 14.9;
        app.update();
        assert!(app.world().resource::<Wave>().announce_pending);
    }

    fn kinds<R: RngExt>(number: usize, cooldown: usize, rng: &mut R) -> EnemyKind {
        decide_wave_kind(number, cooldown, rng).0
    }

    #[test]
    fn enemy_count_grows_sublinearly() {
        assert_eq!(enemies_per_wave(1), 1);
        assert_eq!(enemies_per_wave(2), 2);
        assert_eq!(enemies_per_wave(5), 7);
        assert_eq!(enemies_per_wave(10), 17);
        assert_eq!(enemies_per_wave(50), 132);
        assert_eq!(enemies_per_wave(100), 316);
        for wave in [1, 10, 50, 100] {
            assert!(
                enemies_per_wave(wave) > enemies_per_wave(wave - 1),
                "wave {wave} must grow"
            );
        }
    }

    #[test]
    fn loaded_fully_spawned_wave_announces_next_wave_without_underflow() {
        let mut app = App::new();
        app.add_message::<SpawnEnemyEvent>()
            .add_message::<WaveStartedEvent>()
            .insert_resource(IngameTime(10.))
            .insert_resource(Wave::default())
            .add_systems(Update, wave_system);
        {
            let mut wave = app.world_mut().resource_mut::<Wave>();
            wave.number = 5;
            wave.enemies_count = 0;
            wave.sync_to_loaded_state(10.);
        }
        let wave = app.world().resource::<Wave>();
        assert!(wave.started);
        assert!(!wave.announce_pending);
        for t in [14., 15.] {
            app.world_mut().resource_mut::<IngameTime>().0 = t;
            app.update();
        }
        assert_eq!(app.world().resource::<Wave>().number, 6);
    }

    #[test]
    fn no_specials_before_wave_ten() {
        let mut rng = StdRng::seed_from_u64(42);
        assert_eq!(kinds(9, 50, &mut rng), EnemyKind::Normal);
        assert_eq!(kinds(1, 50, &mut rng), EnemyKind::Normal);
        assert_ne!(kinds(9, 50, &mut rng), EnemyKind::Speeder);
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
    fn no_speeder_special_before_ten() {
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..200 {
            let (kind, special) = decide_wave_kind(9, 50, &mut rng);
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
            assert_ne!(spawn_kind(EnemyKind::Normal, 9), EnemyKind::Speeder);
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
