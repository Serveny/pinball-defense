use crate::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ball)
            .add_system(ball_reset_system);
    }
}

#[derive(Component)]
struct Ball;

fn setup_ball(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_ball(&mut cmds, &mut meshes, &mut materials)
}

fn spawn_ball(
    cmds: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let radius = 5.;
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
            transform: Transform::from_xyz(-100., 200., 0.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(radius),
        Ccd::enabled(),
        ColliderDebugColor(Color::hsl(220.0, 1.0, 0.3)),
        GravityScale(200.),
    ))
    .insert(Ball);
}

fn ball_reset_system(
    mut cmds: Commands,
    q_ball: Query<(Entity, &Transform), With<Ball>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, transform) in q_ball.iter() {
        if transform.translation.y <= -300. {
            cmds.get_entity(entity).unwrap().despawn_recursive();
            spawn_ball(&mut cmds, &mut meshes, &mut materials);
        }
    }
}
