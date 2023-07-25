use super::{
    create_tower_spawn_animator, spawn_tower_base, tower_material, tower_start_pos, TowerHead,
};
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use bevy_tweening::Animator;

#[derive(Component)]
pub struct MicrowaveTower;

pub fn spawn_tower_microwave(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    parent
        .spawn(spatial_from_pos(tower_start_pos(pos)))
        .insert(MicrowaveTower)
        .insert(Name::new("Microwave Tower"))
        .insert(Animator::new(create_tower_spawn_animator(pos)))
        .with_children(|parent| {
            spawn_tower_base(parent, materials, assets, g_sett);
            parent
                .spawn(PbrBundle {
                    mesh: assets.tower_microwave_top.clone(),
                    material: materials.add(tower_material()),
                    transform: Transform::from_xyz(0., 0.04, 0.),
                    ..default()
                })
                .insert(TowerHead);
        });
}
