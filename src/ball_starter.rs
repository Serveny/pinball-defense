use crate::prelude::*;

#[derive(Component)]
pub struct BallStarter;

pub fn spawn(
    cmds: &mut ChildBuilder,
    pos: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let half_size = 5.3;
    cmds.spawn(SpatialBundle {
        transform: Transform {
            translation: pos,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::new(half_size * 2.))),
                material: materials.add(Color::RED.into()),
                // Pos inside on x/z-axis and up or down on y-axis
                transform: Transform::from_xyz(
                    pos.x.signum() * -half_size,
                    pos.y.signum() * half_size,
                    pos.z.signum() * -half_size,
                ),
                ..default()
            },
            RigidBody::KinematicPositionBased,
            Collider::cuboid(half_size, half_size, half_size),
            ColliderDebugColor(Color::GREEN),
        ));
    })
    .insert(BallStarter)
    .insert(Name::new("Ball Starter"));
}
