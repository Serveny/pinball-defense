use super::{Progress, level_glow_material, neon_material};
use crate::prelude::*;
use crate::utils::RelEntity;
use std::f32::consts::{FRAC_PI_2, PI};

#[derive(Component, Default)]
pub struct RadialProgressBar {
    phase: f32,
    fast_forward_target: Option<f32>,
}

impl RadialProgressBar {
    pub fn is_fast_forwarding(&self) -> bool {
        self.fast_forward_target.is_some()
    }

    pub(super) fn start_fast_forward(&mut self) {
        self.fast_forward_target = Some(self.phase + PI);
    }

    pub(super) fn stop_fast_forward(&mut self) {
        self.fast_forward_target = None;
    }
}

#[derive(Component, Default)]
pub struct RadialProgressCasing;

#[derive(Component)]
pub struct RadialProgressGlow;

const RADIAL_ICON_LIFT: f32 = 0.0068;

pub fn spawn_radial(
    spawner: &mut ChildSpawnerCommands,
    assets: &PinballDefenseGltfAssets,
    icon: Option<&Handle<Image>>,
    mats: &mut Assets<StandardMaterial>,
    rel_id: Entity,
    color: Color,
    init_val: f32,
    casing_color: Option<Color>,
) {
    spawner
        .spawn(radial_casing_bundle(assets, rel_id))
        .with_children(|spawner| {
            if let Some(color) = casing_color {
                spawner.spawn((
                    Name::new("Radial Progress Glow"),
                    Mesh3d(assets.radial_progress_casing.clone()),
                    MeshMaterial3d(mats.add(level_glow_material(color))),
                    Transform::from_xyz(0., 0., -0.001).with_scale(Vec3::new(1.045, 1.045, 1.)),
                    RadialProgressGlow,
                    RelEntity(rel_id),
                ));
            }
            spawner.spawn(radial_bar_bundle(assets, mats, color, rel_id, init_val));
            if let Some(icon) = icon {
                spawner.spawn(radial_icon_bundle(assets, icon, mats));
            }
        });
}

fn radial_casing_bundle(assets: &PinballDefenseGltfAssets, rel_id: Entity) -> impl Bundle {
    (
        Name::new("Radial Progress Casing"),
        Mesh3d(assets.radial_progress_casing.clone()),
        MeshMaterial3d(assets.foundation_lid_material.clone()),
        Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)),
        RadialProgressCasing,
        RelEntity(rel_id),
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
        MeshMaterial3d(mats.add(neon_material(color))),
        Transform::from_rotation(Quat::from_rotation_z(PI * (1. + init_val) + PI + FRAC_PI_2)),
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
        if let Some(target) = bar.fast_forward_target {
            let remaining = target - bar.phase;
            if remaining <= RADIAL_TOLERANCE {
                bar.fast_forward_target = None;
                bar.phase = -PI * (1. - progress.0);
            } else {
                bar.phase += (time.delta_secs() * 2. * PI).min(remaining);
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
        trans.rotation = Quat::from_rotation_z(bar.phase + PI * 3.5);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spawn_test_radial(
        mut cmds: Commands,
        assets: Res<PinballDefenseGltfAssets>,
        mut mats: ResMut<Assets<StandardMaterial>>,
    ) {
        cmds.spawn_empty().with_children(|p| {
            spawn_radial(
                p,
                &assets,
                None,
                &mut mats,
                Entity::PLACEHOLDER,
                Color::WHITE,
                0.,
                Some(Color::srgb_u8(80, 180, 255)),
            );
        });
    }

    #[test]
    fn iron_casing_has_separate_outer_glow() {
        let mut app = App::new();
        app.insert_resource(PinballDefenseGltfAssets::default())
            .init_resource::<Assets<StandardMaterial>>()
            .add_systems(Update, spawn_test_radial);
        app.update();

        let iron = app
            .world()
            .resource::<PinballDefenseGltfAssets>()
            .foundation_lid_material
            .clone();
        let mut casing = app
            .world_mut()
            .query_filtered::<&MeshMaterial3d<StandardMaterial>, With<RadialProgressCasing>>();
        assert_eq!(casing.single(app.world()).unwrap().0, iron);

        let mut glow = app.world_mut().query_filtered::<(&MeshMaterial3d<StandardMaterial>, &Transform), With<RadialProgressGlow>>();
        let (material, transform) = glow.single(app.world()).unwrap();
        assert_ne!(material.0, iron);
        assert!(transform.scale.x > 1.);
        assert!(transform.translation.z < 0.);
        assert_eq!(
            app.world()
                .resource::<Assets<StandardMaterial>>()
                .get(&material.0)
                .unwrap()
                .emissive,
            Color::srgb_u8(80, 180, 255).to_linear()
        );
    }
}
