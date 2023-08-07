use crate::events::collision::PinballEnemyHitEvent;
use crate::events::tween_completed::ROAD_POINT_REACHED_EVENT_ID;
use crate::prelude::*;
use crate::road::points::ROAD_DISTS;
use crate::settings::GraphicsSettings;
use crate::tower::light::{ContactLight, LightOnCollision};
use crate::{road::points::ROAD_POINTS, GameState};
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseMethod, Tween};
use std::time::Duration;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RoadPointReachedEvent>().add_systems(
            Update,
            (set_next_road_point_system, pinball_hit_system).run_if(in_state(GameState::Ingame)),
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

pub fn spawn_enemy(
    parent: &mut ChildBuilder,
    assets: &PinballDefenseAssets,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    g_sett: &GraphicsSettings,
) {
    parent
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.05,
                    ..default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    perceptual_roughness: 0.,
                    metallic: 1.,
                    reflectance: 1.,
                    ..default()
                }),
                ..default()
            },
            Sensor,
            RigidBody::KinematicPositionBased,
            ColliderDebugColor(Color::RED),
            Collider::ball(0.06),
            Restitution {
                coefficient: 2.,
                combine_rule: CoefficientCombineRule::Multiply,
            },
            ActiveEvents::COLLISION_EVENTS,
            LightOnCollision,
            Enemy::new(1),
            Name::new("Enemy"),
            to_pos_animation(ROAD_POINTS[0], ROAD_POINTS[1], calc_walk_time(0)),
        ))
        .with_children(|parent| {
            parent.spawn((
                PointLightBundle {
                    transform: Transform::from_xyz(0., 0.005, 0.),
                    point_light: PointLight {
                        intensity: 0.,
                        color: Color::RED,
                        shadows_enabled: g_sett.is_shadows,
                        radius: 0.01,
                        range: 0.5,
                        ..default()
                    },
                    ..default()
                },
                ContactLight,
            ));

            crate::progress_bar::spawn(
                parent,
                assets,
                materials,
                parent.parent_entity(),
                Transform {
                    translation: Vec3::new(0.034, -0.007, 0.),
                    scale: Vec3::new(0.5, 1., 0.5),
                    ..default()
                },
                Color::ORANGE,
            );
        });
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

const WALK_SPEED: f32 = 0.4;

fn calc_walk_time(i: usize) -> f32 {
    ROAD_DISTS[i] / WALK_SPEED
}

#[derive(Event)]
pub struct RoadPointReachedEvent(pub Entity);

pub fn set_next_road_point_system(
    mut cmds: Commands,
    mut evr: EventReader<RoadPointReachedEvent>,
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

                log!(
                    "🍆 Next road point: {}",
                    ROAD_POINTS[enemy.i_next_road_point]
                );
            } else {
                cmds.entity(entity).despawn_recursive();
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
