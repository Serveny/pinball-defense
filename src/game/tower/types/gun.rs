use super::animations::RotateToTarget;
use super::target::AimFirstEnemy;
use crate::game::tower::damage::DamageOverTime;
use crate::game::tower::{tower_material, ShotLight, TowerHead};
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::utils::RelEntity;

#[derive(Component)]
pub struct GunTower;

#[derive(Component)]
pub struct GunTowerMount;

#[derive(Component)]
pub struct GunTowerHead;

#[derive(Component)]
pub struct GunTowerBarrel;

pub fn spawn(
    pb_world: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    let sight_radius = 0.3;
    let tower_mat = mats.add(tower_material());

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
            DamageOverTime(10.),
        ),
        |tower| {
            let rel_id = tower.parent_entity();

            // Children of tower
            let muzzle_flash_light = |parent: &mut ChildBuilder| {
                parent.spawn(muzzle_flash_light(g_sett, rel_id, sight_radius));
            };
            let mg_barrel = |parent: &mut ChildBuilder| {
                parent
                    .spawn(barrel(tower_mat.clone(), assets, rel_id))
                    .with_children(muzzle_flash_light);
            };
            let mg_head = |parent: &mut ChildBuilder| {
                parent
                    .spawn(head(tower_mat.clone(), assets, rel_id))
                    .with_children(mg_barrel);
            };
            let mg_mounting = |parent: &mut ChildBuilder| {
                parent
                    .spawn(mounting(tower_mat.clone(), assets, rel_id))
                    .with_children(mg_head);
            };

            mg_mounting(tower);
        },
    );
}

#[derive(Component)]
pub struct MuzzleFlashLight;

fn muzzle_flash_light(g_sett: &GraphicsSettings, rel_id: Entity, range: f32) -> impl Bundle {
    (
        Name::new("Muzzle Flash"),
        SpotLightBundle {
            transform: Transform::from_xyz(0., 0.04, 0.)
                .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Z),
            spot_light: SpotLight {
                intensity: 0., // lumens - roughly a 100W non-halogen incandescent bulb
                color: Color::rgba_u8(215, 205, 117, 255),
                shadows_enabled: g_sett.is_shadows,
                range,
                inner_angle: 0.02,
                outer_angle: 0.8,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        MuzzleFlashLight,
        ShotLight,
        RelEntity(rel_id),
    )
}

fn barrel(
    tower_mat: Handle<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    rel_id: Entity,
) -> impl Bundle {
    (
        Name::new("Barrel"),
        PbrBundle {
            mesh: assets.tower_mg_barrel.clone(),
            material: tower_mat,
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        GunTowerBarrel,
        RelEntity(rel_id),
    )
}

fn head(
    tower_mat: Handle<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    rel_id: Entity,
) -> impl Bundle {
    (
        Name::new("Head"),
        PbrBundle {
            mesh: assets.tower_mg_head.clone(),
            material: tower_mat,
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        GunTowerHead,
        //RotateToTarget::X,
        RelEntity(rel_id),
    )
}

fn mounting(
    tower_mat: Handle<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    rel_id: Entity,
) -> impl Bundle {
    (
        Name::new("Mounting"),
        PbrBundle {
            mesh: assets.tower_mg_mounting.clone(),
            material: tower_mat.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.023),
                scale: Vec3::new(0.9, 0.9, 0.9),
                ..default()
            },
            ..default()
        },
        GunTowerMount,
        RotateToTarget,
        RelEntity(rel_id),
        TowerHead,
    )
}

pub(in super::super) fn shoot_animation_system(
    time: Res<Time>,
    q_gun_tower: Query<(Entity, &AimFirstEnemy), With<GunTower>>,
    mut q_barrel: Query<(&mut Transform, &RelEntity), With<GunTowerBarrel>>,
    mut q_muzzle_flash: Query<
        (&mut Visibility, &mut SpotLight, &RelEntity),
        With<MuzzleFlashLight>,
    >,
) {
    for (tower_id, enemy_id) in q_gun_tower.iter() {
        let mut flash = get_flash(&mut q_muzzle_flash, tower_id);
        match enemy_id.0 {
            Some(_) => {
                let sin = (time.elapsed_seconds() * 64.).sin();
                *flash.0 = Visibility::Inherited;
                get_barrel(&mut q_barrel, tower_id).0.translation.y = sin * 0.002;
                flash.1.intensity = (sin + 1.) * 32.;
            }
            None => {
                if *flash.0 != Visibility::Hidden {
                    *flash.0 = Visibility::Hidden;
                    get_barrel(&mut q_barrel, tower_id).0.translation.y = 0.;
                }
            }
        }
    }
}

fn get_barrel<'a>(
    q_barrel: &'a mut Query<(&mut Transform, &RelEntity), With<GunTowerBarrel>>,
    tower_id: Entity,
) -> (Mut<'a, Transform>, &'a RelEntity) {
    q_barrel
        .iter_mut()
        .find(|(_, rel_id)| rel_id.0 == tower_id)
        .expect("No barrel for tower found")
}
fn get_flash<'a>(
    q_muzzle_flash: &'a mut Query<
        (&mut Visibility, &mut SpotLight, &RelEntity),
        With<MuzzleFlashLight>,
    >,
    tower_id: Entity,
) -> (Mut<'a, Visibility>, Mut<'a, SpotLight>, &'a RelEntity) {
    q_muzzle_flash
        .iter_mut()
        .find(|(_, _, rel_id)| rel_id.0 == tower_id)
        .expect("No muzzle flash for tower found")
}
