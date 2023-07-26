use crate::prelude::*;
use bevy::math::cubic_splines::CubicCurve;

#[derive(Resource)]
struct RoadAnimations(Vec<Handle<AnimationClip>>);

#[derive(Resource)]
struct RoadPath(Vec<Vec3>);

#[derive(Component)]
pub struct Curve(CubicCurve<Vec3>);

const ROAD_PATH: [[Vec3; 4]; 28] = [
    [
        Vec3::new(-1.30, 0.00, 0.50),
        Vec3::new(-1.30, 0.00, 0.50),
        Vec3::new(-1.30, 0.00, 0.50),
        Vec3::new(-1.25, 0.00, 0.50),
    ],
    [
        Vec3::new(-1.25, 0.00, 0.50),
        Vec3::new(-1.25, 0.00, 0.50),
        Vec3::new(-1.25, 0.00, 0.50),
        Vec3::new(-1.15, 0.00, 0.40),
    ],
    [
        Vec3::new(-1.15, 0.00, 0.40),
        Vec3::new(-1.15, 0.00, 0.40),
        Vec3::new(-1.15, 0.00, 0.40),
        Vec3::new(-1.15, 0.00, 0.10),
    ],
    [
        Vec3::new(-1.15, 0.00, 0.10),
        Vec3::new(-1.15, 0.00, 0.10),
        Vec3::new(-1.15, 0.00, 0.10),
        Vec3::new(-0.50, 0.00, -0.50),
    ],
    [
        Vec3::new(-0.50, 0.00, -0.50),
        Vec3::new(-0.50, 0.00, -0.50),
        Vec3::new(-0.50, 0.00, -0.50),
        Vec3::new(0.00, 0.00, -0.50),
    ],
    [
        Vec3::new(0.00, 0.00, -0.50),
        Vec3::new(0.00, 0.00, -0.50),
        Vec3::new(0.00, 0.00, -0.50),
        Vec3::new(0.00, 0.00, -0.30),
    ],
    [
        Vec3::new(0.00, 0.00, -0.30),
        Vec3::new(0.00, 0.00, -0.30),
        Vec3::new(0.00, 0.00, -0.30),
        Vec3::new(-0.70, 0.00, -0.30),
    ],
    [
        Vec3::new(-0.70, 0.00, -0.30),
        Vec3::new(-0.70, 0.00, -0.30),
        Vec3::new(-0.70, 0.00, -0.30),
        Vec3::new(-0.80, 0.00, -0.20),
    ],
    [
        Vec3::new(-0.80, 0.00, -0.20),
        Vec3::new(-0.80, 0.00, -0.20),
        Vec3::new(-0.80, 0.00, -0.20),
        Vec3::new(-0.80, 0.00, -0.00),
    ],
    [
        Vec3::new(-0.80, 0.00, -0.00),
        Vec3::new(-0.80, 0.00, -0.00),
        Vec3::new(-0.80, 0.00, -0.00),
        Vec3::new(-0.90, 0.00, 0.10),
    ],
    [
        Vec3::new(-0.90, 0.00, 0.10),
        Vec3::new(-0.90, 0.00, 0.10),
        Vec3::new(-0.90, 0.00, 0.10),
        Vec3::new(-1.00, 0.00, 0.20),
    ],
    [
        Vec3::new(-1.00, 0.00, 0.20),
        Vec3::new(-1.00, 0.00, 0.20),
        Vec3::new(-1.00, 0.00, 0.20),
        Vec3::new(-1.00, 0.00, 0.50),
    ],
    [
        Vec3::new(-1.00, 0.00, 0.50),
        Vec3::new(-1.00, 0.00, 0.50),
        Vec3::new(-1.00, 0.00, 0.50),
        Vec3::new(-0.90, 0.00, 0.60),
    ],
    [
        Vec3::new(-0.90, 0.00, 0.60),
        Vec3::new(-0.90, 0.00, 0.60),
        Vec3::new(-0.90, 0.00, 0.60),
        Vec3::new(-0.70, -0.00, 0.60),
    ],
    [
        Vec3::new(-0.70, -0.00, 0.60),
        Vec3::new(-0.70, -0.00, 0.60),
        Vec3::new(-0.70, -0.00, 0.60),
        Vec3::new(-0.60, -0.00, 0.50),
    ],
    [
        Vec3::new(-0.60, -0.00, 0.50),
        Vec3::new(-0.60, -0.00, 0.50),
        Vec3::new(-0.60, -0.00, 0.50),
        Vec3::new(-0.60, -0.00, -0.00),
    ],
    [
        Vec3::new(-0.60, -0.00, -0.00),
        Vec3::new(-0.60, -0.00, -0.00),
        Vec3::new(-0.60, -0.00, -0.00),
        Vec3::new(-0.50, -0.00, -0.10),
    ],
    [
        Vec3::new(-0.50, -0.00, -0.10),
        Vec3::new(-0.50, -0.00, -0.10),
        Vec3::new(-0.50, -0.00, -0.10),
        Vec3::new(-0.40, 0.00, -0.00),
    ],
    [
        Vec3::new(-0.40, 0.00, -0.00),
        Vec3::new(-0.40, 0.00, -0.00),
        Vec3::new(-0.40, 0.00, -0.00),
        Vec3::new(-0.40, 0.00, 0.50),
    ],
    [
        Vec3::new(-0.40, 0.00, 0.50),
        Vec3::new(-0.40, 0.00, 0.50),
        Vec3::new(-0.40, 0.00, 0.50),
        Vec3::new(-0.20, 0.00, 0.50),
    ],
    [
        Vec3::new(-0.20, 0.00, 0.50),
        Vec3::new(-0.20, 0.00, 0.50),
        Vec3::new(-0.20, 0.00, 0.50),
        Vec3::new(-0.20, 0.00, -0.00),
    ],
    [
        Vec3::new(-0.20, 0.00, -0.00),
        Vec3::new(-0.20, 0.00, -0.00),
        Vec3::new(-0.20, 0.00, -0.00),
        Vec3::new(0.00, 0.00, -0.00),
    ],
    [
        Vec3::new(0.00, 0.00, -0.00),
        Vec3::new(0.00, 0.00, -0.00),
        Vec3::new(0.00, 0.00, -0.00),
        Vec3::new(0.00, 0.00, 0.50),
    ],
    [
        Vec3::new(0.00, 0.00, 0.50),
        Vec3::new(0.00, 0.00, 0.50),
        Vec3::new(0.00, 0.00, 0.50),
        Vec3::new(0.20, 0.00, 0.50),
    ],
    [
        Vec3::new(0.20, 0.00, 0.50),
        Vec3::new(0.20, 0.00, 0.50),
        Vec3::new(0.20, 0.00, 0.50),
        Vec3::new(0.20, 0.00, 0.14),
    ],
    [
        Vec3::new(0.20, 0.00, 0.14),
        Vec3::new(0.20, 0.00, 0.14),
        Vec3::new(0.20, 0.00, 0.14),
        Vec3::new(0.30, 0.00, 0.04),
    ],
    [
        Vec3::new(0.30, 0.00, 0.04),
        Vec3::new(0.30, 0.00, 0.04),
        Vec3::new(0.30, 0.00, 0.04),
        Vec3::new(1.00, 0.00, 0.04),
    ],
    [
        Vec3::new(1.00, 0.00, 0.04),
        Vec3::new(1.00, 0.00, 0.04),
        Vec3::new(1.00, 0.00, 0.04),
        Vec3::new(1.20, 0.00, 0.04),
    ],
];

pub fn spawn_road(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
) {
    parent
        .spawn(PbrBundle {
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
        })
        .insert(Name::new("Road Mesh"));
}
pub fn add_road_path(
    parent: &mut ChildBuilder,
    assets: &PinballDefenseAssets,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    // Make a CubicCurve
    let bezier = Bezier::new(ROAD_PATH).to_curve();
    // Create the animation player, and set it to repeat
    let mut player = AnimationPlayer::default();
    player.play(assets.road_path.clone()).repeat();
    let curve = Curve(bezier);

    parent.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::new(0.1).into()),
            material: materials.add(Color::ORANGE.into()),
            transform: Transform::from_translation(ROAD_PATH[0][0]),
            ..default()
        },
        curve,
    ));
}

pub fn animate_cube(time: Res<Time>, mut query: Query<(&mut Transform, &Curve)>) {
    let t = time.elapsed_seconds() * 2.;

    for (mut transform, cubic_curve) in &mut query {
        // Draw the curve
        // and 1 is the last point
        transform.translation = cubic_curve.0.position(t);
    }
}
