use super::audio::SoundEvent;
use super::ball::CollisionWithBallEvent;
use super::events::collision::GameLayer;
use super::level::PointsEvent;
use super::{EventState, GameState};
use crate::prelude::*;
use std::f32::consts::PI;

pub struct FlipperPlugin;

impl Plugin for FlipperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (flipper_system, sound_system).run_if(in_state(GameState::Ingame)),
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

#[derive(Component, Debug, PartialEq, Eq)]
pub enum FlipperType {
    Left = 1,
    Right = -1,
}

#[derive(Component, Debug, Default)]
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
            rotation: Quat::from_rotation_y(-PI / 2. * 0.85),
            ..default()
        },
        RigidBody::Kinematic,
        Collider::rectangle(0.06, 0.24),
        Restitution {
            coefficient: 0.1,
            combine_rule: CoefficientCombine::Multiply,
        },
        CollisionLayers::new(GameLayer::Map, GameLayer::Ball),
        FlipperCollider,
    )
}

fn flipper_system(
    mut q_flipper: Query<(&mut Transform, &FlipperStatus, &mut Flipper, &FlipperType)>,
    time: Res<Time>,
) {
    let time = time.delta_secs();
    for (mut transform, status, mut flipper, f_type) in q_flipper.iter_mut() {
        let mut change_angle = f_type.signum();
        match status {
            FlipperStatus::Idle => {
                flipper.acceleration_factor = 1.;
                change_angle *= 8. * time;
            }
            FlipperStatus::Pushed => {
                change_angle *= -time * flipper.acceleration_factor;
                flipper.acceleration_factor += time * 256.;
            }
        }
        let new_angle = flipper.curr_angle + change_angle;
        let new_clamped_angle = new_angle.clamp(-0.4, 0.4);
        let pivot_rotation = Quat::from_rotation_z(new_clamped_angle - flipper.curr_angle);
        let translation = transform.translation;
        transform.rotate_around(translation, pivot_rotation);
        flipper.curr_angle = new_clamped_angle;
    }
}

fn sound_system(
    mut sound_ev: EventWriter<SoundEvent>,
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
    mut points_ev: EventWriter<PointsEvent>,
    mut evr: EventReader<CollisionWithBallEvent>,
    q_flipper: Query<Entity, With<FlipperCollider>>,
) {
    for ev in evr.read() {
        if q_flipper.contains(ev.0) {
            points_ev.write(PointsEvent::FlipperHit);
        }
    }
}
