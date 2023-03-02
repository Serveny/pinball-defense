use crate::ball::BallSpawn;
use crate::ball_starter::{get_ball_spawn_global_pos, BallStarter};
use crate::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        static POST: &str = "post";
        app.add_startup_system(setup_world)
            .add_startup_stage_after(
                StartupStage::PostStartup,
                POST,
                SystemStage::single_threaded(),
            )
            .add_startup_system_to_stage(POST, set_ball_spawn);
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        let half_ground_height = 2.;
        parent
            .spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box {
                        min_x: -SIZE.x / 2.,
                        max_x: SIZE.x / 2.,
                        min_y: -half_ground_height,
                        max_y: half_ground_height,
                        min_z: -SIZE.z / 2.,
                        max_z: SIZE.z / 2.,
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::GRAY,
                        perceptual_roughness: 0.5,
                        metallic: 0.5,
                        reflectance: 0.5,
                        ..default()
                    }),
                    ..default()
                },
                Collider::cuboid(SIZE.x / 2., half_ground_height, SIZE.z / 2.),
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
            Vec3::new(SIZE.x / 2., half_ground_height, -SIZE.z / 2.),
            &mut meshes,
            &mut materials,
        );
    })
    .insert(World)
    .insert(Name::new("Pinball World"));
}

fn set_ball_spawn(mut cmds: Commands, q_starter: Query<&GlobalTransform, With<BallStarter>>) {
    let spawn_pos = get_ball_spawn_global_pos(q_starter);
    println!("Set ball spawn to {spawn_pos}");
    cmds.insert_resource(BallSpawn(spawn_pos));
}
