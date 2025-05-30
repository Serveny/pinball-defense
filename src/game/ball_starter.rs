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
            .add_event::<SpawnBallEvent>()
            .add_event::<BallStarterChargeStartedEvent>()
            .add_event::<BallStarterFireEndEvent>()
            .add_systems(Startup, setup)
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

#[derive(Event)]
pub struct SpawnBallEvent;

fn on_spawn_ball_system(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut evr: EventReader<SpawnBallEvent>,
    mut sound_ev: EventWriter<SoundEvent>,
    ball_spawn: Res<BallSpawn>,
) {
    for _ in evr.read() {
        ball::spawn(&mut cmds, &mut meshes, &mut materials, ball_spawn.0);
        sound_ev.write(SoundEvent::BallSpawn);
    }
}

pub fn spawn(spawner: &mut ChildSpawnerCommands, pos: Vec3, assets: &PinballDefenseGltfAssets) {
    let collider = |p: &mut ChildSpawnerCommands| {
        p.spawn(collider_bundle());
    };
    spawner
        .spawn((
            spatial_from_pos(pos),
            BallStarter,
            Name::new("Ball Starter"),
        ))
        .with_children(|p| {
            p.spawn(starter_plate(assets))
                // Long cube collider to prevent clipping ball
                .with_children(collider);
            p.spawn(starter_spring(assets));
            p.spawn(starter_rod(assets));
        });
}

fn collider_bundle() -> impl Bundle {
    (
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

#[derive(Component)]
struct StarterPlate;

fn starter_plate(assets: &PinballDefenseGltfAssets) -> impl Bundle {
    (
        Name::new("Starter Plate"),
        StarterPlate,
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
    mut spawn_ball_ev: EventWriter<SpawnBallEvent>,
    q_ball: Query<Entity, With<PinBall>>,
) {
    if q_ball.is_empty() {
        spawn_ball_ev.write(SpawnBallEvent);
    }
}

#[derive(Event)]
pub struct BallStarterChargeStartedEvent;

fn on_charge_started(
    mut charge_started_ev: EventWriter<BallStarterChargeStartedEvent>,
    mut sound_ev: EventWriter<SoundEvent>,
) {
    charge_started_ev.write(BallStarterChargeStartedEvent);
    sound_ev.write(SoundEvent::BallStarterCharge);
}

const MAX_PLATE_TRANSFORM: f32 = 0.08;

fn charge_system(
    mut q_plate: Query<&mut Transform, (With<StarterPlate>, Without<StarterSpring>)>,
    mut q_spring: Query<&mut Transform, (With<StarterSpring>, Without<StarterPlate>)>,
    mut state: ResMut<NextState<BallStarterState>>,
    time: Res<Time>,
) {
    let Ok(mut plate) = q_plate.single_mut() else {
        return;
    };
    let Ok(mut spring) = q_spring.single_mut() else {
        return;
    };
    let x_add = time.delta_secs() * 0.14;
    if starter_add(x_add, &mut plate.translation, &mut spring.scale) >= MAX_PLATE_TRANSFORM {
        state.set(BallStarterState::Idle);
    }
}

fn starter_add(pos_x: f32, plate_pos: &mut Vec3, spring_scale: &mut Vec3) -> f32 {
    let x = (plate_pos.x + pos_x).clamp(0., MAX_PLATE_TRANSFORM);
    plate_pos.x = x;
    spring_scale.x = 1. - (x / MAX_PLATE_TRANSFORM / 2.6);
    x
}

#[derive(Event)]
pub struct BallStarterFireEndEvent;

fn fire_system(
    mut q_plate: Query<&mut Transform, (With<StarterPlate>, Without<StarterSpring>)>,
    mut q_spring: Query<&mut Transform, (With<StarterSpring>, Without<StarterPlate>)>,
    mut state: ResMut<NextState<BallStarterState>>,
    mut leave_ev: EventWriter<BallStarterFireEndEvent>,
    time: Res<Time>,
) {
    let Ok(mut plate) = q_plate.single_mut() else {
        return;
    };
    let Ok(mut spring) = q_spring.single_mut() else {
        return;
    };
    let x_add = -time.delta_secs() * 1.4;
    if starter_add(x_add, &mut plate.translation, &mut spring.scale) <= 0. {
        state.set(BallStarterState::Idle);
        leave_ev.write(BallStarterFireEndEvent);
    }
}

fn on_fire_started(mut sound_ev: EventWriter<SoundEvent>) {
    sound_ev.write(SoundEvent::BallStarterFire);
}
