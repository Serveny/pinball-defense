use super::audio::SoundEvent;
use super::events::collision::GameLayer;
use super::{EventState, GameState};
use crate::game::ball::{self, PinBall};
use crate::prelude::*;
use bevy::color::palettes::css::GOLD;

pub struct BallStarterPlugin;

impl Plugin for BallStarterPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BallStarterState>()
            .add_message::<SpawnBallEvent>()
            .add_message::<BallStarterChargeStartedEvent>()
            .add_message::<BallStarterFireEndEvent>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(BallStarterState::Idle), on_enter_idle)
            .add_systems(
                OnEnter(BallStarterState::Charge),
                (spawn_ball_at_charge, on_charge_started),
            )
            .add_systems(OnEnter(BallStarterState::Fire), on_fire_started)
            .add_systems(
                Update,
                (charge_system)
                    .run_if(in_state(BallStarterState::Charge).and(in_state(GameState::Ingame))),
            )
            .add_systems(
                Update,
                (on_spawn_ball_system).run_if(in_state(EventState::Active)),
            )
            .add_systems(
                Update,
                fire_system
                    .run_if(in_state(BallStarterState::Fire).and(in_state(GameState::Ingame))),
            );
    }
}

#[derive(Message)]
pub struct SpawnBallEvent;

fn on_spawn_ball_system(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut evr: MessageReader<SpawnBallEvent>,
    mut sound_ev: MessageWriter<SoundEvent>,
    ball_spawn: Res<BallSpawn>,
) {
    for _ in evr.read() {
        ball::spawn(&mut cmds, &mut meshes, &mut materials, ball_spawn.0);
        sound_ev.write(SoundEvent::BallSpawn);
    }
}

pub fn spawn(spawner: &mut ChildSpawnerCommands, pos: Vec3, assets: &PinballDefenseGltfAssets) {
    let collider = |p: &mut ChildSpawnerCommands| {
        p.spawn(starter_plate_mesh(assets));
    };
    spawner
        .spawn((
            spatial_from_pos(pos),
            BallStarter,
            Name::new("Ball Starter"),
        ))
        .with_children(|p| {
            // Long cube collider to prevent clipping ball
            p.spawn(collider_bundle()).with_children(collider);
            p.spawn(starter_spring(assets));
            p.spawn(starter_rod(assets));
        });
}

#[derive(Component)]
struct StarterPlate;

fn collider_bundle() -> impl Bundle {
    (
        StarterPlate,
        Name::new("Ball Starter Collider"),
        Transform::from_xyz(-0.107, 0., 0.),
        Collider::rectangle(SIZE.x, SIZE.z),
        RigidBody::Kinematic,
        Restitution {
            coefficient: 1.,
            combine_rule: CoefficientCombine::Multiply,
        },
        DebugRender::collider(GOLD.into()),
        CollisionLayers::new(GameLayer::Map, GameLayer::Ball),
    )
}

fn starter_plate_mesh(assets: &PinballDefenseGltfAssets) -> impl Bundle {
    (
        Name::new("Starter Plate Mesh"),
        Transform::from_xyz(0.107, 0., 0.),
        Mesh3d(assets.starter_plate.clone()),
        MeshMaterial3d(assets.starter_plate_material.clone()),
    )
}

#[derive(Component)]
struct StarterSpring;

fn starter_spring(assets: &PinballDefenseGltfAssets) -> impl Bundle {
    (
        Name::new("Starter Spring"),
        StarterSpring,
        Mesh3d(assets.starter_spring.clone()),
        MeshMaterial3d(assets.starter_spring_material.clone()),
    )
}

#[derive(Component)]
struct StarterRod;

fn starter_rod(assets: &PinballDefenseGltfAssets) -> impl Bundle {
    (
        Name::new("Starter Rod"),
        StarterRod,
        Mesh3d(assets.starter_balance_rod.clone()),
        MeshMaterial3d(assets.starter_balance_rod_material.clone()),
    )
}

#[derive(Resource, Default)]
pub struct BallSpawn(pub Vec3);

fn setup(mut cmds: Commands) {
    cmds.insert_resource(BallSpawn(Vec3::new(0.96, 0.6, -0.02)));
}
const SIZE: Vec3 = Vec3 {
    x: 0.2,
    y: 0.05,
    z: 0.05,
};

#[derive(Component)]
struct BallStarter;

// The number is the signum for the direction
#[derive(States, PartialEq, Eq, Clone, Copy, Debug, Hash, Default, SystemSet)]
pub enum BallStarterState {
    #[default]
    Idle = 0,
    Charge = -1,
    Fire = 1,
}

fn spawn_ball_at_charge(
    mut spawn_ball_ev: MessageWriter<SpawnBallEvent>,
    q_ball: Query<Entity, With<PinBall>>,
) {
    if q_ball.is_empty() {
        spawn_ball_ev.write(SpawnBallEvent);
    }
}

fn on_enter_idle(q_plate: Query<&mut LinearVelocity, With<StarterPlate>>) {
    for mut velocity in q_plate {
        velocity.x = 0.;
    }
}

#[derive(Message)]
pub struct BallStarterChargeStartedEvent;

fn on_charge_started(
    mut charge_started_ev: MessageWriter<BallStarterChargeStartedEvent>,
    mut sound_ev: MessageWriter<SoundEvent>,
    mut q_plate: Query<&mut LinearVelocity, With<StarterPlate>>,
) {
    charge_started_ev.write(BallStarterChargeStartedEvent);
    sound_ev.write(SoundEvent::BallStarterCharge);
    for mut velocity in q_plate.iter_mut() {
        velocity.x = 0.1;
    }
}

fn charge_system(
    q_plate: Query<&Transform, (With<StarterPlate>, Without<StarterSpring>)>,
    mut q_spring: Query<&mut Transform, (With<StarterSpring>, Without<StarterPlate>)>,
    mut state: ResMut<NextState<BallStarterState>>,
) {
    let Ok(plate) = q_plate.single() else {
        return;
    };
    let Ok(mut spring) = q_spring.single_mut() else {
        return;
    };
    let x = plate.translation.x;
    update_spring_scale(x, &mut spring.scale);

    if x >= 0. {
        state.set(BallStarterState::Idle);
    }
}

fn update_spring_scale(plate_x: f32, spring_scale: &mut Vec3) {
    spring_scale.x = 1. - (plate_x + 0.107 / (0.107 * 2.) / 2.6);
}

#[derive(Message)]
pub struct BallStarterFireEndEvent;

fn fire_system(
    q_plate: Query<&Transform, (With<StarterPlate>, Without<StarterSpring>)>,
    mut q_spring: Query<&mut Transform, (With<StarterSpring>, Without<StarterPlate>)>,
    mut state: ResMut<NextState<BallStarterState>>,
    mut leave_ev: MessageWriter<BallStarterFireEndEvent>,
) {
    let Ok(plate) = q_plate.single() else {
        return;
    };
    let Ok(mut spring) = q_spring.single_mut() else {
        return;
    };
    let x = plate.translation.x;
    update_spring_scale(x, &mut spring.scale);
    if x <= -0.107 {
        state.set(BallStarterState::Idle);
        leave_ev.write(BallStarterFireEndEvent);
    }
}

fn on_fire_started(
    mut sound_ev: MessageWriter<SoundEvent>,
    q_plate: Query<&mut LinearVelocity, With<StarterPlate>>,
) {
    for mut velocity in q_plate {
        velocity.x = -20.;
    }
    sound_ev.write(SoundEvent::BallStarterFire);
}
