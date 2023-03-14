use crate::prelude::*;
use crate::GameState;

pub struct FlipperPlugin;

impl Plugin for FlipperPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(flipper_system.in_set(OnUpdate(GameState::Ingame)));
    }
}

#[derive(Component)]
pub struct Flipper;

pub fn spawn_flipper(
    parent: &mut ChildBuilder,
    pos: Vec3,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    assets: &mut PinballDefenseAssets,
) {
    parent
        .spawn((
            PbrBundle {
                mesh: assets.flipper.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::GRAY,
                    perceptual_roughness: 0.5,
                    metallic: 0.5,
                    reflectance: 0.5,
                    ..default()
                }),
                transform: Transform {
                    translation: pos,
                    scale: Vec3::new(1., 1., 1.) * 100.,
                    rotation: Quat::from_rotation_z(0.),
                },
                ..default()
            },
            Ccd::enabled(),
            ColliderDebugColor(Color::NONE),
            Collider::from_bevy_mesh(
                meshes.get(&assets.flipper).expect("Failed to find mesh"),
                &ComputedColliderShape::TriMesh,
            )
            .unwrap(),
        ))
        .insert(Flipper)
        .insert(Name::new("Flipper Left"));
}

fn flipper_system() {}
