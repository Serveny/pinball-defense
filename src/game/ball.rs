use super::EventState;
use super::GameState;
use super::audio::SoundEvent;
use super::controls::KeyboardControls;
use super::enemy::Enemy;
use super::events::collision::GameLayer;
use super::health::ChangeHealthEvent;
use super::level::{BallCollisionPoints, PointsEvent};
use super::pinball_menu::PinballMenuEvent;
use super::player_life::LifeBar;
use super::world::WorldFrame;
use crate::prelude::*;
use bevy::color::palettes::css::GOLD;
use bevy::math::primitives::Sphere;
use bevy::platform::collections::HashSet;
use moonshine_save::prelude::Save;
use std::ops::Range;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<OnBallDespawnEvent>()
            .add_message::<CollisionWithBallEvent>()
            .add_message::<NudgeEvent>()
            .register_type::<PinBall>()
            .add_systems(
                Update,
                (
                    reattach_ball_system,
                    ball_reset_system,
                    clamp_ball_speed_system,
                    nudge_system,
                    enemy_ball_overlap_system,
                )
                    .run_if(in_state(GameState::Ingame)),
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

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Save)]
pub struct PinBall;

pub fn spawn(
    cmds: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
) -> Entity {
    cmds
        .spawn((
            ball_view_bundle(meshes, materials),
            Transform::from_translation(pos),
            PinBall,
            Name::new("Ball"),
        ))
        .id()
}

fn ball_view_bundle(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> impl Bundle {
    let radius = 0.02;
    (
        Mesh3d(meshes.add(Mesh::from(Sphere { radius }))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GOLD.into(),
            perceptual_roughness: 0.,
            metallic: 1.,
            reflectance: 1.,
            ..default()
        })),
        RigidBody::Dynamic,
        SweptCcd::LINEAR.include_dynamic(false),
        MaxLinearSpeed(MAX_BALL_SPEED),
        SleepingDisabled,
        Collider::circle(radius),
        CollisionEventsEnabled,
        DebugRender::collider(GOLD.into()),
        CollisionLayers::new(GameLayer::Ball, [GameLayer::Tower, GameLayer::Map]),
        Mass(0.081),
        Restitution::from(0.65),
        Friction::from(0.01),
    )
}

#[derive(Message)]
pub struct OnBallDespawnEvent;

const X_RANGE: Range<f32> = -1.3..1.3;
const Y_RANGE: Range<f32> = -0.72..0.72;
const HIT_Y_RANGE: Range<f32> = -0.2..0.12;
const MAX_BALL_SPEED: f32 = 10.;

fn reattach_ball_system(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_ball: Query<Entity, (With<PinBall>, Without<Collider>)>,
) {
    for ball_id in q_ball.iter() {
        cmds.entity(ball_id)
            .insert(ball_view_bundle(&mut meshes, &mut materials));
    }
}

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
            if ball_pos.x > 1.2
                && HIT_Y_RANGE.contains(&ball_pos.y)
                && let Ok(lifebar_id) = q_life_bar.single()
            {
                health_ev.write(ChangeHealthEvent::new(lifebar_id, -5., None));
            }
            log!("🎱 Despawn ball");
            cmds.entity(entity).try_despawn();
            evw.write(OnBallDespawnEvent);
        }
    }
}

pub(crate) fn clamp_ball_speed_system(mut q_ball: Query<&mut LinearVelocity, With<PinBall>>) {
    for mut velocity in q_ball.iter_mut() {
        let speed = velocity.length();
        if speed > MAX_BALL_SPEED {
            let scale = MAX_BALL_SPEED / speed;
            velocity.x *= scale;
            velocity.y *= scale;
        }
    }
}

fn nudge_system(
    key: Res<ButtonInput<KeyCode>>,
    controls: Res<KeyboardControls>,
    mut q_ball: Query<Forces, With<PinBall>>,
    mut nudge_ev: MessageWriter<NudgeEvent>,
) {
    if key.just_pressed(controls.nudge) {
        for mut forces in q_ball.iter_mut() {
            forces.apply_linear_impulse(Vec2::new(0., 0.1));
        }
        nudge_ev.write(NudgeEvent);
    }
}

#[derive(Message)]
pub struct NudgeEvent;

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
    q_ball: Query<(Entity, &Transform), With<PinBall>>,
    q_frame: Query<Entity, With<WorldFrame>>,
    q_points: Query<&BallCollisionPoints>,
) {
    let Ok((_, ball_tf)) = q_ball.single() else {
        return;
    };
    for collidator_id in get_ball_collisions(coll_ev, q_ball) {
        coll_with_ball_ev.write(CollisionWithBallEvent(collidator_id));
        if !q_frame.contains(collidator_id)
            && let Ok(points) = q_points.get(collidator_id)
        {
            points_ev.write(PointsEvent::with_points(points.0, ball_tf.translation));
        }
    }
}

fn get_ball_collisions(
    mut evr: MessageReader<CollisionStart>,
    q_ball: Query<(Entity, &Transform), With<PinBall>>,
) -> Vec<Entity> {
    evr.read()
        .filter_map(|ev| {
            if q_ball.contains(ev.collider1) {
                Some(ev.collider2)
            } else if q_ball.contains(ev.collider2) {
                Some(ev.collider1)
            } else {
                None
            }
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

const ENEMY_OVERLAP_RADIUS: f32 = 0.03;

fn enemy_ball_overlap_system(
    mut prev_overlapping: Local<HashSet<Entity>>,
    q_ball: Query<&Transform, With<PinBall>>,
    q_enemy: Query<(&Transform, &Collider, Entity), With<Enemy>>,
    mut coll_with_ball_ev: MessageWriter<CollisionWithBallEvent>,
) {
    let Ok(ball_pos) = q_ball.single() else {
        return;
    };
    let mut now_overlapping = HashSet::new();
    for (enemy_pos, collider, enemy_id) in q_enemy.iter() {
        let radius = collider
            .shape_scaled()
            .as_ball()
            .map_or(0.03, |ball| ball.radius);
        if ball_pos
            .translation
            .xy()
            .distance_squared(enemy_pos.translation.xy())
            <= (radius + ENEMY_OVERLAP_RADIUS).powi(2)
        {
            now_overlapping.insert(enemy_id);
            if !prev_overlapping.contains(&enemy_id) {
                coll_with_ball_ev.write(CollisionWithBallEvent(enemy_id));
            }
        }
    }
    *prev_overlapping = now_overlapping;
}
