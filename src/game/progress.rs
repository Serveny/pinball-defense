use super::{EventState, GameState};
use crate::prelude::*;
use crate::utils::RelEntity;

pub type QueryProgressBar<'w, 's, 'a> = Query<'w, 's, (&'a RelEntity, &'a mut Progress)>;
pub struct ProgressPlugin;

impl Plugin for ProgressPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ProgressBarCountUpEvent>()
            .add_event::<ProgressBarFullEvent>()
            .add_event::<ProgressBarEmptyEvent>()
            .add_systems(
                Update,
                (scale_system, bar_empty_system, bar_full_system)
                    .run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (on_count_up_system).run_if(in_state(EventState::Active)),
            );
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Progress(pub f32);

impl Progress {
    fn is_empty(&self) -> bool {
        self.0 <= 0.
    }

    fn is_full(&self) -> bool {
        self.0 >= 1.
    }
}

#[derive(Event)]
pub struct ProgressBarCountUpEvent {
    rel_id: Entity,
    amount: f32,
}

impl ProgressBarCountUpEvent {
    pub fn new(rel_id: Entity, amount: f32) -> Self {
        Self { rel_id, amount }
    }
}

fn on_count_up_system(
    mut evr: EventReader<ProgressBarCountUpEvent>,
    mut q_progress: QueryProgressBar,
) {
    for ev in evr.read() {
        if let Some((_, mut progress)) = q_progress.iter_mut().find(|(p, _)| p.0 == ev.rel_id) {
            let old = progress.0;
            let new = (old + ev.amount).clamp(0., 1.);
            if new != progress.0 {
                progress.0 = new
            }
        }
    }
}

#[derive(Event)]
pub struct ProgressBarFullEvent(pub Entity);

fn bar_full_system(
    mut full_ev: EventWriter<ProgressBarFullEvent>,
    q_bar: Query<(&RelEntity, &Progress), Changed<Progress>>,
) {
    for (rel_id, bar) in q_bar.iter() {
        if bar.is_full() {
            full_ev.send(ProgressBarFullEvent(rel_id.0));
        }
    }
}

#[derive(Event)]
pub struct ProgressBarEmptyEvent(pub Entity);

fn bar_empty_system(
    mut full_ev: EventWriter<ProgressBarEmptyEvent>,
    q_bar: Query<(&RelEntity, &Progress), Changed<Progress>>,
) {
    for (rel_id, bar) in q_bar.iter() {
        if bar.is_empty() {
            full_ev.send(ProgressBarEmptyEvent(rel_id.0));
        }
    }
}

// 3D Progress Bar
#[derive(Component)]
pub struct ProgressBar;

pub fn spawn(
    parent: &mut ChildBuilder,
    assets: &PinballDefenseGltfAssets,
    mats: &mut Assets<StandardMaterial>,
    rel_id: Entity,
    transform: Transform,
    color: Color,
    init_val: f32,
) {
    parent
        .spawn(frame_bundle(assets, mats, transform))
        .with_children(|parent| {
            parent.spawn(bar_bundle(assets, mats, init_val, rel_id, color));
            parent.spawn(background_bundle(assets, mats, Color::ANTIQUE_WHITE));
        });
}

fn frame_bundle(
    assets: &PinballDefenseGltfAssets,
    mats: &mut Assets<StandardMaterial>,
    transform: Transform,
) -> impl Bundle {
    (
        Name::new("Progress Bar Frame"),
        PbrBundle {
            mesh: assets.progress_bar_frame.clone(),
            material: mats.add(StandardMaterial {
                base_color: Color::BLACK,
                perceptual_roughness: 0.4,
                metallic: 0.6,
                reflectance: 0.5,
                ..default()
            }),
            transform,
            ..default()
        },
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
        PbrBundle {
            mesh: assets.progress_bar.clone(),
            material: mats.add(StandardMaterial {
                base_color: color,
                perceptual_roughness: 0.4,
                metallic: 0.6,
                reflectance: 0.5,
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(0.003, -0.034, 0.003),
                scale: Vec3::new(1., init_val, 1.),
                ..default()
            },
            ..default()
        },
        ProgressBar,
        Progress(init_val),
        RelEntity(rel_id),
        Name::new("Progress Bar"),
    )
}

fn background_bundle(
    assets: &PinballDefenseGltfAssets,
    mats: &mut Assets<StandardMaterial>,
    color: Color,
) -> impl Bundle {
    (
        PbrBundle {
            mesh: assets.progress_bar.clone(),
            material: mats.add(StandardMaterial {
                base_color: color,
                perceptual_roughness: 0.2,
                metallic: 0.6,
                reflectance: 0.1,
                ..default()
            }),
            transform: Transform::from_xyz(0.003, -0.034, 0.002),
            ..default()
        },
        Name::new("Progress Bar Background"),
    )
}

// Makes progress visible
fn scale_system(
    mut q_progress: Query<(&mut Transform, &Progress), With<ProgressBar>>,
    time: Res<Time>,
) {
    for (mut trans, progress) in q_progress.iter_mut() {
        let mut y = trans.scale.y;
        let p = progress.0;
        if y < p - 0.005 {
            y += time.delta_seconds() * 0.5;
            trans.scale.y = y.clamp(0., 1.);
        } else if y > p + 0.005 {
            y -= time.delta_seconds() * 0.5;
            trans.scale.y = y.clamp(0., 1.);
        }
    }
}
