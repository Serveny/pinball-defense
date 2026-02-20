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
        app.add_message::<OnBallDespawnEvent>()
            .add_message::<CollisionWithBallEvent>()
            .add_systems(
                Update,
                (ball_reset_system, clamp_ball_speed_system).run_if(in_state(GameState::Ingame)),
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
    let radius = 0.005;
    cmds.spawn((
        Mesh3d(meshes.add(Mesh::from(Sphere {
            radius: radius * 4.,
            ..default()
        }))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GOLD.into(),
            perceptual_roughness: 0.,
            metallic: 1.,
            reflectance: 1.,
            ..default()
        })),
        Transform::from_translation(pos),
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
        Restitution::from(0.5),
        Friction::from(0.02),
        PinBall,
        Name::new("Ball"),
    ));
}

#[derive(Message)]
pub struct OnBallDespawnEvent;

const X_RANGE: Range<f32> = -1.3..1.3;
const Y_RANGE: Range<f32> = -0.72..0.72;
const HIT_Y_RANGE: Range<f32> = -0.2..0.12;
const MAX_BALL_SPEED: f32 = 16.;

fn ball_reset_system(
    mut cmds: Commands,
    mut evw: MessageWriter<OnBallDespawnEvent>,
    mut health_ev: MessageWriter<ChangeHealthEvent>,
    q_ball: Query<(Entity, &Transform), With<PinBall>>,
    q_life_bar: Query<Entity, With<LifeBar>>,
) {
    for (entity, transform) in q_ball.iter() {
        let ball_pos = transform.translation;
        if !X_RANGE.contains(&ball_pos.x) || !Y_RANGE.contains(&ball_pos.y) {
            if ball_pos.x > 1.2 && HIT_Y_RANGE.contains(&ball_pos.y) {
                if let Ok(lifebar_id) = q_life_bar.single() {
                    health_ev.write(ChangeHealthEvent::new(lifebar_id, -5., None));
                }
            }
            log!("🎱 Despawn ball");
            cmds.get_entity(entity).unwrap().despawn();
            evw.write(OnBallDespawnEvent);
        }
    }
}

fn clamp_ball_speed_system(mut q_ball: Query<&mut LinearVelocity, With<PinBall>>) {
    for mut velocity in q_ball.iter_mut() {
        let speed = velocity.length();
        if speed > MAX_BALL_SPEED {
            let scale = MAX_BALL_SPEED / speed;
            velocity.x *= scale;
            velocity.y *= scale;
        }
    }
}

fn on_ball_despawn_system(
    mut evr: MessageReader<OnBallDespawnEvent>,
    mut pm_status_ev: MessageWriter<PinballMenuEvent>,
    mut sound_ev: MessageWriter<SoundEvent>,
) {
    if evr.read().next().is_some() {
        pm_status_ev.write(PinballMenuEvent::Deactivate);
        sound_ev.write(SoundEvent::BallHitsEnd);
    }
}

#[derive(Message, Debug)]
pub struct CollisionWithBallEvent(pub Entity);

fn on_collision_with_ball_system(
    coll_ev: MessageReader<CollisionStart>,
    mut coll_with_ball_ev: MessageWriter<CollisionWithBallEvent>,
    mut points_ev: MessageWriter<PointsEvent>,
    q_ball: Query<Entity, With<PinBall>>,
) {
    for collidator_id in get_ball_collisions(coll_ev, q_ball) {
        coll_with_ball_ev.write(CollisionWithBallEvent(collidator_id));
        points_ev.write(PointsEvent::BallCollided);
    }
}

fn get_ball_collisions(
    mut evr: MessageReader<CollisionStart>,
    q_ball: Query<Entity, With<PinBall>>,
) -> Vec<Entity> {
    evr.read()
        .filter_map(|ev| match q_ball.contains(ev.collider1) {
            true => Some(ev.collider1),
            false => match q_ball.contains(ev.collider2) {
                true => Some(ev.collider2),
                false => None,
            },
        })
        .collect()
}

fn on_wall_collision_system(
    mut evr: MessageReader<CollisionWithBallEvent>,
    mut sound_ev: MessageWriter<SoundEvent>,
    q_wall: Query<Entity, With<WorldFrame>>,
) {
    for ev in evr.read() {
        if q_wall.contains(ev.0) {
            sound_ev.write(SoundEvent::BallHitsWall);
        }
    }
}
