use super::super::TowerReady;
use super::super::target::EnemiesWithinReach;
use super::super::types::tesla::TeslaTower;
use crate::game::enemy::Enemy;
use crate::prelude::*;
use crate::utils::RelEntity;
use bevy::asset::RenderAssetUsages;
use bevy::color::palettes::css::BLUE;
use bevy::light::NotShadowCaster;
use bevy::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy_hanabi::{
    AccelModifier, AlphaMode, Attribute, ColorOverLifetimeModifier, EffectAsset, EffectSpawner,
    Gradient, LinearDragModifier, Module, ParticleEffect, RoundModifier,
    SetAttributeModifier, SetPositionSphereModifier, SetVelocitySphereModifier,
    ShapeDimension, SimulationSpace, SizeOverLifetimeModifier, SpawnerSettings,
};

const IMPACT_SEGMENTS: usize = 12;
const BOLT_ORIGIN_Z: f32 = 0.078;
const BOLT_WIDTH: f32 = 0.003;
const BOLT_JITTER: f32 = 0.02;
const SPARK_BURST: f32 = 12.;
const SPARK_PERIOD: f32 = 0.04;
const SMOKE_RATE: f32 = 20.;
const FLASH_RANGE: f32 = 0.05;

#[derive(Resource)]
pub(in super::super) struct TeslaEffectAssets {
    pub(in super::super) bolt_mat: Handle<StandardMaterial>,
    pub(in super::super) sparks: Handle<EffectAsset>,
    pub(in super::super) smoke: Handle<EffectAsset>,
}

impl FromWorld for TeslaEffectAssets {
    fn from_world(world: &mut World) -> Self {
        let bolt_mat = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial {
                base_color: Color::srgba(0.35, 0.65, 1., 1.),
                emissive: LinearRgba::rgb(1.2, 2.5, 5.),
                unlit: true,
                alpha_mode: bevy::material::AlphaMode::Add,
                ..default()
            });
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();
        Self {
            bolt_mat,
            sparks: effects.add(sparks_asset()),
            smoke: effects.add(smoke_asset()),
        }
    }
}

fn sparks_asset() -> EffectAsset {
    let mut module = Module::default();
    let center = module.lit(Vec3::ZERO);
    let radius = module.lit(0.004);
    let speed = module.lit(0.25);
    let age = module.lit(0.);
    let lifetime = module.lit(0.22);
    let drag = module.lit(6.);
    let gravity = AccelModifier::constant(&mut module, Vec3::Z * -0.5);
    let round = RoundModifier::ellipse(&mut module);

    let mut color = Gradient::new();
    color.add_key(0.0, Vec4::new(0.6, 1.4, 3., 1.));
    color.add_key(0.4, Vec4::new(0.2, 0.5, 1.4, 0.7));
    color.add_key(1.0, Vec4::new(0.05, 0.1, 0.4, 0.));

    let mut size = Gradient::new();
    size.add_key(0.0, Vec3::splat(0.004));
    size.add_key(1.0, Vec3::splat(0.001));

    EffectAsset::new(
        128,
        SpawnerSettings::burst(SPARK_BURST.into(), SPARK_PERIOD.into()).with_starts_active(false),
        module,
    )
    .with_name("tesla_impact_sparks")
    .with_simulation_space(SimulationSpace::Global)
    .with_alpha_mode(AlphaMode::Add)
    .init(SetPositionSphereModifier {
        center,
        radius,
        dimension: ShapeDimension::Volume,
    })
    .init(SetVelocitySphereModifier { center, speed })
    .init(SetAttributeModifier::new(Attribute::AGE, age))
    .init(SetAttributeModifier::new(Attribute::LIFETIME, lifetime))
    .update(gravity)
    .update(LinearDragModifier::new(drag))
    .render(round)
    .render(ColorOverLifetimeModifier::new(color))
    .render(SizeOverLifetimeModifier {
        gradient: size,
        screen_space_size: false,
    })
}

