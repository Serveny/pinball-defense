use super::audio::SoundEvent;
use super::level::PointsEvent;
use crate::game::ball::{self, PinBall};
use crate::game::events::collision::COLLIDE_ONLY_WITH_BALL;
use crate::prelude::*;

pub struct BallStarterPlugin;

impl Plugin for BallStarterPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<BallStarterState>()
            .add_event::<SpawnBallEvent>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(BallStarterState::Charge), spawn_ball_at_charge)
            .add_systems(
                Update,
                (charge_system, spawn_ball_system).run_if(in_state(BallStarterState::Charge)),
            )
            .add_systems(Update, fire_system.run_if(in_state(BallStarterState::Fire)));
    }
}

#[derive(Event)]
pub struct SpawnBallEvent;

fn spawn_ball_system(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawn_ball_ev: EventReader<SpawnBallEvent>,
    mut points_ev: EventWriter<PointsEvent>,
    mut sound_ev: EventWriter<SoundEvent>,
    ball_spawn: Res<BallSpawn>,
) {
    for _ in spawn_ball_ev.iter() {
        ball::spawn(&mut cmds, &mut meshes, &mut materials, ball_spawn.0);
        points_ev.send(PointsEvent::BallSpawned);
        sound_ev.send(SoundEvent::BallSpawn);
    }
}

#[derive(Resource, Default)]
struct BallSpawn(pub Vec3);

fn setup(mut cmds: Commands) {
    cmds.insert_resource(BallSpawn(Vec3::new(0.96, 0.6, -0.02)));
}
const HALF_SIZE: Vec3 = Vec3 {
    x: 0.099,
    y: 0.025,
    z: 0.025,
};

#[derive(Component)]
struct BallStarter;

#[derive(Component)]
struct BallStarterPlate;

#[derive(Component)]
struct Speed(f32);

// The number is the signum for the direction
#[derive(States, PartialEq, Eq, Clone, Copy, Debug, Hash, Default, SystemSet)]
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
    let collider = |p: &mut ChildBuilder| {
        p.spawn((
            TransformBundle::from(Transform::from_xyz(0.09, 0., 0.)),
            Collider::cuboid(HALF_SIZE.x, HALF_SIZE.z),
            RigidBody::KinematicPositionBased,
            Restitution {
                coefficient: 0.,
                combine_rule: CoefficientCombineRule::Multiply,
            },
            ColliderDebugColor(Color::GOLD),
            COLLIDE_ONLY_WITH_BALL,
        ));
    };
    let plate = |p: &mut ChildBuilder| {
        p.spawn((
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
            BallStarterPlate,
            Speed(1.),
            //Ccd::enabled(),
        ))
        // Long cube collider to prevent clipping ball
        .with_children(collider);
    };
    parent
        .spawn((
            spatial_from_pos(pos),
            BallStarter,
            Name::new("Ball Starter"),
        ))
        .with_children(plate);
}

fn spawn_ball_at_charge(
    mut spawn_ball_ev: EventWriter<SpawnBallEvent>,
    q_ball: Query<With<PinBall>>,
) {
    if q_ball.is_empty() {
        spawn_ball_ev.send(SpawnBallEvent);
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
