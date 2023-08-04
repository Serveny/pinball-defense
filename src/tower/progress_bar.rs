use crate::prelude::*;
use crate::utils::RelParent;

#[derive(Component, Default)]
pub(super) struct ProgressBar(pub f32);

pub fn spawn_progress_bar(
    parent: &mut ChildBuilder,
    assets: &PinballDefenseAssets,
    materials: &mut Assets<StandardMaterial>,
    rel_id: Entity,
    transform: Transform,
    color: Color,
) {
    parent
        .spawn((
            PbrBundle {
                mesh: assets.tower_foundation_progress_bar_frame.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::BLACK,
                    perceptual_roughness: 0.4,
                    metallic: 0.6,
                    reflectance: 0.5,
                    ..default()
                }),
                transform,
                ..default()
            },
            Name::new("Tower Foundation Progress Bar Frame"),
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: assets.tower_foundation_progress_bar.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: color,
                        perceptual_roughness: 0.4,
                        metallic: 0.6,
                        reflectance: 0.5,
                        ..default()
                    }),
                    transform: Transform {
                        translation: Vec3::new(0.003, 0.003, 0.034),
                        scale: Vec3::new(1., 1., 0.),
                        ..default()
                    },
                    ..default()
                },
                ProgressBar::default(),
                RelParent(rel_id),
                Name::new("Tower Foundation Progress Bar"),
            ));
        });
}

// Returns new value
pub(super) fn progress_count_up(
    parent_id: Entity,
    progress_to_add: f32,
    q_progress: &mut Query<(&RelParent, &mut ProgressBar)>,
) -> f32 {
    if let Some((_, mut progress)) = q_progress.iter_mut().find(|(p, _)| p.0 == parent_id) {
        progress.0 = (progress.0 + progress_to_add).clamp(0., 1.);
        return progress.0;
    }
    0.
}

pub(super) fn progress_bar_scale_system(
    mut q_progress: Query<(&mut Transform, &ProgressBar)>,
    time: Res<Time>,
) {
    for (mut trans, progress) in q_progress.iter_mut() {
        if trans.scale.z < progress.0 {
            trans.scale.z += time.delta_seconds() * 0.5;
        }
    }
}
