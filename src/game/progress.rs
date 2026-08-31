use super::{EventState, GameState};
use crate::prelude::*;
use crate::utils::RelEntity;
use bevy::color::palettes::css::ANTIQUE_WHITE;

pub type QueryProgressBar<'w, 's, 'a> = Query<'w, 's, (&'a RelEntity, &'a mut Progress)>;
pub struct ProgressPlugin;

impl Plugin for ProgressPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ProgressBarCountUpEvent>()
            .add_message::<ProgressBarFullEvent>()
            .add_message::<ProgressBarResetEvent>()
            .register_type::<Progress>()
            .add_systems(
                Update,
                (scale_system, bar_full_system, activate_animation_system)
                    .run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (on_count_up_system, reset_on_upgrade_system).run_if(in_state(EventState::Active)),
            );
    }
}

#[derive(Component, Clone, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct Progress(pub f32);

impl Progress {
    fn is_full(&self) -> bool {
        self.0 >= 1.
    }
}

#[derive(Message)]
pub struct ProgressBarCountUpEvent {
    rel_id: Entity,
    amount: f32,
}

impl ProgressBarCountUpEvent {
    pub fn new(rel_id: Entity, amount: f32) -> Self {
        Self { rel_id, amount }
    }

    pub fn rel_id(&self) -> Entity {
        self.rel_id
    }
}

#[allow(clippy::float_cmp)]
fn on_count_up_system(
    mut evr: MessageReader<ProgressBarCountUpEvent>,
    mut q_progress: QueryProgressBar,
) {
    for ev in evr.read() {
        for (_, mut progress) in q_progress.iter_mut().filter(|(p, _)| p.0 == ev.rel_id) {
            let new = (progress.0 + ev.amount).clamp(0., 1.);
            if new != progress.0 {
                progress.0 = new;
            }
        }
    }
}

fn reset_on_upgrade_system(
    mut evr: MessageReader<ProgressBarResetEvent>,
    mut q_progress: Query<(&RelEntity, &mut Progress), With<ProgressBar>>,
) {
    for ev in evr.read() {
        for (_, mut progress) in q_progress.iter_mut().filter(|(r, _)| r.0 == ev.rel_id()) {
            progress.0 = 0.;
        }
    }
}

#[derive(Message)]
pub struct ProgressBarFullEvent(pub Entity);

#[derive(Message)]
pub struct ProgressBarResetEvent {
    rel_id: Entity,
}

impl ProgressBarResetEvent {
    pub fn new(rel_id: Entity) -> Self {
        Self { rel_id }
    }

    pub fn rel_id(&self) -> Entity {
        self.rel_id
    }
}

fn bar_full_system(
    mut full_ev: MessageWriter<ProgressBarFullEvent>,
    q_bar: Query<(&RelEntity, &Progress), Changed<Progress>>,
) {
    for (rel_id, bar) in q_bar.iter() {
        if bar.is_full() {
            full_ev.write(ProgressBarFullEvent(rel_id.0));
        }
    }
}

// 3D Progress Bar
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

fn activate_animation_system(mut q_progess: Query<&mut ProgressBar, Changed<Progress>>) {
    for mut bar in q_progess.iter_mut() {
        bar.is_active_animation = true;
    }
}

const TOLERANCE: f32 = 0.01;

// Makes progress visible
fn scale_system(
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
