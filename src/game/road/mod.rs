use self::points::ROAD_POINTS;
use crate::prelude::*;

pub mod points;

#[derive(Resource)]
struct RoadAnimations(Vec<Handle<AnimationClip>>);

#[derive(Resource)]
struct RoadPath(Vec<Vec3>);

#[allow(unused_variables)]
pub fn spawn_road(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
    assets: &PinballDefenseAssets,
) {
    parent.spawn((
        PbrBundle {
            mesh: assets.road_mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::TEAL,
                perceptual_roughness: 0.8,
                metallic: 0.,
                reflectance: 0.8,
                ..default()
            }),
            transform: Transform::from_xyz(0., -0.04, 0.),
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
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.005,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::GREEN,
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
