use crate::events::collision::BALL;
use crate::events::collision::INTERACT_WITH_BALL;
use crate::events::collision::INTERACT_WITH_ENEMY;
use crate::pinball_menu::PinballMenuEvent;
use crate::prelude::*;
use crate::GameState;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnBallDespawn>()
            .add_event::<CollisionWithBallEvent>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    ball_reset_system,
                    on_ball_despawn_system,
                    max_speed_system,
                    collision_with_ball_system,
                )
                    .run_if(in_state(GameState::Ingame)),
            );
    }
}

#[derive(Component)]
pub struct PinBall;

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
        ColliderDebugColor(Color::GOLD),
        CollisionGroups::new(BALL.union(INTERACT_WITH_ENEMY), INTERACT_WITH_BALL),
        Sleeping::disabled(),
        ColliderMassProperties::Mass(0.081),
        Restitution::coefficient(0.5),
        Friction::coefficient(1.),
        Velocity::default(),
        PinBall,
        Name::new("Ball"),
    ))
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

#[derive(Event)]
pub struct OnBallDespawn;

fn ball_reset_system(
    mut cmds: Commands,
    mut evw: EventWriter<OnBallDespawn>,
    q_ball: Query<(Entity, &Transform), With<PinBall>>,
) {
    for (entity, transform) in q_ball.iter() {
        let trans = transform.translation;
        if trans.y <= -1. || trans.y >= 0.4 || (trans.x > 1.2 && trans.z > -0.3) {
            log!("🎱 Despawn ball");
            cmds.get_entity(entity).unwrap().despawn_recursive();
            evw.send(OnBallDespawn);
        }
    }
}

fn on_ball_despawn_system(
    mut evr: EventReader<OnBallDespawn>,
    mut pm_status_ev: EventWriter<PinballMenuEvent>,
) {
    if evr.iter().next().is_some() {
        pm_status_ev.send(PinballMenuEvent::Deactivate);
    }
}

// Prevent clipping of ball through objects
const MAX_SQUARED_SPEED: f32 = 40.;
const MAX_SQUARED_ANGLE_SPEED: f32 = 20_000.;

fn max_speed_system(mut q_ball: Query<&mut Velocity, With<PinBall>>) {
    q_ball.for_each_mut(limit_velocity);
}

pub fn limit_velocity(mut velocity: Mut<Velocity>) {
    let length = velocity.linvel.length_squared();
    if length > MAX_SQUARED_SPEED {
        velocity.linvel *= MAX_SQUARED_SPEED / length;

        log!("🥨 Limit velocity from {} to {}", length, velocity.linvel);

        // If we reduce speed, maybe we should reduce turn speed too, idk
        let length = velocity.angvel.length_squared();
        if length > MAX_SQUARED_ANGLE_SPEED {
            velocity.angvel *= MAX_SQUARED_ANGLE_SPEED / length;

            log!("💫 Limit turn speed from {} to {}", length, velocity.angvel);
        }
    }
}

#[derive(Event)]
pub struct CollisionWithBallEvent(pub Entity, pub CollisionEventFlags);

impl CollisionWithBallEvent {
    pub fn new(ev: (Entity, CollisionEventFlags)) -> Self {
        Self(ev.0, ev.1)
    }
}

fn collision_with_ball_system(
    coll_ev: EventReader<CollisionEvent>,
    mut coll_with_ball_ev: EventWriter<CollisionWithBallEvent>,
    q_ball: Query<With<PinBall>>,
) {
    for ev in ball_collision_start_only(coll_ev, q_ball) {
        coll_with_ball_ev.send(CollisionWithBallEvent::new(ev));
    }
}

fn ball_collision_start_only(
    mut coll_ev: EventReader<CollisionEvent>,
    q_ball: Query<With<PinBall>>,
) -> Vec<(Entity, CollisionEventFlags)> {
    coll_ev
        .iter()
        .filter_map(|ev| match ev {
            CollisionEvent::Started(id_1, id_2, flag) => match q_ball.contains(*id_1) {
                true => Some((*id_2, *flag)),
                false => match q_ball.contains(*id_2) {
                    true => Some((*id_1, *flag)),
                    false => None,
                },
            },
            CollisionEvent::Stopped(_, _, _) => None,
        })
        .collect()
}
