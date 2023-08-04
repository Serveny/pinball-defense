use super::base::spawn_tower_base;
use super::{create_tower_spawn_animator, tower_material, tower_start_pos, TowerHead};
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
        parent
            .spawn(PbrBundle {
                mesh: assets.tower_mg_barrel.clone(),
                material: tower_material.clone(),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            })
            .insert(MachineGunTowerBarrel);
    };
    let mg_head = |parent: &mut ChildBuilder| {
        parent
            .spawn(PbrBundle {
                mesh: assets.tower_mg_head.clone(),
                material: tower_material.clone(),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            })
            .insert(MachineGunTowerHead)
            .with_children(|parent| mg_barrel(parent));
    };
    let mg_mounting = |parent: &mut ChildBuilder| {
        parent
            .spawn(PbrBundle {
                mesh: assets.tower_mg_mounting.clone(),
                material: tower_material.clone(),
                transform: Transform::from_xyz(0., 0.023, 0.),
                ..default()
            })
            .insert(MachineGunTowerMount)
            .insert(TowerHead)
            .with_children(|parent| mg_head(parent));
    };
    parent
        .spawn(spatial_from_pos(tower_start_pos(pos)))
        .insert(MachineGunTower)
        .insert(Name::new("Machine Gun Tower"))
        .insert(Animator::new(create_tower_spawn_animator(pos)))
        .with_children(|parent| {
            spawn_tower_base(parent, materials, assets, g_sett);
            mg_mounting(parent);
        });
}
