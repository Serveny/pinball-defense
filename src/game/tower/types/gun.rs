use super::animations::RotateToTarget;
use super::target::AimFirstEnemy;
use crate::game::tower::damage::DamageOverTime;
use crate::game::tower::fx::GunFiringEffects;
use crate::game::tower::{ShotLight, TowerHead, TowerReady};
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::utils::RelEntity;
use bevy_hanabi::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct GunTower;

#[derive(Component)]
pub struct GunTowerMount;

#[derive(Component)]
pub struct GunTowerHead;

#[derive(Component)]
pub struct GunTowerBarrel;

pub const MG_BARREL_AXIS_Z: f32 = 0.0496;
pub const MG_MUZZLE_LOCAL: Vec3 = Vec3::new(0., 0.087, 0.0535);

pub fn spawn(
    pb_world: &mut ChildSpawnerCommands,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) -> Entity {
    let sight_radius = 0.3;

    // Tower
    super::spawn(
        pb_world,
        mats,
        assets,
        g_sett,
        pos,
        sight_radius,
        (
            Name::new(" Gun Tower"),
            GunTower,
            AimFirstEnemy(None),
            DamageOverTime(100.),
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
    tower
        .spawn(mounting(assets, rel_id))
        .with_children(|mounting| {
            mounting.spawn(trim(assets, rel_id));
            mounting.spawn(head(assets, rel_id)).with_children(|head| {
                head.spawn(barrels(assets, rel_id));
                head.spawn(muzzle_flash_light(g_sett, rel_id, sight_radius));
            });
        });
}

#[derive(Component)]
pub struct MuzzleFlashLight;

fn muzzle_flash_light(g_sett: &GraphicsSettings, rel_id: Entity, range: f32) -> impl Bundle {
    (
        Name::new("Muzzle Flash"),
        SpotLight {
            intensity: 0., // lumens - roughly a 100W non-halogen incandescent bulb
            color: Color::srgba_u8(215, 205, 117, 255),
            shadow_maps_enabled: g_sett.is_shadows,
            range,
            inner_angle: 0.02,
            outer_angle: 0.8,
            ..default()
        },
        Transform::from_translation(MG_MUZZLE_LOCAL).looking_at(MG_MUZZLE_LOCAL + Vec3::Y, Vec3::Z),
        Visibility::Hidden,
        MuzzleFlashLight,
        ShotLight,
        RelEntity(rel_id),
    )
}

fn mounting(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Mounting"),
        Mesh3d(assets.mg_tower_mounting.clone()),
        MeshMaterial3d(assets.mg_tower_dark_iron_material.clone()),
        Transform::from_xyz(0., 0., 0.),
        GunTowerMount,
        RotateToTarget,
        RelEntity(rel_id),
        TowerHead,
    )
}

fn head(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Head"),
        Mesh3d(assets.mg_tower_head.clone()),
        MeshMaterial3d(assets.mg_tower_copper_material.clone()),
        Transform::from_xyz(0., 0., 0.),
        GunTowerHead,
        RelEntity(rel_id),
    )
}

fn trim(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Trim"),
        Mesh3d(assets.mg_tower_trim.clone()),
        MeshMaterial3d(assets.mg_tower_brass_material.clone()),
        Transform::from_xyz(0., 0., 0.),
        RelEntity(rel_id),
    )
}

fn barrels(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Barrels"),
        Mesh3d(assets.mg_tower_barrels.clone()),
        MeshMaterial3d(assets.mg_tower_steel_material.clone()),
        Transform::from_xyz(0., 0., MG_BARREL_AXIS_Z),
        GunTowerBarrel,
        RelEntity(rel_id),
    )
}

pub(in super::super) fn shoot_animation_system(
    time: Res<Time>,
    q_gun_tower: Query<(Entity, &AimFirstEnemy), (With<GunTower>, With<TowerReady>)>,
    mut q_barrels: Query<(&mut Transform, &RelEntity), With<GunTowerBarrel>>,
    mut q_muzzle_flash: Query<
        (&mut Visibility, &mut SpotLight, &RelEntity),
        With<MuzzleFlashLight>,
    >,
    mut q_effects: Query<(&mut EffectSpawner, &RelEntity), With<GunFiringEffects>>,
) {
    for (tower_id, enemy_id) in q_gun_tower.iter() {
        let firing = enemy_id.0.is_some();
        if let Some((mut barrels, _)) = get_barrels(&mut q_barrels, tower_id) {
            if firing {
                barrels.rotate_y(time.delta_secs() * 36.);
            } else if barrels.rotation != Quat::IDENTITY {
                barrels.rotation = Quat::IDENTITY;
            }
        }
        if let Some(mut flash) = get_flash(&mut q_muzzle_flash, tower_id) {
            if firing {
                let sin = (time.elapsed_secs() * 64.).sin();
                *flash.0 = Visibility::Inherited;
                flash.1.intensity = (sin + 1.) * 32.;
            } else if *flash.0 != Visibility::Hidden {
                *flash.0 = Visibility::Hidden;
            }
        }
        for (mut spawner, rel_id) in q_effects.iter_mut() {
            if rel_id.0 != tower_id {
                continue;
            }
            if spawner.active != firing {
                spawner.active = firing;
            }
        }
    }
}

fn get_barrels<'a>(
    q_barrels: &'a mut Query<(&mut Transform, &RelEntity), With<GunTowerBarrel>>,
    tower_id: Entity,
) -> Option<(Mut<'a, Transform>, &'a RelEntity)> {
    q_barrels
        .iter_mut()
        .find(|(_, rel_id)| rel_id.0 == tower_id)
}

fn get_flash<'a>(
    q_muzzle_flash: &'a mut Query<
        (&mut Visibility, &mut SpotLight, &RelEntity),
        With<MuzzleFlashLight>,
    >,
    tower_id: Entity,
) -> Option<(Mut<'a, Visibility>, Mut<'a, SpotLight>, &'a RelEntity)> {
    q_muzzle_flash
        .iter_mut()
        .find(|(_, _, rel_id)| rel_id.0 == tower_id)
}