fn smoke_asset() -> EffectAsset {
    let mut module = Module::default();
    let center = module.lit(Vec3::ZERO);
    let radius = module.lit(0.008);
    let spread = module.lit(0.03);
    let rise = module.lit(Vec3::Z * 0.05);
    let age = module.lit(0.);
    let lifetime = module.lit(0.5);
    let drag = module.lit(1.5);
    let round = RoundModifier::ellipse(&mut module);

    let mut color = Gradient::new();
    color.add_key(0.0, Vec4::new(0.5, 0.55, 0.65, 0.));
    color.add_key(0.1, Vec4::new(0.4, 0.45, 0.55, 0.45));
    color.add_key(1.0, Vec4::new(0.3, 0.33, 0.4, 0.));

    let mut size = Gradient::new();
    size.add_key(0.0, Vec3::splat(0.006));
    size.add_key(1.0, Vec3::splat(0.03));

    EffectAsset::new(
        128,
        SpawnerSettings::rate(SMOKE_RATE.into()).with_starts_active(false),
        module,
    )
    .with_name("tesla_impact_smoke")
    .with_simulation_space(SimulationSpace::Global)
    .with_alpha_mode(AlphaMode::Blend)
    .init(SetPositionSphereModifier {
        center,
        radius,
        dimension: ShapeDimension::Volume,
    })
    .init(SetVelocitySphereModifier {
        center,
        speed: spread,
    })
    .init(SetAttributeModifier::new(Attribute::VELOCITY, rise))
    .init(SetAttributeModifier::new(Attribute::AGE, age))
    .init(SetAttributeModifier::new(Attribute::LIFETIME, lifetime))
    .update(LinearDragModifier::new(drag))
    .render(round)
    .render(ColorOverLifetimeModifier::new(color))
    .render(SizeOverLifetimeModifier {
        gradient: size,
        screen_space_size: false,
    })
}

fn bolt_mesh(segments: usize) -> Mesh {
    let vert_count = (segments + 1) * 2;
    let mut positions = Vec::with_capacity(vert_count);
    for _ in 0..=segments {
        positions.push([0., 0., 0.]);
        positions.push([0., 0., 0.]);
    }
    let mut indices = Vec::with_capacity(segments * 6);
    for seg in 0..segments {
        let base = u16::try_from(seg * 2).unwrap_or(u16::MAX);
        indices.extend([base, base + 1, base + 2, base + 1, base + 3, base + 2]);
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 0., 1.]; vert_count])
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; vert_count])
    .with_inserted_indices(Indices::U16(indices))
}

#[derive(Component)]
pub(in super::super) struct TeslaBolt;

#[derive(Component)]
pub(in super::super) struct TeslaArcTarget(pub(in super::super) Entity);

#[derive(Component)]
pub(in super::super) struct TeslaImpactSparks;

#[derive(Component)]
pub(in super::super) struct TeslaImpactSmoke;

#[derive(Component)]
pub(in super::super) struct TeslaImpactFlash;

#[allow(clippy::cast_precision_loss)]
fn hash01(x: f32) -> f32 {
    (x.sin() * 43_758.547).fract().abs()
}

pub(in super::super) fn maintain_arcs_system(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Option<Res<TeslaEffectAssets>>,
    q_towers: Query<(Entity, &EnemiesWithinReach), (With<TeslaTower>, With<TowerReady>)>,
    mut q_bolts: Query<
        (&mut TeslaArcTarget, &mut Visibility, &RelEntity),
        With<TeslaBolt>,
    >,
) {
    let Some(assets) = assets else { return };
    for (tower_id, ewr) in q_towers.iter() {
        let mut unclaimed: Vec<Entity> = ewr.0.iter().copied().collect();
        for (mut target, mut vis, rel_id) in q_bolts.iter_mut() {
            if rel_id.0 != tower_id {
                continue;
            }
            if unclaimed.contains(&target.0) {
                unclaimed.retain(|id| *id != target.0);
                if *vis != Visibility::Inherited {
                    *vis = Visibility::Inherited;
                }
            } else if let Some(enemy_id) = unclaimed.pop() {
                target.0 = enemy_id;
                if *vis != Visibility::Inherited {
                    *vis = Visibility::Inherited;
                }
            } else if *vis != Visibility::Hidden {
                *vis = Visibility::Hidden;
            }
        }
        for enemy_id in unclaimed {
            let bolt_mesh = meshes.add(bolt_mesh(IMPACT_SEGMENTS));
            cmds.entity(tower_id).with_children(|p| {
                p.spawn((
                    Name::new("Tesla Bolt"),
                    Mesh3d(bolt_mesh),
                    MeshMaterial3d(assets.bolt_mat.clone()),
                    Transform::from_xyz(0., 0., BOLT_ORIGIN_Z),
                    Visibility::Inherited,
                    NotShadowCaster,
                    TeslaBolt,
                    TeslaArcTarget(enemy_id),
                    RelEntity(tower_id),
                ))
                .with_children(|c| {
                    c.spawn((
                        Name::new("Tesla Impact Sparks"),
                        ParticleEffect::new(assets.sparks.clone()),
                        TeslaImpactSparks,
                    ));
                    c.spawn((
                        Name::new("Tesla Impact Smoke"),
                        ParticleEffect::new(assets.smoke.clone()),
                        TeslaImpactSmoke,
                    ));
                    c.spawn((
                        Name::new("Tesla Impact Flash"),
                        PointLight {
                            intensity: 0.,
                            color: BLUE.into(),
                            shadow_maps_enabled: false,
                            range: FLASH_RANGE,
                            ..default()
                        },
                        TeslaImpactFlash,
                    ));
                });
            });
        }
    }
}

