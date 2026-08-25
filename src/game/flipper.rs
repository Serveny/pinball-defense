use super::audio::SoundEvent;
use super::ball::CollisionWithBallEvent;
use super::events::collision::GameLayer;
use super::level::{PointsEvent, PointsKind};
use super::{EventState, GameState};
use crate::prelude::*;

pub struct FlipperPlugin;

impl Plugin for FlipperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sound_system.run_if(in_state(GameState::Ingame)))
            .add_systems(
                FixedUpdate,
                flipper_system.run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (on_collision_with_ball_system).run_if(in_state(EventState::Active)),
            );
    }
}

#[derive(Component)]
struct Flipper {
    curr_angle: f32,
    acceleration_factor: f32,
}

impl Flipper {
    pub fn new() -> Self {
        Self {
            curr_angle: 0.,
            acceleration_factor: 1.,
        }
    }
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
pub enum FlipperType {
    Left = 1,
    Right = -1,
}

#[derive(Component, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum FlipperStatus {
    #[default]
    Idle,
    Pushed,
}

impl FlipperStatus {
    pub fn by_value(val: f32) -> FlipperStatus {
        match val < 0.5 {
            true => FlipperStatus::Idle,
            false => FlipperStatus::Pushed,
        }
    }
}

impl FlipperType {
    fn signum(&self) -> f32 {
        match self {
            FlipperType::Left => -1.,
            FlipperType::Right => 1.,
        }
    }
}

impl std::fmt::Display for FlipperType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Flipper {self:?}")
    }
}

pub fn spawn_right(
    transform: Transform,
    spawner: &mut ChildSpawnerCommands,
    assets: &PinballDefenseGltfAssets,
) {
    spawn(FlipperType::Right, transform, spawner, assets);
}

pub fn spawn_left(
    transform: Transform,
    spawner: &mut ChildSpawnerCommands,
    assets: &PinballDefenseGltfAssets,
) {
    spawn(FlipperType::Left, transform, spawner, assets);
}

#[derive(Component)]
pub struct FlipperCollider;

fn spawn(
    flipper_type: FlipperType,
    transform: Transform,
    spawner: &mut ChildSpawnerCommands,
    assets: &PinballDefenseGltfAssets,
) {
    let sig = flipper_type.signum();
    spawner
        .spawn(flipper(flipper_type, assets, transform))
        .with_children(|spawner| {
            spawner.spawn(collider(sig));
        });
}

fn flipper(
    flipper_type: FlipperType,
    assets: &PinballDefenseGltfAssets,
    transform: Transform,
) -> impl Bundle {
    (
        Mesh3d(match flipper_type {
            FlipperType::Left => assets.flipper_left.clone(),
            FlipperType::Right => assets.flipper_right.clone(),
        }),
        MeshMaterial3d(assets.flipper_material.clone()),
        transform,
        RigidBody::Kinematic,
        AngularVelocity(0.),
        CenterOfMass::new(0., 0.),
        NoAutoCenterOfMass,
        Flipper::new(),
        Name::new(flipper_type.to_string()),
        FlipperStatus::Idle,
        flipper_type,
    )
}

fn collider(sig: f32) -> impl Bundle {
    (
        Transform {
            translation: Vec3::new(0.008, sig * -0.115, 0.035),
            ..default()
        },
        Collider::rectangle(0.06, 0.24),
        Restitution {
            coefficient: 0.4,
            combine_rule: CoefficientCombine::Multiply,
        },
        CollisionLayers::new(GameLayer::Map, GameLayer::Ball),
        FlipperCollider,
    )
}

fn flipper_system(
    mut q_flipper: Query<(
        &FlipperStatus,
        &mut Flipper,
        &FlipperType,
        &Rotation,
        &mut AngularVelocity,
    )>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    const MAX_ANGLE: f32 = 0.4;
    for (status, mut flipper, f_type, rotation, mut ang_vel) in q_flipper.iter_mut() {
        let sig = f_type.signum();
        let curr = rotation.as_radians();
        let desired = match status {
            FlipperStatus::Idle => {
                flipper.acceleration_factor = 1.;
                sig * 8.
            }
            FlipperStatus::Pushed => {
                let v = -sig * flipper.acceleration_factor;
                flipper.acceleration_factor += dt * 256.;
                v
            }
        };
        let projected = curr + desired * dt;
        let clamped = projected.clamp(-MAX_ANGLE, MAX_ANGLE);
        ang_vel.0 = if dt > 0. { (clamped - curr) / dt } else { 0. };
        flipper.curr_angle = clamped;
    }
}

fn sound_system(
    mut sound_ev: MessageWriter<SoundEvent>,
    q_flipper: Query<&FlipperStatus, Changed<FlipperStatus>>,
) {
    for status in q_flipper.iter() {
        match status {
            FlipperStatus::Idle => sound_ev.write(SoundEvent::FlipperRelease),
            FlipperStatus::Pushed => sound_ev.write(SoundEvent::FlipperPress),
        };
    }
}

fn on_collision_with_ball_system(
    mut points_ev: MessageWriter<PointsEvent>,
    mut evr: MessageReader<CollisionWithBallEvent>,
    q_flipper: Query<(&Transform, Entity), With<FlipperCollider>>,
) {
    for ev in evr.read() {
        if let Ok((tf, _)) = q_flipper.get(ev.0) {
            points_ev.write(PointsEvent::new(PointsKind::FlipperHit, tf.translation));
        }
    }
}
