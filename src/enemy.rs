use crate::events::collision::PinballEnemyHitEvent;
use crate::player_life::LifeBar;
use crate::prelude::*;
use crate::progress_bar::ProgressBarCountUpEvent;
use crate::road::points::ROAD_DISTS;
use crate::tower::light::LightOnCollision;
use crate::{road::points::ROAD_POINTS, GameState};
use std::time::Duration;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemyEvent>()
            .add_event::<RoadEndReachedEvent>()
            .add_systems(
                Update,
                (
                    pinball_hit_system,
                    spawn_enemy_system,
                    walk_system,
                    on_road_end_reached_system,
                )
                    .run_if(in_state(GameState::Ingame)),
            );
    }
}

struct Step {
    pub i_road_point: usize,
    pub direction: Vec3,
    pub distance_to_walk: f32,
    pub distance_walked: f32,
}

impl Step {
    pub fn new(i_point: usize) -> Self {
        let dir = get_direction_to(i_point);
        Self {
            i_road_point: i_point,
            distance_to_walk: ROAD_DISTS[i_point - 1],
            distance_walked: 0.,
            direction: dir.normalize(),
        }
    }

    pub fn next(&self) -> Self {
        Self::new(self.i_road_point + 1)
    }
    pub fn walk(&mut self, current_pos: Vec3, distance: f32) -> Vec3 {
        self.distance_walked += distance;
        current_pos + self.direction * distance
    }

    pub fn start_pos(&self) -> Vec3 {
        ROAD_POINTS[self.i_road_point - 1]
    }

    pub fn is_reached_point(&self) -> bool {
        self.distance_walked >= self.distance_to_walk
    }

    pub fn is_reached_road_end(&self) -> bool {
        self.i_road_point == ROAD_POINTS.len() - 1 && self.is_reached_point()
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
        Enemy::new(),
        Name::new("Enemy"),
    ));
}

const WALK_SPEED: f32 = 0.2;

fn get_direction_to(i: usize) -> Vec3 {
    ROAD_POINTS[i] - ROAD_POINTS[i - 1]
}

fn pinball_hit_system(mut cmds: Commands, mut evr: EventReader<PinballEnemyHitEvent>) {
    for ev in evr.iter() {
        log!("😵 Pinball hits enemy {:?}", ev.0);
        cmds.entity(ev.0).despawn_recursive();
    }
}

fn walk_system(
    mut cmds: Commands,
    mut q_enemy: Query<(Entity, &mut Transform, &mut Enemy)>,
    mut end_reached_ev: EventWriter<RoadEndReachedEvent>,
    time: Res<Time>,
) {
    for (enemy_id, mut trans, mut enemy) in q_enemy.iter_mut() {
        match enemy.walk(trans.translation, time.delta()) {
            Some(pos) => trans.translation = pos,
            None => {
                // Reminder: If you need infos about the enemy, overgive only infos, not enemy id
                cmds.entity(enemy_id).despawn_recursive();
                end_reached_ev.send(RoadEndReachedEvent);
            }
        };
    }
}

#[derive(Event)]
struct RoadEndReachedEvent;

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