pub(in super::super) fn update_arcs_system(
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    q_bolts: Query<
        (Entity, &TeslaArcTarget, &GlobalTransform, &Mesh3d, &Visibility),
        With<TeslaBolt>,
    >,
    q_targets: Query<&GlobalTransform, With<Enemy>>,
    mut q_sparks: Query<
        (&mut Transform, &mut EffectSpawner, &ChildOf),
        (
            With<TeslaImpactSparks>,
            Without<TeslaImpactSmoke>,
            Without<TeslaImpactFlash>,
        ),
    >,
    mut q_smoke: Query<
        (&mut Transform, &mut EffectSpawner, &ChildOf),
        (With<TeslaImpactSmoke>, Without<TeslaImpactFlash>),
    >,
    mut q_flash: Query<(&mut Transform, &mut PointLight, &ChildOf), With<TeslaImpactFlash>>,
) {
    let frame = time.elapsed_secs() * 32.;
    for (bolt_id, target, bolt_gtf, mesh_handle, vis) in q_bolts.iter() {
        let active = *vis == Visibility::Inherited && q_targets.contains(target.0);
        let Some(Ok(target_pos)) = active
            .then(|| q_targets.get(target.0).map(GlobalTransform::translation))
        else {
            for (mut tf, mut spawner, child_of) in q_sparks.iter_mut() {
                if child_of.parent() == bolt_id {
                    tf.translation = Vec3::ZERO;
                    spawner.active = false;
                }
            }
            for (mut tf, mut spawner, child_of) in q_smoke.iter_mut() {
                if child_of.parent() == bolt_id {
                    tf.translation = Vec3::ZERO;
                    spawner.active = false;
                }
            }
            for (mut tf, mut light, child_of) in q_flash.iter_mut() {
                if child_of.parent() == bolt_id {
                    tf.translation = Vec3::ZERO;
                    if light.intensity != 0. {
                        light.intensity = 0.;
                    }
                }
            }
            continue;
        };
        let bolt_pos = bolt_gtf.translation();
        let to_target = target_pos - bolt_pos;
        let Some(mut mesh) = meshes.get_mut(&mesh_handle.0) else {
            continue;
        };
        let Ok(VertexAttributeValues::Float32x3(pos)) =
            mesh.try_attribute_mut(Mesh::ATTRIBUTE_POSITION)
        else {
            continue;
        };
        let dir = to_target.try_normalize().unwrap_or(Vec3::Z);
        let side = dir.cross(Vec3::Z).try_normalize().unwrap_or(Vec3::X);
        #[allow(clippy::cast_precision_loss)]
        let segment_count = pos.len() as f32 / 2. - 1.;
        for (seg, [a, b]) in pos.as_chunks_mut::<2>().0.iter_mut().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let seg = seg as f32;
            let t = seg / segment_count;
            let envelope = (t * (1. - t) * 4.).sqrt();
            let jag = (hash01(seg * 7.31 + frame) - 0.5) * BOLT_JITTER * envelope;
            let p = to_target * t + side * jag;
            let half = BOLT_WIDTH * (1. + envelope);
            *a = (p - side * half).to_array();
            *b = (p + side * half).to_array();
        }
        for (mut tf, mut spawner, child_of) in q_sparks.iter_mut() {
            if child_of.parent() == bolt_id {
                tf.translation = to_target;
                spawner.active = true;
            }
        }
        for (mut tf, mut spawner, child_of) in q_smoke.iter_mut() {
            if child_of.parent() == bolt_id {
                tf.translation = to_target;
                spawner.active = true;
            }
        }
        for (mut tf, mut light, child_of) in q_flash.iter_mut() {
            if child_of.parent() == bolt_id {
                tf.translation = to_target;
                let sin = (time.elapsed_secs() * 48.).sin();
                light.intensity = (sin + 1.) * 24.;
            }
        }
    }
}