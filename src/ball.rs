use crate::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, ball_reset_system);
    }
}

#[derive(Component)]
pub struct Ball;

#[derive(Resource, Default)]
pub struct BallSpawn(pub Vec3);

fn setup(mut cmds: Commands) {
    cmds.init_resource::<BallSpawn>();
}
pub fn spawn_ball(
    cmds: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    pos: Vec3,
) {
    let radius = 0.02;
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius,
                ..default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::SILVER,
                perceptual_roughness: 0.,
                metallic: 1.,
                reflectance: 1.,
                ..default()
            }),
            transform: Transform::from_translation(pos),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(radius),
        //Ccd::enabled(),
        ColliderDebugColor(Color::GOLD),
        Sleeping::disabled(),
        ColliderMassProperties::Mass(0.081),
        Restitution::coefficient(0.5),
        Friction::coefficient(1.),
    ))
    .insert(Ball)
    .insert(Name::new("Ball"))
    .with_children(|parent| {
        parent.spawn(PointLightBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            point_light: PointLight {
                intensity: 0.01,
                color: Color::SILVER,
                shadows_enabled: false,
                radius: 0.001,
                range: 0.1,
                ..default()
            },
            ..default()
        });
    });
}

fn ball_reset_system(mut cmds: Commands, q_ball: Query<(Entity, &Transform), With<Ball>>) {
    for (entity, transform) in q_ball.iter() {
        if transform.translation.y <= -1. {
            cmds.get_entity(entity).unwrap().despawn_recursive();
        }
    }
}
