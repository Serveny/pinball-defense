use crate::assets::PinballDefenseAssets;
use crate::ball::BallSpawn;
use crate::ball_starter::BallStarterPlugin;
use crate::flipper::FlipperPlugin;
use crate::prelude::*;
use crate::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FlipperPlugin)
            .add_plugin(BallStarterPlugin)
            .add_system(setup_world.in_schedule(OnEnter(GameState::Ingame)));
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
    mut assets: ResMut<PinballDefenseAssets>,
) {
    cmds.spawn(SpatialBundle {
        transform: Transform::from_rotation(Quat::from_rotation_z(-0.25)),
        ..default()
    })
    .with_children(|parent| {
        parent
            .spawn((
                PbrBundle {
                    mesh: assets.world_1_mesh.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: Color::BLUE,
                        perceptual_roughness: 0.5,
                        metallic: 0.5,
                        reflectance: 0.5,
                        ..default()
                    }),
                    ..default()
                },
                //Ccd::enabled(),
                RigidBody::KinematicPositionBased,
                ColliderDebugColor(Color::NONE),
                Collider::from_bevy_mesh(
                    meshes
                        .get(&assets.world_1_mesh)
                        .expect("Failed to find mesh"),
                    &ComputedColliderShape::TriMesh,
                )
                .unwrap(),
            ))
            .insert(Ground);

        // Top glass
        let (x, y, z) = (260., 2., 140.);
        parent
            .spawn((
                SpatialBundle {
                    transform: Transform::from_translation(Vec3::new(0., 6., 0.)),
                    ..default()
                },
                ColliderDebugColor(Color::NONE),
                Collider::cuboid(x / 2., y / 2., z / 2.),
            ))
            .insert(Name::new("Pinball Glass"));
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
        crate::flipper::spawn_flipper_left(
            Transform {
                translation: Vec3::new(83., -1., 32.),
                //rotation: Quat::from_rotation_y(f32::to_radians(-12. + 180.)),
                ..default()
            },
            parent,
            &mut meshes,
            &mut materials,
            &mut assets,
        );
        crate::flipper::spawn_flipper_right(
            Transform {
                translation: Vec3::new(83., -1., -24.6),
                //rotation: Quat::from_rotation_y(f32::to_radians(12.)),
                ..default()
            },
            parent,
            &mut meshes,
            &mut materials,
            &mut assets,
        );
    })
    .insert(World)
    .insert(Name::new("Pinball World"));
    ball_spawn.0 = Vec3::new(96., -26., -60.);
}
