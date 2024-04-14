use super::animations::RotateAlways;
use super::{tower_material, TowerHead};
use crate::game::tower::damage::{DamageAllTargetsInReach, DamageOverTime};
use crate::game::tower::target::EnemiesWithinReach;
use crate::game::tower::ShotLight;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::utils::RelEntity;

#[derive(Component)]
pub struct TeslaTower;

pub fn spawn(
    pb_world: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
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
            Name::new("Tesla Tower"),
            TeslaTower,
            DamageAllTargetsInReach,
            DamageOverTime(15.),
        ),
        |tower| {
            tower.spawn(top(tower_mat.clone(), assets));
            tower.spawn(shot_flash_light(
                g_sett,
                tower.parent_entity(),
                sight_radius,
            ));
        },
    );
}

fn top(material: Handle<StandardMaterial>, assets: &PinballDefenseGltfAssets) -> impl Bundle {
    (
        PbrBundle {
            mesh: assets.tower_tesla_top.clone(),
            material,
            transform: Transform::from_xyz(0., 0., 0.02),
            ..default()
        },
        TowerHead,
        RotateAlways,
    )
}

#[derive(Component)]
pub struct ShotFlashLight;

fn shot_flash_light(g_sett: &GraphicsSettings, rel_id: Entity, range: f32) -> impl Bundle {
    (
        Name::new("Shot Flash"),
        PointLightBundle {
            transform: Transform::from_xyz(0., 0., 0.1),
            point_light: PointLight {
                intensity: 0.,
                color: Color::BLUE,
                shadows_enabled: g_sett.is_shadows,
                range,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        ShotFlashLight,
        ShotLight,
        RelEntity(rel_id),
    )
}

pub(in super::super) fn shot_animation_system(
    time: Res<Time>,
    q_tesla: Query<(Entity, &EnemiesWithinReach), With<TeslaTower>>,
    mut q_shot_flash: Query<(&mut Visibility, &mut PointLight, &RelEntity), With<ShotFlashLight>>,
) {
    for (tower_id, ewr) in q_tesla.iter() {
        let mut flash = get_flash(&mut q_shot_flash, tower_id);
        match ewr.0.is_empty() {
            false => {
                let sin = (time.elapsed_seconds() * 32.).sin();
                *flash.0 = Visibility::Inherited;
                flash.1.intensity = (sin + 1.) * 32.;
            }
            true => {
                if *flash.0 != Visibility::Hidden {
                    *flash.0 = Visibility::Hidden;
                }
            }
        }
    }
}

fn get_flash<'a>(
    q_muzzle_flash: &'a mut Query<
        (&mut Visibility, &mut PointLight, &RelEntity),
        With<ShotFlashLight>,
    >,
    tower_id: Entity,
) -> (Mut<'a, Visibility>, Mut<'a, PointLight>, &'a RelEntity) {
    q_muzzle_flash
        .iter_mut()
        .find(|(_, _, rel_id)| rel_id.0 == tower_id)
        .expect("No muzzle flash for tower found")
}
