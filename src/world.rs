use crate::assets::PinballDefenseAssets;
use crate::ball::BallSpawn;
use crate::prelude::*;
use crate::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_world.in_schedule(OnEnter(GameState::Ingame)));
    }
}

const SIZE: Vec3 = Vec3 {
    x: 400.,
    y: 20.,
    z: 200.,
};

#[derive(Component)]
struct World;

#[derive(Component)]
struct Ground;

fn setup_world(
    mut cmds: Commands,
    mut ball_spawn: ResMut<BallSpawn>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: ResMut<PinballDefenseAssets>,
) {
    cmds.spawn(SpatialBundle {
        transform: Transform {
            translation: Vec3::ZERO,
            rotation: Quat::from_rotation_z(-0.25),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        let mesh = meshes
            .get(&assets.world_1_mesh)
            .expect("Failed to find mesh");
        parent
            .spawn((
                PbrBundle {
                    mesh: assets.world_1_mesh.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: Color::GRAY,
                        perceptual_roughness: 0.5,
                        metallic: 0.5,
                        reflectance: 0.5,
                        ..default()
                    }),
                    transform: Transform::from_scale(Vec3::new(1., 1., 1.) * 100.),
                    ..default()
                },
                Ccd::enabled(),
                ColliderDebugColor(Color::NONE),
                Collider::from_bevy_mesh(mesh, &ComputedColliderShape::TriMesh).unwrap(),
            ))
            .insert(Ground);
        parent.spawn(PointLightBundle {
            transform: Transform::from_xyz(0., SIZE.x / 4., 0.).looking_at(Vec3::ZERO, Vec3::Y),
            point_light: PointLight {
                intensity: 320000.,
                color: Color::WHITE,
                shadows_enabled: true,
                radius: SIZE.x / 20.,
                range: SIZE.x,
                ..default()
            },
            ..default()
        });
        crate::ball_starter::spawn(
            parent,
            Vec3::new(117.5, -1.8, -65.7),
            &mut meshes,
            &mut materials,
        );
    })
    .insert(World)
    .insert(Name::new("Pinball World"));
    ball_spawn.0 = Vec3::new(96., -26., -60.);
}
