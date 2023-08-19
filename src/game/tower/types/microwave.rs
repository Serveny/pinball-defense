use super::{tower_material, TowerHead};
use crate::game::tower::animations::RotateToTarget;
use crate::game::tower::target::AimFirstEnemy;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::utils::RelEntity;

#[derive(Component)]
pub struct MicrowaveTower;

pub fn spawn(
    pb_world: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    let sight_radius = 0.15;
    let tower_mat = mats.add(tower_material());
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
        ),
        |tower| {
            tower.spawn(head(tower_mat.clone(), assets, tower.parent_entity()));
        },
    );
}

fn head(
    material: Handle<StandardMaterial>,
    assets: &PinballDefenseAssets,
    rel_id: Entity,
) -> impl Bundle {
    (
        PbrBundle {
            mesh: assets.tower_microwave_top.clone(),
            material,
            transform: Transform::from_xyz(0., 0.04, 0.),
            ..default()
        },
        RotateToTarget,
        RelEntity(rel_id),
        TowerHead,
    )
}
