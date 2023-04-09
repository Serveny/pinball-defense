use crate::prelude::*;
use crate::GameState;

pub struct FlipperPlugin;

impl Plugin for FlipperPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(flipper_system.in_set(OnUpdate(GameState::Ingame)));
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
    Left,
    Right,
}

#[derive(Component, Debug, Default)]
pub enum FlipperStatus {
    #[default]
    Idle,
    Pushed,
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

pub fn spawn_flipper(
    flipper: Flipper,
    flipper_type: FlipperType,
    transform: Transform,
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    assets: &mut PinballDefenseAssets,
) {
    parent
        .spawn((
            PbrBundle {
                mesh: assets.flipper.clone(),
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
            //Ccd::enabled(),
            ColliderDebugColor(Color::NONE),
            Collider::from_bevy_mesh(
                meshes.get(&assets.flipper).expect("Failed to find mesh"),
                &ComputedColliderShape::TriMesh,
            )
            .unwrap(),
            RigidBody::KinematicPositionBased,
        ))
        .insert(flipper)
        .insert(Name::new(flipper_type.to_string()))
        .insert(flipper_type)
        .insert(FlipperStatus::Idle);
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
                flipper.acceleration_factor += time * 500.;
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
