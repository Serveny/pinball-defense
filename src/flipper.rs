use crate::prelude::*;
use crate::GameState;

pub struct FlipperPlugin;

impl Plugin for FlipperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, flipper_system.run_if(in_state(GameState::Ingame)));
    }
}

#[derive(Component)]
pub struct Flipper {
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
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &mut PinballDefenseAssets,
) {
    spawn(
        FlipperType::Right,
        transform,
        parent,
        materials,
        &assets.flipper_right,
    );
}

pub fn spawn_left(
    transform: Transform,
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &mut PinballDefenseAssets,
) {
    spawn(
        FlipperType::Left,
        transform,
        parent,
        materials,
        &assets.flipper_left,
    );
}

#[derive(Component)]
pub struct FlipperCollider;

fn spawn(
    flipper_type: FlipperType,
    transform: Transform,
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    flipper_mesh: &Handle<Mesh>,
) {
    let sig = flipper_type.signum();
    parent
        .spawn((
            PbrBundle {
                mesh: flipper_mesh.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::ORANGE,
                    perceptual_roughness: 0.5,
                    metallic: 0.5,
                    reflectance: 0.5,
                    ..default()
                }),
                transform,
                ..default()
            },
            Flipper::new(),
            Name::new(flipper_type.to_string()),
            FlipperStatus::Idle,
            flipper_type,
        ))
        .with_children(|parent| {
            parent.spawn((
                TransformBundle::from(Transform {
                    translation: Vec3::new(0.008, 0.035, sig * 0.115),
                    rotation: Quat::from_rotation_y(sig * 0.075),
                    ..default()
                }),
                RigidBody::KinematicPositionBased,
                Collider::cuboid(0.03, 0.035, 0.12),
                ActiveEvents::COLLISION_EVENTS,
                Restitution {
                    coefficient: 0.1,
                    combine_rule: CoefficientCombineRule::Multiply,
                },
                FlipperCollider,
            ));
        });
}

fn flipper_system(
    mut q_flipper: Query<(&mut Transform, &FlipperStatus, &mut Flipper, &FlipperType)>,
    time: Res<Time>,
) {
    let time = time.delta_seconds();
    for (mut transform, status, mut flipper, f_type) in q_flipper.iter_mut() {
        let mut change_angle = f_type.signum();
        match status {
            FlipperStatus::Idle => {
                flipper.acceleration_factor = 1.;
                change_angle *= 8. * time;
            }
            FlipperStatus::Pushed => {
                change_angle *= -time * flipper.acceleration_factor;
                flipper.acceleration_factor += time * 480.;
            }
        }
        let new_angle = flipper.curr_angle + change_angle;
        let new_clamped_angle = new_angle.clamp(-0.4, 0.4);
        let pivot_rotation = Quat::from_rotation_y(new_clamped_angle - flipper.curr_angle);
        let translation = transform.translation;
        transform.rotate_around(translation, pivot_rotation);
        flipper.curr_angle = new_clamped_angle;
    }
}
