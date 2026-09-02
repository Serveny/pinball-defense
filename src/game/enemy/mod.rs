use self::step::Step;
use self::walk::{RoadEndReachedEvent, WALK_SPEED, on_road_end_reached_system, walk_system};
use super::audio::SoundEvent;
use super::ball::PinBall;
use super::events::collision::GameLayer;
use super::extra_field::ActiveEffects;
use super::health::{ChangeHealthEvent, Health, HealthEmptyEvent};
use super::level::{BallCollisionPoints, PointsEvent, PointsKind};
use super::{EventState, IngameTime, ui};
use crate::game::GameState;
use crate::game::ball::CollisionWithBallEvent;
use crate::game::world::QueryWorld;
use crate::generated::world_1::road_points::ROAD_POINTS;
use crate::prelude::*;
use bevy::math::primitives::{Cone, Cylinder, Sphere};
use moonshine_save::prelude::Save;
use std::time::Duration;

mod step;
mod walk;

pub(crate) use walk::recover_speed_system;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnEnemyEvent>()
            .add_message::<RoadEndReachedEvent>()
            .add_message::<OnEnemyDespawnEvent>()
            .register_type::<Enemy>()
            .register_type::<step::Step>()
            .add_systems(
                Update,
                reattach_enemies_system.run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (walk_system, recover_speed_system).run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (
                    on_pinball_hit_system,
                    on_spawn_system,
                    on_health_empty_system,
                    on_road_end_reached_system,
                )
                    .run_if(in_state(EventState::Active)),
            );
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Save)]
pub struct Enemy {
    step: Step,
    kind: EnemyKind,
    speed: f32,
    current_speed: f32,
    wave: usize,
}

#[derive(Component, Reflect, Clone, Copy, Debug, PartialEq, Eq, Default)]
#[reflect(Default)]
pub enum EnemyKind {
    #[default]
    Normal,
    Tank,
    Speeder,
}

impl EnemyKind {
    pub fn speed_factor(self) -> f32 {
        match self {
            Self::Normal => 1.,
            Self::Tank => 0.5,
            Self::Speeder => 3.,
        }
    }

    pub fn health_factor(self) -> f32 {
        match self {
            Self::Normal => 1.,
            Self::Tank => 3.,
            Self::Speeder => 0.5,
        }
    }
}

impl Enemy {
    pub fn new(wave: usize, kind: EnemyKind) -> Self {
        let speed = WALK_SPEED * kind.speed_factor();
        Self {
            step: Step::new(1),
            kind,
            speed,
            current_speed: speed,
            wave,
        }
    }

    pub fn kind(&self) -> EnemyKind {
        self.kind
    }

    pub fn walk(&mut self, current_pos: Vec3, dur: Duration) -> Option<Vec3> {
        let distance = dur.as_secs_f32() * self.current_speed;
        let mut new_pos = self.step.walk(current_pos, distance);
        if self.step.is_reached_point() {
            if self.step.is_reached_road_end() {
                return None;
            }
            self.step = self.step.next();
            new_pos = self.step.start_pos();
        }
        Some(new_pos)
    }

    pub fn slow_down(&mut self, factor: f32) {
        self.current_speed = self.speed * factor;
    }
}

#[derive(Message)]
pub struct SpawnEnemyEvent {
    pub wave: usize,
    pub kind: EnemyKind,
}

fn on_spawn_system(
    mut cmds: Commands,
    mut evr: MessageReader<SpawnEnemyEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    q_pqw: QueryWorld,
) {
    for ev in evr.read() {
        let mut enemy_id: Option<Entity> = None;
        let Ok(world) = q_pqw.single() else {
            warn!("[enemy spawn] no world");
            return;
        };
        cmds.entity(world).with_children(|spawner| {
            enemy_id = Some(
                spawner
                    .spawn(enemy(&mut meshes, &mut mats, ev.wave, ev.kind))
                    .id(),
            );
        });
        if let Some(enemy_id) = enemy_id {
            ui::progress_bar::spawn(&mut cmds, enemy_id, 1.);
        }
    }
}

#[derive(Component)]
pub struct LastDamager(pub Option<Entity>);

fn enemy_color(wave: usize) -> Color {
    let t = (wave_sat(wave) / 100.).min(1.);
    let hue = 30. + (220. - 30.) * t;
    let lightness = 0.6 - 0.55 * t;
    Color::hsl(hue, 0.9, lightness)
}

#[allow(clippy::cast_precision_loss)]
fn wave_sat(wave: usize) -> f32 {
    f32::from(u16::try_from(wave).unwrap_or(u16::MAX))
}

