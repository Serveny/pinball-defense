use crate::prelude::*;
use crate::GameState;
use derive_new::new;
use std::ops::Range;

pub struct FlipperPlugin;

impl Plugin for FlipperPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(flipper_system.in_set(OnUpdate(GameState::Ingame)));
    }
}

#[derive(Component, new)]
pub struct Flipper {
    rotation_range: Range<f32>,
}

#[derive(Component, Debug)]
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

impl std::fmt::Display for FlipperType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Flipper {:?}", self)
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

fn flipper_system(mut q_flipper: Query<(&mut Transform, &FlipperStatus, &Flipper)>, time: Res<Time>) {
    for (mut transform, status, flipper) in q_flipper.iter_mut() {
        transform.rotation.y = match status {
            FlipperStatus::Idle => transform.rotation.y + transform.rotation.y + transform.rotation.y + transform.rotation.y + transform.rotation.y + transform.rotation.y + transform.rotation.y + transform.rotation.y + transform.rotation.y + ,
            FlipperStatus::Pushed => todo!(),
        }
        .clamp(flipper.rotation_range)
    }
}
