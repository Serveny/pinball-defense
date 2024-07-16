use super::audio::SoundEvent;
use super::events::collision::BALL;
use super::events::collision::INTERACT_WITH_BALL;
use super::events::collision::INTERACT_WITH_ENEMY;
use super::health::ChangeHealthEvent;
use super::level::PointsEvent;
use super::pinball_menu::PinballMenuEvent;
use super::player_life::LifeBar;
use super::world::WorldFrame;
use super::EventState;
use super::GameState;
use crate::prelude::*;
use bevy::color::palettes::css::GOLD;
use bevy::math::primitives::Sphere;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use std::ops::Range;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnBallDespawnEvent>()
            .add_event::<CollisionWithBallEvent>()
            .add_systems(
                Update,
                (ball_reset_system, max_speed_system).run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (
                    on_ball_despawn_system,
                    on_collision_with_ball_system,
                    on_wall_collision_system,
                )
                    .run_if(in_state(EventState::Active)),
            );
    }
}

#[derive(Component)]
pub struct PinBall;

pub fn spawn(
    cmds: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
) {
    let radius = 0.02;
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere {
                radius,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: GOLD.into(),
                perceptual_roughness: 0.,
                metallic: 1.,
                reflectance: 1.,
                ..default()
            }),
            transform: Transform::from_translation(pos),
            ..default()
        },
        Ccd::enabled(),
        RigidBody::Dynamic,
        Collider::ball(radius),
        ColliderDebugColor(GOLD.into()),
        CollisionGroups::new(
            BALL.union(INTERACT_WITH_ENEMY).union(INTERACT_WITH_BALL),
            INTERACT_WITH_BALL,
        ),
        Sleeping::disabled(),
        ColliderMassProperties::Mass(0.081),
        Restitution::coefficient(0.5),
        Friction::coefficient(0.01),
        Velocity::default(),
        PinBall,
        Name::new("Ball"),
    ));
}

#[derive(Event)]
pub struct OnBallDespawnEvent;

const X_RANGE: Range<f32> = -1.3..1.3;
const Y_RANGE: Range<f32> = -0.72..0.72;
const HIT_Y_RANGE: Range<f32> = -0.2..0.12;

fn ball_reset_system(
    mut cmds: Commands,
    mut evw: EventWriter<OnBallDespawnEvent>,
    mut health_ev: EventWriter<ChangeHealthEvent>,
    q_ball: Query<(Entity, &Transform), With<PinBall>>,
    q_life_bar: Query<Entity, With<LifeBar>>,
) {
    for (entity, transform) in q_ball.iter() {
        let ball_pos = transform.translation;
        if !X_RANGE.contains(&ball_pos.x) || !Y_RANGE.contains(&ball_pos.y) {
            if ball_pos.x > 1.2 && HIT_Y_RANGE.contains(&ball_pos.y) {
                health_ev.send(ChangeHealthEvent::new(q_life_bar.single(), -5., None));
            }
            log!("🎱 Despawn ball");
            cmds.get_entity(entity).unwrap().despawn_recursive();
            evw.send(OnBallDespawnEvent);
        }
    }
}

fn on_ball_despawn_system(
    mut evr: EventReader<OnBallDespawnEvent>,
    mut pm_status_ev: EventWriter<PinballMenuEvent>,
    mut sound_ev: EventWriter<SoundEvent>,
) {
    if evr.read().next().is_some() {
        pm_status_ev.send(PinballMenuEvent::Deactivate);
        sound_ev.send(SoundEvent::BallHitsEnd);
    }
}

// Prevent clipping of ball through objects
const MAX_SQUARED_SPEED: f32 = 40.;
const MAX_SQUARED_ANGLE_SPEED: f32 = 20_000.;

fn max_speed_system(mut q_ball: Query<&mut Velocity, With<PinBall>>) {
    q_ball.iter_mut().for_each(limit_velocity);
}

pub fn limit_velocity(mut velocity: Mut<Velocity>) {
    let length = velocity.linvel.length_squared();
    if length > MAX_SQUARED_SPEED {
        velocity.linvel *= MAX_SQUARED_SPEED / length;

        log!("🥨 Limit velocity from {} to {}", length, velocity.linvel);

        // If we reduce speed, maybe we should reduce turn speed too, idk
        let length = velocity.angvel;
        if length > MAX_SQUARED_ANGLE_SPEED {
            velocity.angvel *= MAX_SQUARED_ANGLE_SPEED / length;

            log!("💫 Limit turn speed from {} to {}", length, velocity.angvel);
        }
    }
}

#[derive(Event, Debug)]
pub struct CollisionWithBallEvent(pub Entity, pub CollisionEventFlags);

impl CollisionWithBallEvent {
    pub fn new(ev: (Entity, CollisionEventFlags)) -> Self {
        Self(ev.0, ev.1)
    }
}

fn on_collision_with_ball_system(
    coll_ev: EventReader<CollisionEvent>,
    mut coll_with_ball_ev: EventWriter<CollisionWithBallEvent>,
    mut points_ev: EventWriter<PointsEvent>,
    q_ball: Query<Entity, With<PinBall>>,
) {
    for ev in get_ball_collisions_start_only(coll_ev, q_ball) {
        coll_with_ball_ev.send(CollisionWithBallEvent::new(ev));
        points_ev.send(PointsEvent::BallCollided);
    }
}

fn get_ball_collisions_start_only(
    mut evr: EventReader<CollisionEvent>,
    q_ball: Query<Entity, With<PinBall>>,
) -> Vec<(Entity, CollisionEventFlags)> {
    evr.read()
        .filter_map(|ev| match ev {
            CollisionEvent::Started(id_1, id_2, flag) => match q_ball.contains(*id_1) {
                true => Some((*id_2, *flag)),
                false => match q_ball.contains(*id_2) {
                    true => Some((*id_1, *flag)),
                    false => None,
                },
            },
            CollisionEvent::Stopped(_, _, _) => None,
        })
        .collect()
}

fn on_wall_collision_system(
    mut evr: EventReader<CollisionWithBallEvent>,
    mut sound_ev: EventWriter<SoundEvent>,
    q_wall: Query<Entity, With<WorldFrame>>,
) {
    for ev in evr.read() {
        if q_wall.contains(ev.0) {
            sound_ev.send(SoundEvent::BallHitsWall);
        }
    }
}
