use super::animations::RotateToTarget;
use super::target::{AimFirstEnemy, ConeAim, ConeFov, TargetPos};
use crate::game::tower::speed::SlowDownFactor;
use crate::game::tower::{ShotLight, TowerHead, TowerReady};
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::utils::RelEntity;
use bevy::color::palettes::css::ORANGE_RED;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MicrowaveTower;

const MW_DISH_PIVOT: Vec3 = Vec3::new(0., 0.07573, 0.08419);
const MW_EMITTER_LOCAL: Vec3 = Vec3::new(0., 0.10362, 0.09272);
const MW_DISH_AXIS: Vec3 = Vec3::new(0., 0.9563, 0.2924);
const MW_DISH_MIN_ELEVATION: f32 = -50_f32.to_radians();
const MW_DISH_MAX_ELEVATION: f32 = 40_f32.to_radians();
const MW_CONE_FOV: f32 = 45_f32.to_radians();

pub fn spawn(
    pb_world: &mut ChildSpawnerCommands,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) -> Entity {
    let sight_radius = 0.3;
    super::spawn(
        pb_world,
        mats,
        assets,
        g_sett,
        pos,
        sight_radius,
        (
            Name::new("Microwave Tower"),
            MicrowaveTower,
            AimFirstEnemy(None),
            SlowDownFactor(0.5),
            ConeFov(MW_CONE_FOV),
        ),
        |tower| build_view(tower, assets, g_sett, sight_radius),
    )
}

pub(crate) fn build_view(
    tower: &mut ChildSpawnerCommands,
    assets: &PinballDefenseGltfAssets,
    g_sett: &GraphicsSettings,
    sight_radius: f32,
) {
    let rel_id = tower.target_entity();
    tower.spawn(boiler(assets, rel_id)).with_children(|boiler| {
        boiler.spawn(trim(assets, rel_id));
        boiler.spawn(pipes(assets, rel_id));
        boiler.spawn(gauges(assets, rel_id));
        boiler.spawn(mount(assets, rel_id));
        boiler.spawn(mount_brass(assets, rel_id));
        boiler.spawn(mount_copper(assets, rel_id));
        boiler
            .spawn((
                Name::new("Dish Pivot"),
                Transform::from_translation(MW_DISH_PIVOT),
                Visibility::default(),
                MicrowaveDishPivot,
                ConeAim(MW_DISH_AXIS),
                RelEntity(rel_id),
            ))
            .with_children(|pivot| {
                pivot.spawn(dish(assets, rel_id));
                pivot.spawn(dish_rim(assets, rel_id));
                pivot.spawn(emitter_core(assets, rel_id));
                pivot.spawn(slow_down_flash_light(g_sett, rel_id, sight_radius));
            });
    });
}

fn boiler(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Boiler"),
        TowerHead,
        Mesh3d(assets.mw_boiler.clone()),
        MeshMaterial3d(assets.microwave_dark_iron_material.clone()),
        Transform::from_xyz(0., 0., 0.04),
        RotateToTarget,
        RelEntity(rel_id),
    )
}

fn trim(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Trim"),
        Mesh3d(assets.mw_boiler_trim.clone()),
        MeshMaterial3d(assets.microwave_brass_material.clone()),
        Transform::IDENTITY,
        RelEntity(rel_id),
    )
}

fn pipes(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Pipes"),
        Mesh3d(assets.mw_pipes.clone()),
        MeshMaterial3d(assets.microwave_copper_material.clone()),
        Transform::IDENTITY,
        RelEntity(rel_id),
    )
}

fn gauges(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Gauges"),
        Mesh3d(assets.mw_gauges.clone()),
        MeshMaterial3d(assets.microwave_ivory_material.clone()),
        Transform::IDENTITY,
        RelEntity(rel_id),
    )
}

fn mount(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Dish Mount"),
        Mesh3d(assets.mw_mount_iron.clone()),
        MeshMaterial3d(assets.microwave_dark_iron_material.clone()),
        Transform::IDENTITY,
        RelEntity(rel_id),
    )
}

fn mount_brass(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Dish Mount Brass"),
        Mesh3d(assets.mw_mount_brass.clone()),
        MeshMaterial3d(assets.microwave_brass_material.clone()),
        Transform::IDENTITY,
        RelEntity(rel_id),
    )
}

fn mount_copper(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Dish Mount Copper"),
        Mesh3d(assets.mw_mount_copper.clone()),
        MeshMaterial3d(assets.microwave_copper_material.clone()),
        Transform::IDENTITY,
        RelEntity(rel_id),
    )
}

fn dish(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Dish"),
        Mesh3d(assets.mw_dish_assembly.clone()),
        MeshMaterial3d(assets.microwave_brass_material.clone()),
        Transform::from_translation(-MW_DISH_PIVOT),
        RelEntity(rel_id),
    )
}

fn dish_rim(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Dish Rim"),
        Mesh3d(assets.mw_dish_rim.clone()),
        MeshMaterial3d(assets.microwave_dark_iron_material.clone()),
        Transform::from_translation(-MW_DISH_PIVOT),
        RelEntity(rel_id),
    )
}

#[derive(Component)]
pub(in super::super) struct MicrowaveDishPivot;

#[derive(Component)]
pub struct MicrowaveEmitterCore;

