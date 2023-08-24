use self::damage::DamageOverTime;
use self::light::{contact_light_bundle, FlashLight, LightOnCollision};
use self::target::{EnemiesWithinReach, SightRadius, TargetPos};
use super::ball::CollisionWithBallEvent;
use super::events::collision::{COLLIDE_ONLY_WITH_BALL, COLLIDE_ONLY_WITH_ENEMY};
use super::level::{Level, PointsEvent};
use super::pinball_menu::{PinballMenuTrigger, UpgradeMenuExecuteEvent};
use super::progress_bar::{self, ProgressBarCountUpEvent};
use super::{analog_counter, GameState};
use crate::game::analog_counter::AnalogCounterSetEvent;
use crate::game::tower::light::disable_flash_light;
use crate::game::world::QueryWorld;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::{Animator, Delay, EaseFunction, Sequence, Tween};
use std::time::Duration;
pub use types::TowerType;
use types::*;

mod animations;
mod damage;
pub mod foundation;
pub mod light;
mod speed;
mod target;
mod types;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTowerEvent>()
            .add_event::<DamageUpgradeEvent>()
            .add_event::<RangeUpgradeEvent>()
            // Tower systems
            .add_systems(
                Update,
                (
                    progress_system,
                    spawn_tower_system,
                    upgrade_system,
                    damage_upgrade_system,
                    range_upgrade_system,
                )
                    .run_if(in_state(GameState::Ingame)),
            )
            // Child systems
            .add_systems(
                Update,
                (
                    animations::rotate_always_system,
                    animations::rotate_to_target_system,
                    damage::afe_damage_over_time_system,
                    damage::datir_damage_over_time_system,
                    foundation::despawn_system,
                    foundation::progress_system,
                    light::contact_light_on_system,
                    light::add_flashlight_system.after(light::disable_contact_light_system),
                    light::flash_light_system,
                    light::disable_contact_light_system,
                    speed::afe_slow_down_system,
                    target::aim_first_enemy_system,
                    target::enemy_within_reach_system,
                    target::remove_despawned_enemies_from_ewr_system,
                    target::target_pos_by_afe_system,
                    types::gun::shot_animation_system,
                    types::microwave::shot_animation_system,
                    types::tesla::shot_animation_system,
                )
                    .run_if(in_state(GameState::Ingame)),
            );
    }
}

#[derive(Component, Debug)]
pub struct Tower {
    pos: Vec3,
}

impl Tower {
    pub fn new(pos: Vec3) -> Self {
        Self { pos }
    }
}

#[derive(Component)]
pub struct TowerHead;

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum TowerUpgrade {
    Damage,
    Range,
}

fn tower_bundle(pos: Vec3, sight_radius: f32) -> impl Bundle {
    (
        // General Tower components
        spatial_from_pos(tower_start_pos(pos)),
        Tower::new(pos),
        TowerLevel(0),
        //
        // Enemy target system
        TargetPos(None),
        SightRadius(sight_radius),
        EnemiesWithinReach::default(),
        //
        // Collider
        RigidBody::KinematicPositionBased,
        Restitution {
            coefficient: 2.,
            combine_rule: CoefficientCombineRule::Multiply,
        },
        ActiveEvents::COLLISION_EVENTS,
        ColliderDebugColor(Color::RED),
        Collider::ball(0.06),
        COLLIDE_ONLY_WITH_BALL,
        PinballMenuTrigger::Upgrade,
        LightOnCollision,
        //
        // Spawn animation
        Animator::new(create_tower_spawn_animator(pos)),
    )
}

#[derive(Component)]
pub struct TowerSightSensor;

fn tower_sight_sensor_bundle(radius: f32) -> impl Bundle {
    (
        PbrBundle::default(),
        Sensor,
        RigidBody::KinematicPositionBased,
        ColliderDebugColor(Color::ORANGE),
        Collider::ball(radius),
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::KINEMATIC_KINEMATIC,
        COLLIDE_ONLY_WITH_ENEMY,
        TowerSightSensor,
    )
}

fn tower_base_bundle(
    assets: &PinballDefenseGltfAssets,
    mats: &mut Assets<StandardMaterial>,
) -> impl Bundle {
    (
        Name::new("Tower Base"),
        PbrBundle {
            mesh: assets.tower_base.clone(),
            material: mats.add(tower_material()),
            ..default()
        },
    )
}

fn spawn(
    pb_world: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
    sight_radius: f32,
    tower_type_bundle: impl Bundle,
    add_to_tower: impl Fn(&mut ChildBuilder),
) {
    pb_world
        .spawn(tower_bundle(pos, sight_radius))
        .insert(tower_type_bundle)
        .with_children(|p| {
            let tower_id = p.parent_entity();
            let color = Color::RED;
            let bar_trans =
                Transform::from_xyz(0.034, 0., -0.007).with_scale(Vec3::new(0.5, 0.5, 1.));
            let counter_trans = Transform::from_xyz(0.016, 0., 0.004)
                .with_scale(Vec3::new(0.25, 0.25, 0.25))
                .with_rotation(Quat::from_rotation_y(1.05));

            p.spawn(tower_base_bundle(assets, mats));
            p.spawn(contact_light_bundle(g_sett, color));
            p.spawn(tower_sight_sensor_bundle(sight_radius));

            progress_bar::spawn(p, assets, mats, tower_id, bar_trans, color, 0.);
            analog_counter::spawn_2_digit(p, assets, counter_trans, Some(p.parent_entity()));

            add_to_tower(p);
        });
}

