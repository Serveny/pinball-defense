use super::animations::RotateToTarget;
use super::base::spawn_tower_base;
use super::target::{AimFirstEnemy, SightRadius, TargetPos};
use super::{create_tower_spawn_animator, tower_material, tower_start_pos, Tower, TowerHead};
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use bevy_tweening::Animator;

#[derive(Component)]
pub struct MachineGunTower;

#[derive(Component)]
pub struct MachineGunTowerMount;

#[derive(Component)]
pub struct MachineGunTowerHead;

#[derive(Component)]
pub struct MachineGunTowerBarrel;

pub fn spawn_tower_machine_gun(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    let tower_material = materials.add(tower_material());
    let mg_barrel = |parent: &mut ChildBuilder| {
        parent.spawn((
            PbrBundle {
                mesh: assets.tower_mg_barrel.clone(),
                material: tower_material.clone(),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },
            MachineGunTowerBarrel,
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
                MachineGunTowerHead,
            ))
            .with_children(mg_barrel);
    };
    let mg_mounting = |parent: &mut ChildBuilder| {
        parent
            .spawn((
                PbrBundle {
                    mesh: assets.tower_mg_mounting.clone(),
                    material: tower_material.clone(),
                    transform: Transform::from_xyz(0., 0.023, 0.),
                    ..default()
                },
                MachineGunTowerMount,
                AimFirstEnemy(None),
                TargetPos(None),
                RotateToTarget,
                TowerHead,
            ))
            .with_children(mg_head);
    };
    let sight_radius = 0.15;
    parent
        .spawn((
            spatial_from_pos(tower_start_pos(pos)),
            MachineGunTower,
            Tower,
            SightRadius(sight_radius),
            Name::new("Machine Gun Tower"),
            Animator::new(create_tower_spawn_animator(pos)),
        ))
        .with_children(|parent| {
            spawn_tower_base(parent, materials, assets, g_sett, sight_radius);
            mg_mounting(parent);
        });
}
