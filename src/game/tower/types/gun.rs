use super::animations::RotateToTarget;
use super::target::AimFirstEnemy;
use crate::game::tower::{tower_material, TowerHead};
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
    assets: &PinballDefenseAssets,
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
        (Name::new(" Gun Tower"), GunTower, AimFirstEnemy(None)),
        |tower| {
            let rel_id = tower.parent_entity();

            // Children of tower
            let mg_barrel = |parent: &mut ChildBuilder| {
                parent.spawn(barrel(tower_mat.clone(), assets));
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

fn barrel(tower_mat: Handle<StandardMaterial>, assets: &PinballDefenseAssets) -> impl Bundle {
    (
        PbrBundle {
            mesh: assets.tower_mg_barrel.clone(),
            material: tower_mat,
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        GunTowerBarrel,
    )
}

fn head(
    tower_mat: Handle<StandardMaterial>,
    assets: &PinballDefenseAssets,
    rel_id: Entity,
) -> impl Bundle {
    (
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
    assets: &PinballDefenseAssets,
    rel_id: Entity,
) -> impl Bundle {
    (
        PbrBundle {
            mesh: assets.tower_mg_mounting.clone(),
            material: tower_mat.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0.023, 0.),
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
