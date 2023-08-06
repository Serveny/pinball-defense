use crate::prelude::*;
use crate::utils::RelParent;
use crate::GameState;

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ProgressBarCountUpEvent>()
            .add_event::<ProgressBarFullEvent>()
            .add_systems(
                Update,
                (count_up_system, scale_system).run_if(in_state(GameState::Ingame)),
            );
    }
}

#[derive(Component, Default)]
pub struct ProgressBar(pub f32);

pub fn spawn(
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

#[derive(Event)]
pub struct ProgressBarCountUpEvent(pub Entity, pub f32);

#[derive(Event)]
pub struct ProgressBarFullEvent(pub Entity);

fn count_up_system(
    mut evr: EventReader<ProgressBarCountUpEvent>,
    mut q_progress: Query<(&RelParent, &mut ProgressBar)>,
    mut evw: EventWriter<ProgressBarFullEvent>,
) {
    for ev in evr.iter() {
        let (rel_id, to_add) = (ev.0, ev.1);
        if let Some((_, mut progress)) = q_progress.iter_mut().find(|(p, _)| p.0 == rel_id) {
            log!("Progress: {}", progress.0);
            if progress.0 < 1. {
                progress.0 += to_add;
                if progress.0 >= 1. {
                    evw.send(ProgressBarFullEvent(rel_id));
                }
            }
        }
    }
}

// Makes progress visible
fn scale_system(mut q_progress: Query<(&mut Transform, &ProgressBar)>, time: Res<Time>) {
    for (mut trans, progress) in q_progress.iter_mut() {
        if trans.scale.z < progress.0 {
            trans.scale.z += time.delta_seconds() * 0.5;
        }
    }
}
