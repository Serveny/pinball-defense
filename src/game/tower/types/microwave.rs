use super::base::spawn_tower_base;
use super::target::{EnemiesWithinReach, SightRadius};
use super::{create_tower_spawn_animator, tower_material, tower_start_pos, Tower, TowerHead};
use crate::game::tower::animations::RotateToTarget;
use crate::game::tower::target::{AimFirstEnemy, TargetPos, TowerPos};
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::utils::RelEntity;
use bevy_tweening::Animator;

#[derive(Component)]
pub struct MicrowaveTower;

pub fn spawn(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    let sight_radius = 0.15;
    parent
        .spawn((
            spatial_from_pos(tower_start_pos(pos)),
            Tower,
            MicrowaveTower,
            SightRadius(0.1),
            EnemiesWithinReach::default(),
            AimFirstEnemy(None),
            TowerPos(pos),
            TargetPos(None),
            Name::new("Microwave Tower"),
            Animator::new(create_tower_spawn_animator(pos)),
        ))
        .with_children(|parent| {
            spawn_tower_base(parent, materials, assets, g_sett, sight_radius);
            parent.spawn((
                PbrBundle {
                    mesh: assets.tower_microwave_top.clone(),
                    material: materials.add(tower_material()),
                    transform: Transform::from_xyz(0., 0.04, 0.),
                    ..default()
                },
                RotateToTarget,
                RelEntity(parent.parent_entity()),
                TowerHead,
            ));
        });
}
