use super::audio::SoundEvent;
use super::events::collision::GameLayer;
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
use std::ops::Range;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnBallDespawnEvent>()
            .add_event::<CollisionWithBallEvent>()
            .add_systems(
                Update,
                (ball_reset_system).run_if(in_state(GameState::Ingame)),
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
        RigidBody::Dynamic,
        SweptCcd::default(),
        SleepingDisabled::default(),
        Collider::circle(radius),
        DebugRender::collider(GOLD.into()),
        CollisionLayers::new(
            GameLayer::Ball,
            [GameLayer::Enemy, GameLayer::Tower, GameLayer::Map],
        ),
        Mass(0.081),
        Restitution::from(1.0),
        Friction::from(0.00),
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

#[derive(Event, Debug)]
pub struct CollisionWithBallEvent(pub Entity);

fn on_collision_with_ball_system(
    coll_ev: EventReader<CollisionStarted>,
    mut coll_with_ball_ev: EventWriter<CollisionWithBallEvent>,
    mut points_ev: EventWriter<PointsEvent>,
    q_ball: Query<Entity, With<PinBall>>,
) {
    for collidator_id in get_ball_collisions(coll_ev, q_ball) {
        coll_with_ball_ev.send(CollisionWithBallEvent(collidator_id));
        points_ev.send(PointsEvent::BallCollided);
    }
}

fn get_ball_collisions(
    mut evr: EventReader<CollisionStarted>,
    q_ball: Query<Entity, With<PinBall>>,
) -> Vec<Entity> {
    evr.read()
        .filter_map(|ev| match q_ball.contains(ev.0) {
            true => Some(ev.1),
            false => match q_ball.contains(ev.1) {
                true => Some(ev.0),
                false => None,
            },
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