fn emitter_core(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Emitter Core"),
        Mesh3d(assets.mw_emitter_core.clone()),
        MeshMaterial3d(assets.microwave_emitter_glow_material.clone()),
        Transform::from_translation(MW_EMITTER_LOCAL - MW_DISH_PIVOT),
        MicrowaveEmitterCore,
        RelEntity(rel_id),
    )
}

#[derive(Component)]
pub struct SlowDownFlashLight;

fn slow_down_flash_light(g_sett: &GraphicsSettings, rel_id: Entity, range: f32) -> impl Bundle {
    let emitter = MW_EMITTER_LOCAL - MW_DISH_PIVOT;
    (
        Name::new("Slow Down Flash"),
        SpotLight {
            intensity: 0.,
            color: ORANGE_RED.into(),
            shadow_maps_enabled: g_sett.is_shadows,
            range,
            inner_angle: 0.02,
            outer_angle: 0.8,
            ..default()
        },
        Transform::from_translation(emitter).looking_at(emitter + MW_DISH_AXIS, Vec3::Z),
        Visibility::Hidden,
        SlowDownFlashLight,
        ShotLight,
        RelEntity(rel_id),
    )
}

pub(in super::super) fn rotate_dish_to_target_system(
    mut q_pivot: Query<(&mut Transform, &GlobalTransform, &RelEntity), With<MicrowaveDishPivot>>,
    q_tower: Query<&TargetPos>,
) {
    for (mut transform, global, rel_id) in q_pivot.iter_mut() {
        if let Ok(TargetPos(Some(target))) = q_tower.get(rel_id.0) {
            let direction = *target - global.translation();
            if direction != Vec3::ZERO {
                transform.rotation = Quat::from_rotation_x(dish_pitch(direction).clamp(
                    MW_DISH_MIN_ELEVATION,
                    MW_DISH_MAX_ELEVATION,
                ));
            }
        }
    }
}

fn dish_pitch(direction: Vec3) -> f32 {
    direction.z.atan2(direction.truncate().length()) - MW_DISH_AXIS.z.atan2(MW_DISH_AXIS.y)
}

pub(in super::super) fn shot_animation_system(
    time: Res<Time>,
    q_gun_tower: Query<(Entity, &AimFirstEnemy, &ConeFov), (With<MicrowaveTower>, With<TowerReady>)>,
    mut q_slow_flash: Query<
        (&mut Visibility, &mut SpotLight, &RelEntity),
        With<SlowDownFlashLight>,
    >,
    mut q_core: Query<(&mut Transform, &RelEntity), With<MicrowaveEmitterCore>>,
) {
    for (tower_id, enemy_id, fov) in q_gun_tower.iter() {
        let firing = enemy_id.0.is_some();
        if let Some(mut flash) = get_flash(&mut q_slow_flash, tower_id) {
            if firing {
                let sin = (time.elapsed_secs() * 16.).sin();
                *flash.0 = Visibility::Inherited;
                flash.1.intensity = (sin + 1.) * 32.;
                flash.1.outer_angle = fov.0 / 2.;
                flash.1.inner_angle = fov.0 / 4.;
            } else if *flash.0 != Visibility::Hidden {
                *flash.0 = Visibility::Hidden;
            }
        }
        if let Some((mut core, _)) = get_core(&mut q_core, tower_id) {
            if firing {
                let pulse = 1. + 0.3 * (time.elapsed_secs() * 16.).sin();
                core.scale = Vec3::splat(pulse);
            } else if core.scale != Vec3::ONE {
                core.scale = Vec3::ONE;
            }
        }
    }
}

fn get_flash<'a>(
    q_muzzle_flash: &'a mut Query<
        (&mut Visibility, &mut SpotLight, &RelEntity),
        With<SlowDownFlashLight>,
    >,
    tower_id: Entity,
) -> Option<(Mut<'a, Visibility>, Mut<'a, SpotLight>, &'a RelEntity)> {
    q_muzzle_flash
        .iter_mut()
        .find(|(_, _, rel_id)| rel_id.0 == tower_id)
}

fn get_core<'a>(
    q_core: &'a mut Query<(&mut Transform, &RelEntity), With<MicrowaveEmitterCore>>,
    tower_id: Entity,
) -> Option<(Mut<'a, Transform>, &'a RelEntity)> {
    q_core.iter_mut().find(|(_, rel_id)| rel_id.0 == tower_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pitch_is_relative_to_the_modeled_dish_angle() {
        assert!(dish_pitch(MW_DISH_AXIS).abs() < 0.0001);
        assert!((dish_pitch(Vec3::Y) + 17_f32.to_radians()).abs() < 0.001);
    }

    #[test]
    #[allow(clippy::float_cmp, clippy::manual_assert_eq)]
    fn pitch_is_clamped() {
        let min = dish_pitch(Vec3::new(0., 0.1, -1.)).clamp(MW_DISH_MIN_ELEVATION, MW_DISH_MAX_ELEVATION);
        let max = dish_pitch(Vec3::new(0., 1., 2.)).clamp(MW_DISH_MIN_ELEVATION, MW_DISH_MAX_ELEVATION);
        assert!(min == MW_DISH_MIN_ELEVATION);
        assert!(max == MW_DISH_MAX_ELEVATION);
    }
}
