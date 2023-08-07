use crate::events::tween_completed::ROAD_POINT_REACHED_EVENT_ID;
use crate::prelude::*;
use crate::{road::points::ROAD_POINTS, GameState};
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseMethod, Tween};
use std::time::Duration;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RoadPointReachedEvent>().add_systems(
            Update,
            set_next_road_point_system.run_if(in_state(GameState::Ingame)),
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
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    parent.spawn((
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
        Enemy::new(1),
        Name::new("Enemy"),
        to_pos_animation(ROAD_POINTS[0], ROAD_POINTS[1]),
    ));
}

fn to_pos_animation(start: Vec3, end: Vec3) -> Animator<Transform> {
    Animator::new(
        Tween::new(
            EaseMethod::Linear,
            Duration::from_secs_f32(0.5),
            TransformPositionLens { start, end },
        )
        .with_completed_event(ROAD_POINT_REACHED_EVENT_ID),
    )
}

#[derive(Event)]
pub struct RoadPointReachedEvent(pub Entity);

pub fn set_next_road_point_system(
    mut cmds: Commands,
    mut evr: EventReader<RoadPointReachedEvent>,
    mut q_enemy: Query<&mut Enemy>,
) {
    for ev in evr.iter() {
        let entity = ev.0;
        if let Ok(mut enemy) = q_enemy.get_mut(entity) {
            if enemy.i_next_road_point < ROAD_POINTS.len() - 1 {
                cmds.entity(entity).insert(to_pos_animation(
                    ROAD_POINTS[enemy.i_next_road_point],
                    ROAD_POINTS[enemy.i_next_road_point + 1],
                ));
                enemy.i_next_road_point += 1;

                log!(
                    "🍆 Next road point: {}",
                    ROAD_POINTS[enemy.i_next_road_point]
                );
            }
        }
    }
}
