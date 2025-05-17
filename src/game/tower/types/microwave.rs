use super::{tower_material, TowerHead};
use crate::game::tower::animations::RotateToTarget;
use crate::game::tower::speed::SlowDownFactor;
use crate::game::tower::target::AimFirstEnemy;
use crate::game::tower::ShotLight;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::utils::RelEntity;
use bevy::color::palettes::css::ORANGE_RED;

#[derive(Component)]
pub struct MicrowaveTower;

pub fn spawn(
    pb_world: &mut ChildSpawnerCommands,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    let sight_radius = 0.3;
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
            SlowDownFactor(0.5),
        ),
        |tower| {
            let rel_id = tower.target_entity();
            tower
                .spawn(head(tower_mat.clone(), assets, rel_id))
                .with_children(|head| {
                    head.spawn(slow_down_flash_light(g_sett, rel_id, sight_radius));
                });
        },
    );
}

fn head(
    material: Handle<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    rel_id: Entity,
) -> impl Bundle {
    (
        TowerHead,
        Mesh3d(assets.tower_microwave_top.clone()),
        MeshMaterial3d(material),
        Transform::from_xyz(0., 0., 0.04),
        RotateToTarget,
        RelEntity(rel_id),
    )
}

#[derive(Component)]
pub struct SlowDownFlashLight;

fn slow_down_flash_light(g_sett: &GraphicsSettings, rel_id: Entity, range: f32) -> impl Bundle {
    (
        Name::new("Slow Down Flash"),
        SpotLight {
            intensity: 0.,
            color: ORANGE_RED.into(),
            shadows_enabled: g_sett.is_shadows,
            range,
            inner_angle: 0.02,
            outer_angle: 0.8,
            ..default()
        },
        Transform::from_xyz(0., 0.04, 0.).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Z),
        Visibility::Hidden,
        SlowDownFlashLight,
        ShotLight,
        RelEntity(rel_id),
    )
}

pub(in super::super) fn shot_animation_system(
    time: Res<Time>,
    q_gun_tower: Query<(Entity, &AimFirstEnemy), With<MicrowaveTower>>,
    mut q_slow_flash: Query<
        (&mut Visibility, &mut SpotLight, &RelEntity),
        With<SlowDownFlashLight>,
    >,
) {
    for (tower_id, enemy_id) in q_gun_tower.iter() {
        let mut flash = get_flash(&mut q_slow_flash, tower_id);
        match enemy_id.0 {
            Some(_) => {
                let sin = (time.elapsed_secs() * 16.).sin();
                *flash.0 = Visibility::Inherited;
                flash.1.intensity = (sin + 1.) * 32.;
            }
            None => {
                if *flash.0 != Visibility::Hidden {
                    *flash.0 = Visibility::Hidden;
                }
            }
        }
    }
}

fn get_flash<'a>(
    q_muzzle_flash: &'a mut Query<
        (&mut Visibility, &mut SpotLight, &RelEntity),
        With<SlowDownFlashLight>,
    >,
    tower_id: Entity,
) -> (Mut<'a, Visibility>, Mut<'a, SpotLight>, &'a RelEntity) {
    q_muzzle_flash
        .iter_mut()
        .find(|(_, _, rel_id)| rel_id.0 == tower_id)
        .expect("No muzzle flash for tower found")
}
