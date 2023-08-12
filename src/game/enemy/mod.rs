use self::step::Step;
use self::walk::{road_end_reached_system, walk_system, RoadEndReachedEvent, WALK_SPEED};
use crate::game::ball::CollisionWithBallEvent;
use crate::game::events::collision::{ENEMY, INTERACT_WITH_BALL, INTERACT_WITH_ENEMY};
use crate::game::progress_bar;
use crate::game::road::points::ROAD_POINTS;
use crate::game::world::QueryWorld;
use crate::game::GameState;
use crate::prelude::*;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;
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
                (
                    pinball_hit_system,
                    spawn_enemy_system,
                    walk_system,
                    road_end_reached_system,
                )
                    .run_if(in_state(GameState::Ingame)),
            );
    }
}

#[derive(Component)]
pub struct Enemy {
    step: Step,
    speed: f32,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            step: Step::new(1),
            speed: WALK_SPEED,
        }
    }

    pub fn walk(&mut self, current_pos: Vec3, dur: Duration) -> Option<Vec3> {
        let distance = dur.as_secs_f32() * self.speed;
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
}

#[derive(Event)]
pub struct SpawnEnemyEvent;

fn spawn_enemy_system(
    mut cmds: Commands,
    mut evr: EventReader<SpawnEnemyEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    assets: Res<PinballDefenseAssets>,
    q_pqw: QueryWorld,
) {
    for _ in evr.iter() {
        cmds.entity(q_pqw.single()).with_children(|parent| {
            spawn_enemy(parent, &assets, &mut meshes, &mut mats);
        });
    }
}

fn spawn_enemy(
    parent: &mut ChildBuilder,
    assets: &PinballDefenseAssets,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    parent
        .spawn((
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
            Collider::ball(0.03),
            ColliderDebugColor(Color::RED),
            CollisionGroups::new(ENEMY.union(INTERACT_WITH_BALL), INTERACT_WITH_ENEMY),
            Restitution {
                coefficient: 2.,
                combine_rule: CoefficientCombineRule::Multiply,
            },
            ActiveEvents::COLLISION_EVENTS,
            Enemy::new(),
            Name::new("Enemy"),
        ))
        .with_children(|parent| {
            progress_bar::spawn(
                parent,
                assets,
                materials,
                parent.parent_entity(),
                Transform {
                    translation: Vec3::new(0., 0.04, 0.),
                    rotation: Quat::from_rotation_z(f32::to_radians(90.)),
                    scale: Vec3::new(0.5, 1., 0.5),
                },
                Color::ORANGE_RED,
                1.,
            )
        });
}

fn pinball_hit_system(
    mut cmds: Commands,
    mut despawn_ev: EventWriter<OnEnemyDespawnEvent>,
    mut ball_coll_ev: EventReader<CollisionWithBallEvent>,
    q_enemy: Query<With<Enemy>>,
) {
    for CollisionWithBallEvent(id, flag) in ball_coll_ev.iter() {
        if *flag == CollisionEventFlags::SENSOR && q_enemy.contains(*id) {
            log!("😵 Pinball hits enemy {:?}", *id);
            cmds.entity(*id).despawn_recursive();
            despawn_ev.send(OnEnemyDespawnEvent(*id));
        }
    }
}

#[derive(Event)]
pub struct OnEnemyDespawnEvent(pub Entity);
