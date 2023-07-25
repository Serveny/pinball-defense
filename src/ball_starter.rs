use crate::ball::{spawn_ball, BallSpawn, PinBall};
use crate::prelude::*;

pub struct BallStarterPlugin;

impl Plugin for BallStarterPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<BallStarterState>()
            .add_systems(OnEnter(BallStarterState::Charge), spawn_ball_at_charge)
            .add_systems(
                Update,
                charge_system.run_if(in_state(BallStarterState::Charge)),
            )
            .add_systems(Update, fire_system.run_if(in_state(BallStarterState::Fire)));
    }
}
const HALF_SIZE: Vec3 = Vec3 {
    x: 0.099,
    y: 0.025,
    z: 0.025,
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
                        material: materials.add(Color::ORANGE.into()),
                        transform: Transform::from_translation(Vec3::new(-HALF_SIZE.x, 0., 0.)),
                        ..default()
                    },
                    RigidBody::KinematicPositionBased,
                    ColliderDebugColor(Color::GOLD),
                    //Ccd::enabled(),
                ))
                // Long cube collider to prevent clipping ball
                .with_children(|parent| {
                    parent
                        .spawn(Collider::cuboid(HALF_SIZE.x, HALF_SIZE.y, HALF_SIZE.z))
                        .insert(TransformBundle::from(Transform::from_xyz(0.09, 0., 0.)));
                })
                .insert(BallStarterPlate)
                .insert(Speed(1.));
        })
        .insert(BallStarter)
        .insert(Name::new("Ball Starter"));
}

fn spawn_ball_at_charge(
    mut cmds: Commands,
    ball_spawn: Res<BallSpawn>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_ball: Query<&PinBall>,
) {
    if q_ball.is_empty() {
        spawn_ball(&mut cmds, &mut meshes, &mut materials, ball_spawn.0);
    }
}

fn charge_system(
    mut q_ball_starter: Query<&mut Transform, With<BallStarterPlate>>,
    time: Res<Time>,
) {
    q_ball_starter.for_each_mut(|mut transform| {
        transform.translation.x =
            (transform.translation.x + time.delta_seconds() * 0.2).clamp(-HALF_SIZE.x, HALF_SIZE.x)
    });
}

fn fire_system(
    mut ball_starter_state: ResMut<NextState<BallStarterState>>,
    mut q_ball_starter: Query<(&mut Speed, &mut Transform)>,
    time: Res<Time>,
) {
    q_ball_starter.for_each_mut(|(mut speed, mut transform)| {
        speed.0 += speed.0 * time.delta_seconds() * 32.;
        transform.translation.x -= time.delta_seconds() * speed.0;

        if transform.translation.x <= -HALF_SIZE.x {
            transform.translation.x = -HALF_SIZE.x;
            speed.0 = 1.;
            ball_starter_state.set(BallStarterState::Idle);
        }
    });
}