fn enemy_view_bundle(
    meshes: &mut Assets<Mesh>,
    mats: &mut Assets<StandardMaterial>,
    wave: usize,
    kind: EnemyKind,
) -> impl Bundle {
    let color = enemy_color(wave);
    (
        Name::new("Enemy"),
        Mesh3d(meshes.add(match kind {
            EnemyKind::Normal => Mesh::from(Sphere { radius: 0.03 }),
            EnemyKind::Tank => Mesh::from(Cylinder {
                radius: 0.036,
                half_height: 0.018,
            }),
            EnemyKind::Speeder => Mesh::from(Cone {
                radius: 0.021,
                height: 0.05,
            }),
        })),
        MeshMaterial3d(mats.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 1.,
            metallic: 0.,
            reflectance: 0.,
            ..default()
        })),
        Sensor,
        RigidBody::Kinematic,
        Collider::circle(match kind {
            EnemyKind::Normal => 0.03,
            EnemyKind::Tank => 0.036,
            EnemyKind::Speeder => 0.021,
        }),
        CollisionEventsEnabled,
        DebugRender::default().with_collider_color(color),
        CollisionLayers::new(GameLayer::Enemy, [GameLayer::Ball, GameLayer::Tower]),
        BallCollisionPoints(15),
        BallSlowDown(match kind {
            EnemyKind::Tank => TANK_SLOW_DOWN,
            _ => 1.,
        }),
        Restitution {
            coefficient: 2.,
            combine_rule: CoefficientCombine::Multiply,
        },
    )
}

const TANK_SLOW_DOWN: f32 = 0.9;

#[derive(Component)]
pub struct BallSlowDown(pub f32);

fn enemy(
    meshes: &mut Assets<Mesh>,
    mats: &mut Assets<StandardMaterial>,
    wave: usize,
    kind: EnemyKind,
) -> impl Bundle {
    (
        enemy_view_bundle(meshes, mats, wave, kind),
        Enemy::new(wave, kind),
        Health::new(100. * (1. + wave_sat(wave) * 0.5) * kind.health_factor()),
        LastDamager(None),
        Transform::from_translation(ROAD_POINTS[0]),
    )
}

fn reattach_enemies_system(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    q_world: QueryWorld,
    q_enemies: Query<(Entity, &Enemy), Without<Collider>>,
) {
    let Ok(world) = q_world.single() else { return };
    for (enemy_id, enemy) in q_enemies.iter() {
        cmds.entity(enemy_id).insert(enemy_view_bundle(
            &mut meshes,
            &mut mats,
            enemy.wave,
            enemy.kind(),
        ));
        cmds.entity(world).add_child(enemy_id);
    }
}

const BALL_DAMAGE: f32 = 250.;

fn on_pinball_hit_system(
    mut evr: MessageReader<CollisionWithBallEvent>,
    mut sound_ev: MessageWriter<SoundEvent>,
    mut health_ev: MessageWriter<ChangeHealthEvent>,
    mut q_ball: Query<&mut LinearVelocity, With<PinBall>>,
    q_enemy: Query<(&Enemy, &BallSlowDown, &Health), With<Enemy>>,
    effects: Res<ActiveEffects>,
    ig_time: Res<IngameTime>,
) {
    for CollisionWithBallEvent(id) in evr.read() {
        let Ok((_, slow_down, health)) = q_enemy.get(*id) else {
            continue;
        };
        log!("😵 Pinball hits enemy {:?}", *id);
        health_ev.write(ChangeHealthEvent::new(
            *id,
            crate::game::extra_field_effects::ball_damage(&effects, **ig_time, health.max()),
            None,
        ));
        if slow_down.0 < 1. {
            // ponytail: applies to all balls; per-ball attribution needs an event refactor
            for mut vel in q_ball.iter_mut() {
                **vel *= slow_down.0;
            }
        }
        sound_ev.write(SoundEvent::BallHitsEnemy);
    }
}

#[derive(Message)]
pub struct OnEnemyDespawnEvent(pub Entity);

fn on_health_empty_system(
    mut cmds: Commands,
    mut evr: MessageReader<HealthEmptyEvent>,
    mut despawn_ev: MessageWriter<OnEnemyDespawnEvent>,
    mut points_ev: MessageWriter<PointsEvent>,
    q_enemy: Query<(&Transform, Entity), With<Enemy>>,
) {
    for ev in evr.read() {
        if let Ok((tf, _)) = q_enemy.get(ev.0) {
            let pos = tf.translation;
            cmds.entity(ev.0).try_despawn();
            despawn_ev.write(OnEnemyDespawnEvent(ev.0));
            points_ev.write(PointsEvent::new(PointsKind::EnemyDied, pos));
        }
    }
}