fn tower_material() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::BEIGE,
        perceptual_roughness: 0.6,
        metallic: 0.6,
        reflectance: 0.1,
        ..default()
    }
}

fn create_tower_spawn_animator(pos: Vec3) -> Sequence<Transform> {
    let delay = Delay::new(Duration::from_secs(1));
    let tween = Tween::new(
        EaseFunction::ExponentialInOut,
        std::time::Duration::from_secs(4),
        TransformPositionLens {
            start: tower_start_pos(pos),
            end: pos,
        },
    );
    delay.then(tween)
}

fn tower_start_pos(pos: Vec3) -> Vec3 {
    Vec3::new(pos.x, pos.y, pos.z - 0.1)
}

#[derive(Event)]
pub struct SpawnTowerEvent(pub TowerType, pub Vec3);

fn spawn_tower_system(
    mut cmds: Commands,
    mut evs: EventReader<SpawnTowerEvent>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut points_ev: EventWriter<PointsEvent>,
    assets: Res<PinballDefenseGltfAssets>,
    q_pbw: QueryWorld,
    g_sett: Res<GraphicsSettings>,
) {
    for ev in evs.iter() {
        cmds.entity(q_pbw.single()).with_children(|parent| {
            let pos = ev.1;
            match ev.0 {
                TowerType::Gun => gun::spawn(parent, &mut mats, &assets, &g_sett, pos),
                TowerType::Tesla => tesla::spawn(parent, &mut mats, &assets, &g_sett, pos),
                TowerType::Microwave => microwave::spawn(parent, &mut mats, &assets, &g_sett, pos),
            };
            points_ev.send(PointsEvent::TowerBuild);
        });
    }
}

fn progress_system(
    mut prog_bar_ev: EventWriter<ProgressBarCountUpEvent>,
    mut ball_coll_ev: EventReader<CollisionWithBallEvent>,
    mut points_ev: EventWriter<PointsEvent>,
    q_tower: Query<With<Tower>>,
) {
    ball_coll_ev
        .iter()
        .for_each(|CollisionWithBallEvent(id, flag)| {
            if *flag != CollisionEventFlags::SENSOR && q_tower.contains(*id) {
                prog_bar_ev.send(ProgressBarCountUpEvent(*id, 1.));
                points_ev.send(PointsEvent::TowerHit);
            }
        });
}

#[derive(Component, Default)]
struct TowerLevel(Level);

fn upgrade_system(
    mut cmds: Commands,
    mut upgrade_menu_exec_ev: EventReader<UpgradeMenuExecuteEvent>,
    mut q_light: Query<(Entity, &Parent, &mut Visibility), With<FlashLight>>,
    mut points_ev: EventWriter<PointsEvent>,
    mut prog_bar_ev: EventWriter<ProgressBarCountUpEvent>,
    mut q_tower: Query<&mut TowerLevel>,
    mut ac_set_ev: EventWriter<AnalogCounterSetEvent>,
    mut range_upgrade_ev: EventWriter<RangeUpgradeEvent>,
    mut damage_upgrade_ev: EventWriter<DamageUpgradeEvent>,
) {
    for ev in upgrade_menu_exec_ev.iter() {
        let mut tower_level = q_tower
            .get_mut(ev.tower_id)
            .unwrap_or_else(|_| panic!("😥 No tower level for id {:?} found", ev.tower_id));
        tower_level.0 += 1;
        disable_flash_light(&mut cmds, &mut q_light, ev.tower_id);
        ac_set_ev.send(AnalogCounterSetEvent::new(
            ev.tower_id,
            tower_level.0 as u32,
        ));
        points_ev.send(PointsEvent::TowerUpgrade);
        prog_bar_ev.send(ProgressBarCountUpEvent(ev.tower_id, -1.));
        match ev.upgrade {
            TowerUpgrade::Damage => damage_upgrade_ev.send(DamageUpgradeEvent(ev.tower_id)),
            TowerUpgrade::Range => range_upgrade_ev.send(RangeUpgradeEvent(ev.tower_id)),
        }
        log!(
            "🐱 Upgrade tower {:?} to level {:?}",
            ev.tower_id,
            tower_level.0
        );
    }
}

#[derive(Event)]
struct RangeUpgradeEvent(Entity);

fn range_upgrade_system(
    mut range_upgrade_ev: EventReader<RangeUpgradeEvent>,
    mut q_tower: Query<(Entity, &mut SightRadius), With<Tower>>,
    mut q_coll: Query<(&mut Transform, &Parent), With<TowerSightSensor>>,
) {
    for ev in range_upgrade_ev.iter() {
        if let Ok((tower_id, mut sight_radius)) = q_tower.get_mut(ev.0) {
            let upgrade_factor = 0.01;
            sight_radius.0 += upgrade_factor;
            q_coll
                .iter_mut()
                .find(|(_, parent)| parent.get() == tower_id)
                .expect("No tower sight radius for tower found")
                .0
                .scale += 0.1;
        }
    }
}

#[derive(Event)]
struct DamageUpgradeEvent(Entity);

fn damage_upgrade_system(
    mut damage_upgrade_ev: EventReader<DamageUpgradeEvent>,
    mut q_tower: Query<&mut DamageOverTime, With<Tower>>,
) {
    for ev in damage_upgrade_ev.iter() {
        if let Ok(mut damage) = q_tower.get_mut(ev.0) {
            damage.0 *= 2.;
        }
    }
}
