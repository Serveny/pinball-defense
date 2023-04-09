use crate::prelude::*;

pub struct BallStarterPlugin;

impl Plugin for BallStarterPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<BallStarterState>()
            .add_system(charge_system.in_set(OnUpdate(BallStarterState::Charge)))
            .add_system(fire_system.in_set(OnUpdate(BallStarterState::Fire)));
    }
}
const HALF_SIZE: Vec3 = Vec3 {
    x: 9.9,
    y: 2.5,
    z: 2.5,
};

#[derive(Component)]
pub struct BallStarter;

#[derive(Component)]
pub struct BallStarterPlate;

#[derive(Component)]
pub struct Speed(f32);

// The number is the signum for the direction
#[derive(States, PartialEq, Eq, Clone, Copy, Debug, Hash, Default, SystemSet)]
#[allow(dead_code)]
pub enum BallStarterState {
    #[default]
    Idle = 0,
    Charge = -1,
    Fire = 1,
}

pub fn spawn(
    parent: &mut ChildBuilder,
    pos: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    parent
        .spawn((SpatialBundle {
            transform: Transform::from_translation(pos),
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box::new(
                            HALF_SIZE.x / 4.,
                            HALF_SIZE.y * 2.,
                            HALF_SIZE.z * 2.,
                        ))),
                        material: materials.add(Color::RED.into()),
                        transform: Transform::from_translation(Vec3::new(-HALF_SIZE.x, 0., 0.)),
                        ..default()
                    },
                    RigidBody::KinematicPositionBased,
                    Collider::cuboid(HALF_SIZE.x / 8., HALF_SIZE.y, HALF_SIZE.z),
                    ColliderDebugColor(Color::GREEN),
                    //Ccd::enabled(),
                ))
                .insert(BallStarterPlate)
                .insert(Speed(1.));
        })
        .insert(BallStarter)
        .insert(Name::new("Ball Starter"));
}

fn charge_system(
    mut q_ball_starter: Query<&mut Transform, With<BallStarterPlate>>,
    time: Res<Time>,
) {
    q_ball_starter.for_each_mut(|mut transform| {
        transform.translation.x =
            (transform.translation.x + time.delta_seconds() * 20.).clamp(-HALF_SIZE.x, HALF_SIZE.x)
    });
}

fn fire_system(
    mut ball_starter_state: ResMut<NextState<BallStarterState>>,
    mut q_ball_starter: Query<(&mut Speed, &mut Transform)>,
    time: Res<Time>,
) {
    q_ball_starter.for_each_mut(|(mut speed, mut transform)| {
        speed.0 += speed.0 * time.delta_seconds() * 100.;
        transform.translation.x -= time.delta_seconds() * speed.0;

        if transform.translation.x <= -HALF_SIZE.x {
            transform.translation.x = -HALF_SIZE.x;
            speed.0 = 1.;
            ball_starter_state.set(BallStarterState::Idle);
        }
    });
}
