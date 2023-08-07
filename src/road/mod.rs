use self::points::ROAD_POINTS;
use crate::prelude::*;
use bevy::math::cubic_splines::CubicCurve;

pub mod points;

#[derive(Resource)]
struct RoadAnimations(Vec<Handle<AnimationClip>>);

#[derive(Resource)]
struct RoadPath(Vec<Vec3>);

#[derive(Component)]
pub struct Curve(CubicCurve<Vec3>);

pub fn spawn_road(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    meshes: &mut Assets<Mesh>,
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
            transform: Transform::from_xyz(0., -0.04, -0.008),
            ..default()
        },
        Name::new("Road Mesh"),
    ));
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

//pub fn add_road_path(
//parent: &mut ChildBuilder,
//meshes: &mut Assets<Mesh>,
//materials: &mut Assets<StandardMaterial>,
////animations: &mut Assets<AnimationClip>,
//) {
//// Make a CubicCurve
//let bezier = Bezier::new(ROAD_PATH).to_curve();
//// Create the animation player, and set it to repeat
////let mut player = AnimationPlayer::default();
////player.play(animations.add(ROAD_PATH.into())).repeat();
//let curve = Curve(bezier);

//parent.spawn((
//PbrBundle {
//mesh: meshes.add(shape::Cube::new(0.1).into()),
//material: materials.add(Color::ORANGE.into()),
//transform: Transform::from_translation(ROAD_PATH[0][0]),
//..default()
//},
//curve,
//));
//}

//pub fn animate_cube(time: Res<Time>, mut query: Query<(&mut Transform, &Curve)>) {
//let t = time.elapsed_seconds();

//for (mut transform, cubic_curve) in &mut query {
//// Draw the curve
//// and 1 is the last point
//transform.translation = cubic_curve.0.position(t);
//}
//}
