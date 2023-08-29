use super::{tower_material, TowerHead};
use crate::game::tower::animations::RotateToTarget;
use crate::game::tower::speed::SlowDownFactor;
use crate::game::tower::target::AimFirstEnemy;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::utils::RelEntity;

#[derive(Component)]
pub struct MicrowaveTower;

pub fn spawn(
    pb_world: &mut ChildBuilder,
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
            let rel_id = tower.parent_entity();
            tower
                .spawn(head(tower_mat.clone(), assets, rel_id))
                .with_children(|head| {
                    head.spawn(slow_down_flash_light(g_sett, rel_id));
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
        PbrBundle {
            mesh: assets.tower_microwave_top.clone(),
            material,
            transform: Transform::from_xyz(0., 0., 0.04),
            ..default()
        },
        RotateToTarget,
        RelEntity(rel_id),
        TowerHead,
    )
}

#[derive(Component)]
pub struct SlowDownFlashLight;

fn slow_down_flash_light(g_sett: &GraphicsSettings, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Slow Down Flash"),
        SpotLightBundle {
            transform: Transform::from_xyz(0., 0.04, 0.)
                .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Z),
            spot_light: SpotLight {
                intensity: 0., // lumens - roughly a 100W non-halogen incandescent bulb
                color: Color::ORANGE_RED,
                shadows_enabled: g_sett.is_shadows,
                range: 0.4,
                inner_angle: 0.02,
                outer_angle: 0.8,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        SlowDownFlashLight,
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
                let sin = (time.elapsed_seconds() * 16.).sin();
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
