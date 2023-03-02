use crate::prelude::*;

const HALF_SIZE: Vec3 = Vec3 {
    x: 10.3,
    y: 5.3,
    z: 5.3,
};

#[derive(Component)]
pub struct BallStarter;

#[derive(Component)]
pub struct BallStarterPlate;

pub fn spawn(
    cmds: &mut ChildBuilder,
    pos: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    cmds.spawn(SpatialBundle {
        transform: Transform {
            translation: (pos
                + Vec3::new(
                    pos.x.signum() * -HALF_SIZE.x,
                    HALF_SIZE.y,
                    pos.z.signum() * -HALF_SIZE.z,
                )),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent
            .spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::new(HALF_SIZE.y * 2.))),
                    material: materials.add(Color::RED.into()),
                    ..default()
                },
                RigidBody::KinematicPositionBased,
                Collider::cuboid(HALF_SIZE.y, HALF_SIZE.y, HALF_SIZE.y),
                ColliderDebugColor(Color::GREEN),
            ))
            .insert(BallStarterPlate);
    })
    .insert(BallStarter)
    .insert(Name::new("Ball Starter"));
}

pub fn get_ball_spawn_global_pos(q_starter: Query<&GlobalTransform, With<BallStarter>>) -> Vec3 {
    let mut pos = q_starter
        .get_single()
        .expect("No ball starter")
        .translation();
    // Ball spawn in the upper half of the starter
    pos.x += pos.x.signum() * -HALF_SIZE.x * 2.;
    pos.y += HALF_SIZE.y * 2.;
    pos
}
