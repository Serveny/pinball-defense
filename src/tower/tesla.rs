use super::base::spawn_tower_base;
use super::target::{AimFirstEnemy, SightRadius};
use super::{create_tower_spawn_animator, tower_material, tower_start_pos, Tower, TowerHead};
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use bevy_tweening::Animator;

#[derive(Component)]
pub struct TeslaTower;

pub fn spawn_tower_tesla(
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
            TeslaTower,
            AimFirstEnemy,
            SightRadius(0.1),
            Name::new("Tesla Tower"),
            Animator::new(create_tower_spawn_animator(pos)),
        ))
        .with_children(|parent| {
            spawn_tower_base(parent, materials, assets, g_sett, sight_radius);
            parent.spawn((
                PbrBundle {
                    mesh: assets.tower_tesla_top.clone(),
                    material: materials.add(tower_material()),
                    transform: Transform::from_xyz(0., 0.02, 0.),
                    ..default()
                },
                TowerHead,
            ));
        });
}
