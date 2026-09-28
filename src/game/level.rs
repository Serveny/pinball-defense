use super::{
    EventState, GameState,
    analog_counter::AnalogCounterSetEvent,
    light::{FlashLight, LevelUpLamp},
    wave::{Wave, WaveStartedEvent},
};
use crate::prelude::*;
use std::time::Duration;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PointsEvent>()
            .add_message::<LevelUpEvent>()
            .register_type::<PointHub>()
            .register_type::<LevelHub>()
            .add_systems(OnEnter(GameState::Init), init_resources)
            .add_systems(
                Update,
                (
                    level_up_system,
                    update_points_counter_system,
                    update_level_counter_system,
                    level_up_animation_system,
                )
                    .run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (on_add_points_system, on_level_up_lamp).run_if(in_state(EventState::Active)),
            );
    }
}

// Using insert_resource to reset previous resources of same type
fn init_resources(mut cmds: Commands) {
    cmds.insert_resource(PointHub::default());
    cmds.insert_resource(LevelHub::default());
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum PointsKind {
    EnemyDied = 85,
    TowerUpgrade = 500,
    TowerBuild = 1000,
}

impl PointsKind {
    fn points(self) -> Points {
        self as Points
    }
}

const POINT_FACTOR: u32 = 1;

#[derive(Message, Clone, Copy)]
pub struct PointsEvent {
    pub points: Points,
    pub pos: Vec3,
}

impl PointsEvent {
    pub fn new(kind: PointsKind, pos: Vec3) -> Self {
        Self {
            points: kind.points() * POINT_FACTOR,
            pos,
        }
    }

    pub fn with_points(points: Points, pos: Vec3) -> Self {
        Self {
            points: points * POINT_FACTOR,
            pos,
        }
    }
}

fn on_add_points_system(mut evr: MessageReader<PointsEvent>, mut points: ResMut<PointHub>) {
    for ev in evr.read() {
        points.0 += ev.points;
    }
}

pub type Points = u32;
pub type Level = u32;

#[derive(Component, Clone, Copy)]
pub struct BallCollisionPoints(pub Points);

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PointHub(pub Points);

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct LevelHub {
    level: Level,
}

impl LevelHub {
    pub fn foundation_hit_progress(&self) -> f32 {
        1. / (f32::from(u16::try_from(self.level).unwrap_or(u16::MAX)) * 3.)
    }

    pub fn level(&self) -> Level {
        self.level
    }
}

#[derive(Message, Clone, Copy)]
pub struct LevelUpEvent(pub Level);

fn level_up_system(
    mut lvl_up_ev: MessageWriter<LevelUpEvent>,
    mut level: ResMut<LevelHub>,
    wave: Res<Wave>,
) {
    let new_level = Level::try_from(wave.number()).unwrap_or(Level::MAX);
    if level.level != new_level {
        level.level = new_level;
        lvl_up_ev.write(LevelUpEvent(new_level));
        log!("🥳 Level up: {new_level}!");
    }
}

#[derive(Resource)]
pub struct PointCounterId(pub Entity);

impl Default for PointCounterId {
    fn default() -> Self {
        Self(Entity::from_bits(0))
    }
}

fn update_points_counter_system(
    points: Res<PointHub>,
    mut ac_set_ev: MessageWriter<AnalogCounterSetEvent>,
    pc_id: Res<PointCounterId>,
) {
    if points.is_changed() {
        ac_set_ev.write(AnalogCounterSetEvent::new(pc_id.0, points.0));
    }
}

#[derive(Resource)]
pub struct LevelCounterId(pub Entity);

impl Default for LevelCounterId {
    fn default() -> Self {
        Self(Entity::from_bits(0))
    }
}

fn update_level_counter_system(
    level: Res<LevelHub>,
    mut ac_set_ev: MessageWriter<AnalogCounterSetEvent>,
    lc_id: Res<LevelCounterId>,
) {
    if level.is_changed() {
        ac_set_ev.write(AnalogCounterSetEvent::new(lc_id.0, level.level));
    }
}

#[derive(Component)]
struct LevelUpAnimation(Timer);

fn on_level_up_lamp(
    mut cmds: Commands,
    mut q_lvl_up_lamp: Query<(Entity, &mut Visibility), With<LevelUpLamp>>,
    mut wave_started_ev: MessageReader<WaveStartedEvent>,
) {
    for WaveStartedEvent { .. } in wave_started_ev.read() {
        let Ok((lamp_id, mut visi)) = q_lvl_up_lamp.single_mut() else {
            return;
        };
        *visi = Visibility::Inherited;
        cmds.entity(lamp_id)
            .insert(FlashLight)
            .insert(LevelUpAnimation(Timer::new(
                Duration::from_secs(4),
                TimerMode::Once,
            )));
    }
}

fn level_up_animation_system(
    mut cmds: Commands,
    mut q_anim: Query<(Entity, &mut Visibility, &mut LevelUpAnimation)>,
    time: Res<Time>,
) {
    for (lamp_id, mut visi, mut anim) in &mut q_anim {
        if anim.0.is_finished() {
            *visi = Visibility::Hidden;
            cmds.entity(lamp_id)
                .remove::<FlashLight>()
                .remove::<LevelUpAnimation>();
        } else {
            anim.0.tick(time.delta());
        }
    }
}
