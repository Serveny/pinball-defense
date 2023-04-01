use crate::prelude::*;
use crate::GameState;
use std::ops::Range;

pub struct FlipperPlugin;

impl Plugin for FlipperPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(flipper_system.in_set(OnUpdate(GameState::Ingame)));
    }
}

#[derive(Component)]
pub struct Flipper {
    rotation_range: Range<f32>,
}

impl Flipper {
    pub fn new(min_degree: f32, max_degree: f32) -> Self {
        Self {
            rotation_range: f32::to_radians(min_degree)..f32::to_radians(max_degree),
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

impl FlipperStatus {
    fn value_degree(&self) -> f32 {
        match self {
            FlipperStatus::Idle => -10.,
            FlipperStatus::Pushed => 20.,
        }
    }
}

impl FlipperType {
    fn signum(&self) -> f32 {
        match self {
            FlipperType::Left => 1.,
            FlipperType::Right => -1.,
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
            Ccd::enabled(),
            ColliderDebugColor(Color::RED),
            Collider::from_bevy_mesh(
                meshes.get(&assets.flipper).expect("Failed to find mesh"),
                &ComputedColliderShape::TriMesh,
            )
            .unwrap(),
        ))
        .insert(flipper)
        .insert(Name::new(flipper_type.to_string()))
        .insert(flipper_type)
        .insert(FlipperStatus::Idle);
}

fn flipper_system(
    mut q_flipper: Query<(&mut Transform, &FlipperStatus, &Flipper, &FlipperType)>,
    time: Res<Time>,
) {
    for (mut transform, status, flipper, f_type) in q_flipper.iter_mut() {
        let mut new_rotation = transform.rotation;
        new_rotation *=
            Quat::from_rotation_y(f_type.signum() * status.value_degree() * time.delta_seconds());
        let rotation_y = new_rotation.to_axis_angle().1;
        //println!("{rotation_y} ({:?})", flipper.rotation_range);
        transform.rotation = Quat::from_rotation_y(
            rotation_y.clamp(flipper.rotation_range.start, flipper.rotation_range.end),
        );
    }
}
