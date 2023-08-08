use crate::events::collision::PinballEnemyHitEvent;
use crate::events::tween_completed::ROAD_POINT_REACHED_EVENT_ID;
use crate::player_life::LifeBar;
use crate::prelude::*;
use crate::progress_bar::ProgressBarCountUpEvent;
use crate::road::points::ROAD_DISTS;
use crate::tower::light::LightOnCollision;
use crate::{road::points::ROAD_POINTS, GameState};
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseMethod, Tween};
use std::time::Duration;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RoadPointReachedEvent>()
            .add_event::<RoadEndReachedEvent>()
            .add_event::<SpawnEnemyEvent>()
            .add_systems(
                Update,
                (
                    set_next_road_point_system,
                    pinball_hit_system,
                    spawn_enemy_system,
                    on_road_end_reached_system,
                )
                    .run_if(in_state(GameState::Ingame)),
            );
    }
}

#[derive(Component)]
pub struct Enemy {
    pub i_next_road_point: usize,
}

impl Enemy {
    pub fn new(i_next_road_point: usize) -> Self {
        Self { i_next_road_point }
    }
}

#[derive(Event)]
pub struct SpawnEnemyEvent;

fn spawn_enemy_system(
    mut cmds: Commands,
    mut evr: EventReader<SpawnEnemyEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    q_pqw: QueryWorld,
) {
    for _ in evr.iter() {
        cmds.entity(q_pqw.single()).with_children(|parent| {
            spawn_enemy(parent, &mut meshes, &mut mats);
        });
    }
}

fn spawn_enemy(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    parent.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.03,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                perceptual_roughness: 0.,
                metallic: 1.,
                reflectance: 1.,
                ..default()
            }),
            transform: Transform::from_translation(ROAD_POINTS[0]),
            ..default()
        },
        Sensor,
        RigidBody::KinematicPositionBased,
        ColliderDebugColor(Color::RED),
        Collider::ball(0.03),
        Restitution {
            coefficient: 2.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
        ActiveEvents::COLLISION_EVENTS,
        LightOnCollision,
        Enemy::new(1),
        Name::new("Enemy"),
        to_pos_animation(ROAD_POINTS[0], ROAD_POINTS[1], calc_walk_time(0)),
    ));
}

fn to_pos_animation(start: Vec3, end: Vec3, secs: f32) -> Animator<Transform> {
    Animator::new(
        Tween::new(
            EaseMethod::Linear,
            Duration::from_secs_f32(secs),
            TransformPositionLens { start, end },
        )
        .with_completed_event(ROAD_POINT_REACHED_EVENT_ID),
    )
}

const WALK_SPEED: f32 = 0.2;

fn calc_walk_time(i: usize) -> f32 {
    ROAD_DISTS[i] / WALK_SPEED
}

#[derive(Event)]
pub struct RoadPointReachedEvent(pub Entity);

fn set_next_road_point_system(
    mut cmds: Commands,
    mut evr: EventReader<RoadPointReachedEvent>,
    mut road_end_ev: EventWriter<RoadEndReachedEvent>,
    mut q_enemy: Query<(Entity, &mut Enemy)>,
) {
    for ev in evr.iter() {
        let entity = ev.0;
        if let Ok((entity, mut enemy)) = q_enemy.get_mut(entity) {
            if enemy.i_next_road_point < ROAD_POINTS.len() - 1 {
                cmds.entity(entity).insert(to_pos_animation(
                    ROAD_POINTS[enemy.i_next_road_point],
                    ROAD_POINTS[enemy.i_next_road_point + 1],
                    calc_walk_time(enemy.i_next_road_point),
                ));
                enemy.i_next_road_point += 1;
            } else {
                cmds.entity(entity).despawn_recursive();
                road_end_ev.send(RoadEndReachedEvent);
            }
        }
    }
}

fn pinball_hit_system(mut cmds: Commands, mut evr: EventReader<PinballEnemyHitEvent>) {
    for ev in evr.iter() {
        log!("😵 Pinball hits enemy {:?}", ev.0);
        cmds.entity(ev.0).despawn_recursive();
    }
}

#[derive(Event)]
pub struct RoadEndReachedEvent;

fn on_road_end_reached_system(
    mut evr: EventReader<RoadEndReachedEvent>,
    mut progress_ev: EventWriter<ProgressBarCountUpEvent>,
    q_life_bar: Query<Entity, With<LifeBar>>,
) {
    for _ in evr.iter() {
        log!("🔚 Enemy reached road end");
        progress_ev.send(ProgressBarCountUpEvent(q_life_bar.single(), -0.1));
    }
}
