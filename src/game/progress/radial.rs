use super::Progress;
use crate::prelude::*;
use crate::utils::RelEntity;
use std::f32::consts::{FRAC_PI_2, PI};

#[derive(Component, Default)]
pub struct RadialProgressBar {
    phase: f32,
    pub(super) rewinding: bool,
}

impl RadialProgressBar {
    pub fn is_rewinding(&self) -> bool {
        self.rewinding
    }
}

#[derive(Component, Default)]
pub struct RadialProgressCasing;

const RADIAL_ICON_LIFT: f32 = 0.0068;

pub fn spawn_radial(
    spawner: &mut ChildSpawnerCommands,
    assets: &PinballDefenseGltfAssets,
    icon: Option<&Handle<Image>>,
    mats: &mut Assets<StandardMaterial>,
    rel_id: Entity,
    color: Color,
    init_val: f32,
) {
    spawner
        .spawn(radial_casing_bundle(assets))
        .with_children(|spawner| {
            spawner.spawn(radial_bar_bundle(assets, mats, color, rel_id, init_val));
            if let Some(icon) = icon {
                spawner.spawn(radial_icon_bundle(assets, icon, mats));
            }
        });
}

fn radial_casing_bundle(assets: &PinballDefenseGltfAssets) -> impl Bundle {
    (
        Name::new("Radial Progress Casing"),
        Mesh3d(assets.radial_progress_casing.clone()),
        MeshMaterial3d(assets.foundation_lid_material.clone()),
        Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)),
        RadialProgressCasing,
        Visibility::default(),
    )
}

fn radial_bar_bundle(
    assets: &PinballDefenseGltfAssets,
    mats: &mut Assets<StandardMaterial>,
    color: Color,
    rel_id: Entity,
    init_val: f32,
) -> impl Bundle {
    (
        Name::new("Radial Progress Bar"),
        Mesh3d(assets.radial_progress_bar.clone()),
        MeshMaterial3d(mats.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.4,
            metallic: 0.6,
            reflectance: 0.5,
            ..default()
        })),
        Transform::from_rotation(Quat::from_rotation_z(PI * (1. - init_val) + PI + FRAC_PI_2)),
        RadialProgressBar {
            phase: -PI * (1. - init_val),
            ..default()
        },
        Progress(init_val),
        RelEntity(rel_id),
    )
}

fn radial_icon_bundle(
    assets: &PinballDefenseGltfAssets,
    icon: &Handle<Image>,
    mats: &mut Assets<StandardMaterial>,
) -> impl Bundle {
    (
        Name::new("Radial Progress Icon"),
        Mesh3d(assets.extra_field.clone()),
        MeshMaterial3d(mats.add(StandardMaterial {
            base_color_texture: Some(icon.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        })),
        Transform::from_translation(Vec3::new(0., 0., RADIAL_ICON_LIFT))
            .with_rotation(Quat::from_rotation_z(3. * FRAC_PI_2)),
    )
}

const RADIAL_TOLERANCE: f32 = 0.003;

pub(super) fn radial_rotation_system(
    mut q_progress: Query<(&mut Transform, &Progress, &mut RadialProgressBar)>,
    time: Res<Time>,
) {
    for (mut trans, progress, mut bar) in q_progress.iter_mut() {
        if bar.rewinding {
            let remaining = PI * (1. + progress.0) - bar.phase;
            if remaining <= RADIAL_TOLERANCE {
                bar.rewinding = false;
                bar.phase = -PI * (1. - progress.0);
            } else {
                bar.phase += (time.delta_secs() * PI).min(remaining);
            }
        } else {
            let target = -PI * (1. - progress.0);
            let delta = target - bar.phase;
            if delta.abs() < RADIAL_TOLERANCE {
                bar.phase = target;
            } else {
                bar.phase += (time.delta_secs() * 0.5).min(delta.abs()) * delta.signum();
            }
        }
        trans.rotation = Quat::from_rotation_z(-bar.phase + PI + FRAC_PI_2);
    }
}
