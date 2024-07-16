use crate::generated::world_1::road_points::ROAD_POINTS;
use crate::prelude::*;
use bevy::color::palettes::css::GREEN;
use bevy::math::primitives::Sphere;

#[allow(dead_code)]
#[derive(Resource)]
struct RoadAnimations(Vec<Handle<AnimationClip>>);

#[allow(dead_code)]
#[derive(Resource)]
struct RoadPath(Vec<Vec3>);

#[allow(unused_variables)]
pub fn spawn_road(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
    assets: &PinballDefenseGltfAssets,
) {
    parent.spawn((
        PbrBundle {
            mesh: assets.road_mesh.clone(),
            material: assets.road_material.clone(),
            ..default()
        },
        Name::new("Road Mesh"),
    ));
    //spawn_road_milestones(parent, materials, meshes);
}

#[allow(dead_code)]
fn spawn_road_milestones(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
) {
    for pos in ROAD_POINTS {
        parent.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere {
                radius: 0.005,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: GREEN.into(),
                perceptual_roughness: 0.8,
                metallic: 0.,
                reflectance: 0.8,
                ..default()
            }),
            transform: Transform::from_translation(pos),
            ..default()
        });
    }
}
