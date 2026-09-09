use super::Progress;
use crate::prelude::*;
use crate::utils::RelEntity;
use bevy::color::palettes::css::ANTIQUE_WHITE;

#[derive(Component, Default)]
pub struct ProgressBar {
    is_active_animation: bool,
}

pub fn spawn(
    spawner: &mut ChildSpawnerCommands,
    assets: &PinballDefenseGltfAssets,
    mats: &mut Assets<StandardMaterial>,
    rel_id: Entity,
    transform: Transform,
    color: Color,
    init_val: f32,
) {
    spawner
        .spawn(frame_bundle(assets, mats, transform))
        .with_children(|spawner| {
            spawner.spawn(bar_bundle(assets, mats, init_val, rel_id, color));
            spawner.spawn(background_bundle(assets, mats, ANTIQUE_WHITE.into()));
        });
}

fn frame_bundle(
    assets: &PinballDefenseGltfAssets,
    mats: &mut Assets<StandardMaterial>,
    transform: Transform,
) -> impl Bundle {
    (
        Name::new("Progress Bar Frame"),
        Mesh3d(assets.progress_bar_frame.clone()),
        MeshMaterial3d(mats.add(StandardMaterial {
            base_color: Color::BLACK,
            perceptual_roughness: 0.4,
            metallic: 0.6,
            reflectance: 0.5,
            ..default()
        })),
        transform,
    )
}

fn bar_bundle(
    assets: &PinballDefenseGltfAssets,
    mats: &mut Assets<StandardMaterial>,
    init_val: f32,
    rel_id: Entity,
    color: Color,
) -> impl Bundle {
    (
        Name::new("Progress Bar"),
        Mesh3d(assets.progress_bar.clone()),
        MeshMaterial3d(mats.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.4,
            metallic: 0.6,
            reflectance: 0.5,
            ..default()
        })),
        Transform {
            translation: Vec3::new(0.003, -0.034, 0.003),
            scale: Vec3::new(1., init_val, 1.),
            ..default()
        },
        ProgressBar::default(),
        Progress(init_val),
        RelEntity(rel_id),
    )
}

fn background_bundle(
    assets: &PinballDefenseGltfAssets,
    mats: &mut Assets<StandardMaterial>,
    color: Color,
) -> impl Bundle {
    (
        Name::new("Progress Bar Background"),
        Mesh3d(assets.progress_bar.clone()),
        MeshMaterial3d(mats.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.2,
            metallic: 0.6,
            reflectance: 0.1,
            ..default()
        })),
        Transform::from_xyz(0.003, -0.034, 0.002),
    )
}

pub(super) fn activate_animation_system(mut q_progess: Query<&mut ProgressBar, Changed<Progress>>) {
    for mut bar in q_progess.iter_mut() {
        bar.is_active_animation = true;
    }
}

const TOLERANCE: f32 = 0.01;
pub(super) fn scale_system(
    mut q_progress: Query<(&mut Transform, &Progress, &mut ProgressBar)>,
    time: Res<Time>,
) {
    for (mut trans, progress, mut bar) in q_progress
        .iter_mut()
        .filter(|(_, _, bar)| bar.is_active_animation)
    {
        let p = progress.0;
        let mut y = trans.scale.y;
        y += time.delta_secs() * 0.5 * (p - y).signum();

        if (y - p).abs() < TOLERANCE {
            y = p;
            bar.is_active_animation = false;
        }

        trans.scale.y = y;
    }
}