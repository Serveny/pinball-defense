use self::step::Step;
use self::walk::{
    on_road_end_reached_system, recover_speed_system, walk_system, RoadEndReachedEvent, WALK_SPEED,
};
use super::audio::SoundEvent;
use super::health::{ChangeHealthEvent, Health, HealthEmptyEvent};
use super::level::PointsEvent;
use super::{ui, EventState};
use crate::game::ball::CollisionWithBallEvent;
use crate::game::events::collision::{ENEMY, INTERACT_WITH_BALL, INTERACT_WITH_ENEMY};
use crate::game::world::QueryWorld;
use crate::game::GameState;
use crate::generated::world_1::road_points::ROAD_POINTS;
use crate::prelude::*;
use bevy::color::palettes::css::RED;
use bevy::math::primitives::Sphere;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use std::time::Duration;

mod step;
mod walk;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemyEvent>()
            .add_event::<RoadEndReachedEvent>()
            .add_event::<OnEnemyDespawnEvent>()
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

#[derive(Component)]
pub struct Enemy {
    step: Step,
    speed: f32,
    current_speed: f32,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            step: Step::new(1),
            speed: WALK_SPEED,
            current_speed: WALK_SPEED,
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

#[derive(Event)]
pub struct SpawnEnemyEvent;

fn on_spawn_system(
    mut cmds: Commands,
    mut evr: EventReader<SpawnEnemyEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    q_pqw: QueryWorld,
) {
    for _ in evr.read() {
        let mut enemy_id: Option<Entity> = None;
        let Ok(world) = q_pqw.single() else {
            warn!("[enemy spawn] no world");
            return;
        };
        cmds.entity(world).with_children(|spawner| {
            enemy_id = Some(spawner.spawn(enemy(&mut meshes, &mut mats)).id());
        });
        if let Some(enemy_id) = enemy_id {
            ui::progress_bar::spawn(&mut cmds, enemy_id, 1.);
        }
    }
}

#[derive(Component)]
pub struct LastDamager(pub Option<Entity>);

fn enemy(meshes: &mut Assets<Mesh>, mats: &mut Assets<StandardMaterial>) -> impl Bundle {
    (
        Name::new("Enemy"),
        Enemy::new(),
        Health::new(100.),
        LastDamager(None),
        Mesh3d(meshes.add(Mesh::from(Sphere {
            radius: 0.03,
            ..default()
        }))),
        MeshMaterial3d(mats.add(StandardMaterial {
            base_color: RED.into(),
            perceptual_roughness: 0.,
            metallic: 1.,
            reflectance: 1.,
            ..default()
        })),
        Transform::from_translation(ROAD_POINTS[0]),
        Sensor,
        RigidBody::KinematicPositionBased,
        Collider::ball(0.03),
        ColliderDebugColor(RED.into()),
        CollisionGroups::new(ENEMY.union(INTERACT_WITH_BALL), INTERACT_WITH_ENEMY),
        Restitution {
            coefficient: 2.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
        ActiveEvents::COLLISION_EVENTS,
    )
}

fn on_pinball_hit_system(
    mut evr: EventReader<CollisionWithBallEvent>,
    mut points_ev: EventWriter<PointsEvent>,
    mut sound_ev: EventWriter<SoundEvent>,
    mut health_ev: EventWriter<ChangeHealthEvent>,
    q_enemy: Query<Entity, With<Enemy>>,
) {
    for CollisionWithBallEvent(id, flag) in evr.read() {
        if *flag == CollisionEventFlags::SENSOR && q_enemy.contains(*id) {
            log!("😵 Pinball hits enemy {:?}", *id);
            health_ev.write(ChangeHealthEvent::new(*id, -100., None));
            points_ev.write(PointsEvent::BallEnemyHit);
            sound_ev.write(SoundEvent::BallHitsEnemy);
        }
    }
}

#[derive(Event)]
pub struct OnEnemyDespawnEvent(pub Entity);

fn on_health_empty_system(
    mut cmds: Commands,
    mut evr: EventReader<HealthEmptyEvent>,
    mut despawn_ev: EventWriter<OnEnemyDespawnEvent>,
    mut points_ev: EventWriter<PointsEvent>,
    q_enemy: Query<Entity, With<Enemy>>,
) {
    for ev in evr.read() {
        if q_enemy.contains(ev.0) {
            cmds.entity(ev.0).despawn();
            despawn_ev.write(OnEnemyDespawnEvent(ev.0));
            points_ev.write(PointsEvent::EnemyDied);
        }
    }
}
