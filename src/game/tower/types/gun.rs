use super::animations::RotateToTarget;
use super::base::spawn_tower_base;
use super::target::{AimFirstEnemy, EnemiesWithinReach, SightRadius, TargetPos};
use super::{create_tower_spawn_animator, tower_material, tower_start_pos, Tower, TowerHead};
use crate::game::tower::target::TowerPos;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::utils::RelEntity;
use bevy_tweening::Animator;

#[derive(Component)]
pub struct GunTower;

#[derive(Component)]
pub struct GunTowerMount;

#[derive(Component)]
pub struct GunTowerHead;

#[derive(Component)]
pub struct GunTowerBarrel;

pub fn spawn(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    let tower_material = materials.add(tower_material());

    // Tower
    let sight_radius = 0.3;
    let mut tower = parent.spawn((
        spatial_from_pos(tower_start_pos(pos)),
        GunTower,
        Tower,
        Name::new(" Gun Tower"),
        SightRadius(sight_radius),
        AimFirstEnemy(None),
        EnemiesWithinReach::default(),
        TowerPos(pos),
        //TowerPos(Vec3::new(pos.x, 0., pos.y)),
        TargetPos(None),
        Animator::new(create_tower_spawn_animator(pos)),
    ));
    let rel_id = tower.id();

    // Children of tower
    let mg_barrel = |parent: &mut ChildBuilder| {
        parent.spawn((
            PbrBundle {
                mesh: assets.tower_mg_barrel.clone(),
                material: tower_material.clone(),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },
            GunTowerBarrel,
        ));
    };
    let mg_head = |parent: &mut ChildBuilder| {
        parent
            .spawn((
                PbrBundle {
                    mesh: assets.tower_mg_head.clone(),
                    material: tower_material.clone(),
                    transform: Transform::from_xyz(0., 0., 0.),
                    ..default()
                },
                GunTowerHead,
                //RotateToTarget::X,
                RelEntity(rel_id),
            ))
            .with_children(mg_barrel);
    };
    let mg_mounting = |parent: &mut ChildBuilder| {
        parent
            .spawn((
                PbrBundle {
                    mesh: assets.tower_mg_mounting.clone(),
                    material: tower_material.clone(),
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
            ))
            .with_children(mg_head);
    };

    tower.with_children(|parent| {
        spawn_tower_base(parent, materials, assets, g_sett, sight_radius);
        mg_mounting(parent);
    });
}
