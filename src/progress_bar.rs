use crate::prelude::*;
use crate::utils::RelParent;
use crate::GameState;

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ProgressBarCountUpEvent>()
            .add_event::<ProgressBarFullEvent>()
            .add_event::<ProgressBarEmptyEvent>()
            .add_systems(
                Update,
                (count_up_system, scale_system).run_if(in_state(GameState::Ingame)),
            );
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct ProgressBar(pub f32);

pub fn spawn(
    parent: &mut ChildBuilder,
    assets: &PinballDefenseAssets,
    materials: &mut Assets<StandardMaterial>,
    rel_id: Entity,
    transform: Transform,
    color: Color,
    init_val: f32,
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
            Name::new("Progress Bar Frame"),
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: assets.progress_bar.clone(),
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
                ProgressBar(init_val),
                RelParent(rel_id),
                Name::new("Progress Bar"),
            ));
        });
}

#[derive(Event)]
pub struct ProgressBarCountUpEvent(pub Entity, pub f32);

#[derive(Event)]
pub struct ProgressBarFullEvent(pub Entity);

#[derive(Event)]
pub struct ProgressBarEmptyEvent(pub Entity);

fn count_up_system(
    mut evr: EventReader<ProgressBarCountUpEvent>,
    mut q_progress: Query<(&RelParent, &mut ProgressBar)>,
    mut full_ev: EventWriter<ProgressBarFullEvent>,
    mut empty_ev: EventWriter<ProgressBarEmptyEvent>,
) {
    for ev in evr.iter() {
        let (rel_id, to_add) = (ev.0, ev.1);
        if let Some((_, mut progress)) = q_progress.iter_mut().find(|(p, _)| p.0 == rel_id) {
            let old = progress.0;
            if to_add.is_sign_negative() {
                if old == 0. {
                    continue;
                }

                let mut new = progress.0 + to_add;
                if new <= 0. {
                    empty_ev.send(ProgressBarEmptyEvent(rel_id));
                    new = 0.;
                }
                progress.0 = new;
            } else {
                if old == 1. {
                    continue;
                }

                let mut new = progress.0 + to_add;
                if new >= 1. {
                    full_ev.send(ProgressBarFullEvent(rel_id));
                    new = 1.;
                }
                progress.0 = new;
                log!("🧑‍💻 Progress: {} + {} = {}", old, to_add, new);
            }
        }
    }
}

// Makes progress visible
fn scale_system(mut q_progress: Query<(&mut Transform, &ProgressBar)>, time: Res<Time>) {
    for (mut trans, progress) in q_progress.iter_mut() {
        let mut z = trans.scale.z;
        let p = progress.0;
        if z < p - 0.005 {
            z += time.delta_seconds() * 0.5;
            trans.scale.z = z.clamp(0., 1.);
        } else if z > p + 0.005 {
            z -= time.delta_seconds() * 0.5;
            trans.scale.z = z.clamp(0., 1.);
        }
    }
}
