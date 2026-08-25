use self::step::Step;
use self::walk::{
    RoadEndReachedEvent, WALK_SPEED, on_road_end_reached_system, recover_speed_system, walk_system,
};
use super::audio::SoundEvent;
use super::events::collision::GameLayer;
use super::health::{ChangeHealthEvent, Health, HealthEmptyEvent};
use super::level::{PointsEvent, PointsKind};
use super::{EventState, ui};
use crate::game::GameState;
use crate::game::ball::CollisionWithBallEvent;
use crate::game::world::QueryWorld;
use crate::generated::world_1::road_points::ROAD_POINTS;
use crate::prelude::*;
use bevy::math::primitives::Sphere;
use moonshine_save::prelude::Save;
use std::time::Duration;

mod step;
mod walk;

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
    speed: f32,
    current_speed: f32,
    wave: usize,
}

impl Enemy {
    pub fn new(wave: usize) -> Self {
        Self {
            step: Step::new(1),
            speed: WALK_SPEED,
            current_speed: WALK_SPEED,
            wave,
        }
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
pub struct SpawnEnemyEvent(pub usize);

fn on_spawn_system(
    mut cmds: Commands,
    mut evr: MessageReader<SpawnEnemyEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    q_pqw: QueryWorld,
) {
    for SpawnEnemyEvent(wave) in evr.read() {
        let mut enemy_id: Option<Entity> = None;
        let Ok(world) = q_pqw.single() else {
            warn!("[enemy spawn] no world");
            return;
        };
        cmds.entity(world).with_children(|spawner| {
            enemy_id = Some(spawner.spawn(enemy(&mut meshes, &mut mats, *wave)).id());
        });
        if let Some(enemy_id) = enemy_id {
            ui::progress_bar::spawn(&mut cmds, enemy_id, 1.);
        }
    }
}

#[derive(Component)]
pub struct LastDamager(pub Option<Entity>);

fn enemy_color(wave: usize) -> Color {
    let t = (wave as f32 / 100.).min(1.);
    let hue = 30. + (220. - 30.) * t;
    let lightness = 0.6 - 0.55 * t;
    Color::hsl(hue, 0.9, lightness)
}

fn enemy_view_bundle(
    meshes: &mut Assets<Mesh>,
    mats: &mut Assets<StandardMaterial>,
    wave: usize,
) -> impl Bundle {
    let color = enemy_color(wave);
    (
        Name::new("Enemy"),
        Mesh3d(meshes.add(Mesh::from(Sphere { radius: 0.03 }))),
        MeshMaterial3d(mats.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 1.,
            metallic: 0.,
            reflectance: 0.,
            ..default()
        })),
        Sensor,
        RigidBody::Kinematic,
        Collider::circle(0.03),
        CollisionEventsEnabled,
        DebugRender::default().with_collider_color(color),
        CollisionLayers::new(GameLayer::Enemy, [GameLayer::Ball, GameLayer::Tower]),
        Restitution {
            coefficient: 2.,
            combine_rule: CoefficientCombine::Multiply,
        },
    )
}

fn enemy(meshes: &mut Assets<Mesh>, mats: &mut Assets<StandardMaterial>, wave: usize) -> impl Bundle {
    (
        enemy_view_bundle(meshes, mats, wave),
        Enemy::new(wave),
        Health::new(100. * (1. + wave as f32 * 0.5)),
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
        cmds.entity(enemy_id)
            .insert(enemy_view_bundle(&mut meshes, &mut mats, enemy.wave));
        cmds.entity(world).add_child(enemy_id);
    }
}

fn on_pinball_hit_system(
    mut evr: MessageReader<CollisionWithBallEvent>,
    mut points_ev: MessageWriter<PointsEvent>,
    mut sound_ev: MessageWriter<SoundEvent>,
    mut health_ev: MessageWriter<ChangeHealthEvent>,
    q_enemy: Query<(&Transform, Entity), With<Enemy>>,
) {
    for CollisionWithBallEvent(id) in evr.read() {
        // flag == CollisionEventFlags::SENSOR &&
        if let Ok((tf, _)) = q_enemy.get(*id) {
            log!("😵 Pinball hits enemy {:?}", *id);
            health_ev.write(ChangeHealthEvent::new(*id, -100., None));
            points_ev.write(PointsEvent::new(PointsKind::BallEnemyHit, tf.translation));
            sound_ev.write(SoundEvent::BallHitsEnemy);
        }
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
