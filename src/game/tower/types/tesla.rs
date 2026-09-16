use super::TowerHead;
use super::animations::RotateAlways;
use crate::game::tower::damage::DamageOverTime;
use crate::game::tower::target::{TargetAllInReach, Targets};
use crate::game::tower::{ShotLight, TowerReady};
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::utils::RelEntity;
use bevy::color::palettes::css::BLUE;

pub(in super::super) const TESLA_MODEL_LIFT: f32 = 0.031;
const DOME_ELECTRODE_Z: f32 = 0.126;
const TUBE_GLOW: LinearRgba = LinearRgba::rgb(1.2, 0.55, 0.15);
const TUBE_IDLE_GLOW: f32 = 0.2;
const TUBE_FIRING_GLOW: f32 = 1.6;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TeslaTower;

#[derive(Component)]
pub struct TeslaTubes;

pub fn spawn(
    pb_world: &mut ChildSpawnerCommands,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) -> Entity {
    let sight_radius = 0.15;
    let tube_mat = tube_material(mats, assets);
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
            TargetAllInReach,
            Targets::default(),
            DamageOverTime(33.),
        ),
        |tower| build_view(tower, tube_mat.clone(), assets, g_sett, sight_radius),
    )
}

pub(in super::super) fn tube_material(
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
) -> Handle<StandardMaterial> {
    let glass = mats
        .get(&assets.tesla_glass_material)
        .cloned()
        .unwrap_or_default();
    mats.add(glass)
}

pub(crate) fn build_view(
    tower: &mut ChildSpawnerCommands,
    tube_mat: Handle<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    g_sett: &GraphicsSettings,
    sight_radius: f32,
) {
    let rel_id = tower.target_entity();
    let lift = Transform::from_xyz(0., 0., TESLA_MODEL_LIFT);
    tower.spawn((
        Name::new("Tesla Copper Parts"),
        Mesh3d(assets.tesla_copper_parts.clone()),
        MeshMaterial3d(assets.tesla_copper_material.clone()),
        lift,
    ));
    tower.spawn((
        Name::new("Tesla Brass Parts"),
        Mesh3d(assets.tesla_brass_parts.clone()),
        MeshMaterial3d(assets.tesla_brass_material.clone()),
        lift,
    ));
    tower.spawn((
        Name::new("Tesla Gauge Face"),
        Mesh3d(assets.tesla_gauge_face.clone()),
        MeshMaterial3d(assets.tesla_enamel_material.clone()),
        lift,
    ));
    tower.spawn((
        Name::new("Tesla Glass Tubes"),
        Mesh3d(assets.tesla_glass_tubes.clone()),
        MeshMaterial3d(tube_mat),
        lift,
        TeslaTubes,
        RelEntity(rel_id),
    ));
    tower.spawn((
        Name::new("Tesla Dome"),
        Mesh3d(assets.tesla_dome.clone()),
        MeshMaterial3d(assets.tesla_copper_material.clone()),
        lift,
        TowerHead,
        RotateAlways,
    ));
    tower.spawn(shot_flash_light(
        g_sett,
        rel_id,
        sight_radius,
        TESLA_MODEL_LIFT + DOME_ELECTRODE_Z,
    ));
}

#[derive(Component)]
pub struct ShotFlashLight;

fn shot_flash_light(g_sett: &GraphicsSettings, rel_id: Entity, range: f32, z: f32) -> impl Bundle {
    (
        Name::new("Shot Flash"),
        PointLight {
            intensity: 0.,
            color: BLUE.into(),
            shadow_maps_enabled: g_sett.is_shadows,
            range,
            ..default()
        },
        Transform::from_xyz(0., 0., z),
        Visibility::Hidden,
        ShotFlashLight,
        ShotLight,
        RelEntity(rel_id),
    )
}

pub(in super::super) fn shot_animation_system(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q_tesla: Query<(Entity, &Targets), (With<TeslaTower>, With<TowerReady>)>,
    mut q_shot_flash: Query<(&mut Visibility, &mut PointLight, &RelEntity), With<ShotFlashLight>>,
    mut q_tubes: Query<(&'static MeshMaterial3d<StandardMaterial>, &RelEntity), With<TeslaTubes>>,
) {
    for (tower_id, targets) in q_tesla.iter() {
        let firing = !targets.0.is_empty();
        let Some(mut iflash) = get_flash(&mut q_shot_flash, tower_id) else {
            debug!("No shot flash for tower {tower_id}");
            continue;
        };
        if firing {
            let sin = (time.elapsed_secs() * 32.).sin();
            *iflash.0 = Visibility::Inherited;
            iflash.1.intensity = (sin + 1.) * 32.;
        } else if *iflash.0 != Visibility::Hidden {
            *iflash.0 = Visibility::Hidden;
        }
        if let Some((tube_mat, _)) = get_tubes(&mut q_tubes, tower_id)
            && let Some(mut mat) = materials.get_mut(tube_mat.id())
        {
            let strength = if firing {
                let pulse = 0.5 + 0.5 * (time.elapsed_secs() * 12.).sin();
                TUBE_IDLE_GLOW + pulse * (TUBE_FIRING_GLOW - TUBE_IDLE_GLOW)
            } else {
                TUBE_IDLE_GLOW
            };
            mat.emissive = TUBE_GLOW * strength;
        }
    }
}

fn get_flash<'a>(
    q_muzzle_flash: &'a mut Query<
        (&mut Visibility, &mut PointLight, &RelEntity),
        With<ShotFlashLight>,
    >,
    tower_id: Entity,
) -> Option<(Mut<'a, Visibility>, Mut<'a, PointLight>, &'a RelEntity)> {
    q_muzzle_flash
        .iter_mut()
        .find(|(_, _, rel_id)| rel_id.0 == tower_id)
}

fn get_tubes<'a>(
    q_tubes: &'a mut Query<
        (&'static MeshMaterial3d<StandardMaterial>, &RelEntity),
        With<TeslaTubes>,
    >,
    tower_id: Entity,
) -> Option<(&'a MeshMaterial3d<StandardMaterial>, &'a RelEntity)> {
    q_tubes.iter_mut().find(|(_, rel_id)| rel_id.0 == tower_id)
}
