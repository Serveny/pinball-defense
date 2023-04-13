use crate::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(ball_reset_system);
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
    let radius = 0.025;
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
        ActiveEvents::COLLISION_EVENTS,
        ColliderMassProperties::Mass(0.081),
        Restitution::coefficient(0.5),
    ))
    .insert(Ball)
    .insert(Name::new("Ball"));
}

fn ball_reset_system(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_ball: Query<(Entity, &Transform), With<Ball>>,
    ball_spawn: Res<BallSpawn>,
) {
    for (entity, transform) in q_ball.iter() {
        if transform.translation.y <= -1. {
            cmds.get_entity(entity).unwrap().despawn_recursive();
            spawn_ball(&mut cmds, &mut meshes, &mut materials, ball_spawn.0);
        }
    }
}
