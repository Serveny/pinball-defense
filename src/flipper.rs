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
    name: &str,
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
        .insert(Flipper)
        .insert(Name::new(String::from(name)));
}

fn flipper_system() {}
