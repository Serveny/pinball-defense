use super::animations::RotateAlways;
use super::{tower_material, TowerHead};
use crate::prelude::*;
use crate::settings::GraphicsSettings;

#[derive(Component)]
pub struct TeslaTower;

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
        (Name::new("Tesla Tower"), TeslaTower),
        |tower| {
            tower.spawn(top(tower_mat.clone(), assets));
        },
    );
}

fn top(material: Handle<StandardMaterial>, assets: &PinballDefenseAssets) -> impl Bundle {
    (
        PbrBundle {
            mesh: assets.tower_tesla_top.clone(),
            material,
            transform: Transform::from_xyz(0., 0.02, 0.),
            ..default()
        },
        TowerHead,
        RotateAlways,
    )
}
