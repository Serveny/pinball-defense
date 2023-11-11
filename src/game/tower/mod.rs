use self::damage::DamageOverTime;
use self::speed::SlowDownFactor;
use self::target::{EnemiesWithinReach, SightRadius, TargetPos};
use super::audio::SoundEvent;
use super::ball::CollisionWithBallEvent;
use super::cfg::CONFIG;
use super::events::collision::{COLLIDE_ONLY_WITH_BALL, COLLIDE_ONLY_WITH_ENEMY};
use super::level::{Level, PointsEvent};
use super::light::{
    contact_light_bundle, sight_radius_light, FlashLight, LightOnCollision, SightRadiusLight,
};
use super::pinball_menu::{PinballMenuTrigger, UpgradeMenuExecuteEvent};
use super::progress_bar::{self, ProgressBarCountUpEvent};
use super::{EventState, GameState};
use crate::game::analog_counter::AnalogCounterSetEvent;
use crate::game::light::disable_flash_light;
use crate::game::world::QueryWorld;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::utils::RelEntity;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::{Animator, Delay, EaseFunction, Sequence, Tween};
use std::time::Duration;
pub use types::TowerType;
use types::*;

mod animations;
mod damage;
pub mod foundation;
mod speed;
mod target;
mod types;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTowerEvent>()
            .add_event::<DamageUpgradeEvent>()
            .add_event::<RangeUpgradeEvent>()
            .add_systems(
                Update,
                (
                    animations::rotate_always_system,
                    animations::rotate_to_target_system,
                    damage::afe_damage_over_time_system,
                    damage::datir_damage_over_time_system,
                    speed::afe_slow_down_system,
                    target::aim_first_enemy_system,
                    target::target_pos_by_afe_system,
                    types::gun::shoot_animation_system,
                    types::microwave::shot_animation_system,
                    types::tesla::shot_animation_system,
                )
                    .run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (
                    on_progress_system,
                    on_spawn_tower_system,
                    on_upgrade_system,
                    on_damage_upgrade_system,
                    on_range_upgrade_system,
                    foundation::on_spawn_system,
                    foundation::on_despawn_system,
                    foundation::on_progress_system,
                    target::on_enemy_within_reach_system,
                    target::on_remove_despawned_enemies_from_ewr_system,
                )
                    .run_if(in_state(EventState::Active)),
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
            let color = Color::rgb_u8(115, 27, 7);
            let bar_trans =
                Transform::from_xyz(0.034, 0., -0.007).with_scale(Vec3::new(0.5, 0.5, 1.));
            p.spawn(tower_base_bundle(assets, mats));
            p.spawn(contact_light_bundle(g_sett, color));
            p.spawn(tower_sight_sensor_bundle(sight_radius));
            p.spawn(sight_radius_light(sight_radius));
            progress_bar::spawn(p, assets, mats, tower_id, bar_trans, color, 0.);
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

fn on_spawn_tower_system(
    mut cmds: Commands,
    mut evr: EventReader<SpawnTowerEvent>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut points_ev: EventWriter<PointsEvent>,
    mut sound_ev: EventWriter<SoundEvent>,
    assets: Res<PinballDefenseGltfAssets>,
    q_pbw: QueryWorld,
    g_sett: Res<GraphicsSettings>,
) {
    for ev in evr.read() {
        cmds.entity(q_pbw.single()).with_children(|parent| {
            let pos = ev.1;
            match ev.0 {
                TowerType::Gun => gun::spawn(parent, &mut mats, &assets, &g_sett, pos),
                TowerType::Tesla => tesla::spawn(parent, &mut mats, &assets, &g_sett, pos),
                TowerType::Microwave => microwave::spawn(parent, &mut mats, &assets, &g_sett, pos),
            };
            points_ev.send(PointsEvent::TowerBuild);
            sound_ev.send(SoundEvent::TowerBuild);
        });
    }
}

fn on_progress_system(
    mut prog_bar_ev: EventWriter<ProgressBarCountUpEvent>,
    mut evr: EventReader<CollisionWithBallEvent>,
    mut points_ev: EventWriter<PointsEvent>,
    mut sound_ev: EventWriter<SoundEvent>,
    q_tower: Query<With<Tower>>,
) {
    evr.read().for_each(|CollisionWithBallEvent(id, flag)| {
        if *flag != CollisionEventFlags::SENSOR && q_tower.contains(*id) {
            prog_bar_ev.send(ProgressBarCountUpEvent::new(*id, CONFIG.tower_hit_progress));
            points_ev.send(PointsEvent::TowerHit);
            sound_ev.send(SoundEvent::TowerHit);
        }
    });
}

#[derive(Component, Default)]
struct TowerLevel(Level);

impl SoundEvent {
    fn upgrade_sound(upgrade: TowerUpgrade) -> Self {
        match upgrade {
            TowerUpgrade::Damage => Self::TowerUpgradeDamage,
            TowerUpgrade::Range => Self::TowerUpgradeRange,
        }
    }
}

fn on_upgrade_system(
    mut cmds: Commands,
    mut evr: EventReader<UpgradeMenuExecuteEvent>,
    mut q_light: Query<(Entity, &Parent, &mut Visibility), With<FlashLight>>,
    mut points_ev: EventWriter<PointsEvent>,
    mut prog_bar_ev: EventWriter<ProgressBarCountUpEvent>,
    mut q_tower: Query<&mut TowerLevel>,
    mut ac_set_ev: EventWriter<AnalogCounterSetEvent>,
    mut range_upgrade_ev: EventWriter<RangeUpgradeEvent>,
    mut damage_upgrade_ev: EventWriter<DamageUpgradeEvent>,
    mut sound_ev: EventWriter<SoundEvent>,
) {
    for ev in evr.read() {
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
        prog_bar_ev.send(ProgressBarCountUpEvent::new(ev.tower_id, -1.));
        match ev.upgrade {
            TowerUpgrade::Damage => damage_upgrade_ev.send(DamageUpgradeEvent(ev.tower_id)),
            TowerUpgrade::Range => range_upgrade_ev.send(RangeUpgradeEvent(ev.tower_id)),
        }
        sound_ev.send(SoundEvent::upgrade_sound(ev.upgrade));
        log!(
            "🐱 Upgrade tower {:?} to level {:?}",
            ev.tower_id,
            tower_level.0
        );
    }
}

#[derive(Event)]
struct RangeUpgradeEvent(Entity);

type QShotLight<'w, 's, 'a> = Query<
    'w,
    's,
    (
        Option<&'a mut SpotLight>,
        Option<&'a mut PointLight>,
        &'a RelEntity,
    ),
    (With<ShotLight>, Without<SightRadiusLight>),
>;

fn on_range_upgrade_system(
    mut evr: EventReader<RangeUpgradeEvent>,
    mut q_tower: Query<(Entity, &mut SightRadius), With<Tower>>,
    mut q_coll: Query<(&mut Transform, &Parent), With<TowerSightSensor>>,
    mut q_sr_light: Query<(&mut SpotLight, &Parent), With<SightRadiusLight>>,
    mut q_shot_light: QShotLight,
) {
    for ev in evr.read() {
        if let Ok((tower_id, mut sight_radius)) = q_tower.get_mut(ev.0) {
            sight_radius.0 += CONFIG.range_upgade_factor;
            update_collider_size(&mut q_coll, CONFIG.range_upgade_factor, tower_id);
            update_sight_radius_light_size(&mut q_sr_light, sight_radius.0, tower_id);
            update_shot_light_size(&mut q_shot_light, sight_radius.0, tower_id);
        }
    }
}

fn update_collider_size(
    q_coll: &mut Query<(&mut Transform, &Parent), With<TowerSightSensor>>,
    upgrade_factor: f32,
    tower_id: Entity,
) {
    q_coll
        .iter_mut()
        .find(|(_, parent)| parent.get() == tower_id)
        .expect("No tower sight radius for tower found")
        .0
        .scale += upgrade_factor;
}

fn update_sight_radius_light_size(
    q_sr_light: &mut Query<(&mut SpotLight, &Parent), With<SightRadiusLight>>,
    sight_radius: f32,
    tower_id: Entity,
) {
    let (mut light, _) = q_sr_light
        .iter_mut()
        .find(|(_, parent)| parent.get() == tower_id)
        .expect("No tower sight radius light for tower found");
    light.inner_angle = sight_radius;
    light.outer_angle = sight_radius;
}

fn update_shot_light_size(q_shot_light: &mut QShotLight, sight_radius: f32, tower_id: Entity) {
    let (spot, point, _) = q_shot_light
        .iter_mut()
        .find(|(_, _, rel_id)| rel_id.0 == tower_id)
        .expect("No shot light for tower found");

    if let Some(mut light) = spot {
        light.outer_angle = sight_radius;
    } else if let Some(mut light) = point {
        light.range = sight_radius;
    }
}

#[derive(Event)]
struct DamageUpgradeEvent(Entity);

fn on_damage_upgrade_system(
    mut evr: EventReader<DamageUpgradeEvent>,
    mut q_tower: Query<(Option<&mut DamageOverTime>, Option<&mut SlowDownFactor>), With<Tower>>,
) {
    for ev in evr.read() {
        if let Ok((dmg_over_time, slow_down_factor)) = q_tower.get_mut(ev.0) {
            if let Some(mut dmg_over_time) = dmg_over_time {
                dmg_over_time.0 *= CONFIG.damage_upgrade_factor;
            }
            if let Some(mut slow_down_factor) = slow_down_factor {
                slow_down_factor.0 *= 0.98;
            }
        }
    }
}

#[derive(Component)]
struct ShotLight;
