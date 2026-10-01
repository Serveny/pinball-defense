use super::super::TowerReady;
use super::super::target::EnemiesWithinReach;
use super::super::types::tesla::TeslaTower;
use crate::game::enemy::Enemy;
use crate::prelude::*;
use crate::utils::RelEntity;
use bevy::asset::RenderAssetUsages;
use bevy::camera::visibility::NoFrustumCulling;
use bevy::color::palettes::css::BLUE;
use bevy::light::NotShadowCaster;
use bevy::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy_hanabi::{
    AccelModifier, AlphaMode, Attribute, ColorOverLifetimeModifier, EffectAsset, EffectSpawner,
    Gradient, LinearDragModifier, Module, ParticleEffect, RoundModifier, SetAttributeModifier,
    SetPositionSphereModifier, SetVelocitySphereModifier, ShapeDimension, SimulationSpace,
    SizeOverLifetimeModifier, SpawnerSettings,
};

const IMPACT_SEGMENTS: usize = 12;
const MAX_BRANCHES: usize = 3;
const PATH_VERTICES: usize = (IMPACT_SEGMENTS + 1) * 8;
const BOLT_ORIGIN_Z: f32 = 0.157;
const BOLT_WIDTH: f32 = 0.0018;
const BOLT_JITTER: f32 = 0.014;
const SPARK_BURST: f32 = 12.;
const SPARK_PERIOD: f32 = 0.04;
const SMOKE_RATE: f32 = 20.;
const FLASH_RANGE: f32 = 0.1;

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
                base_color: Color::WHITE,
                emissive: LinearRgba::rgb(1.2, 2.5, 5.),
                unlit: true,
                alpha_mode: bevy::prelude::AlphaMode::Add,
                cull_mode: None,
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

fn bolt_mesh() -> Mesh {
    let vert_count = PATH_VERTICES * (MAX_BRANCHES + 1);
    let edge_lin = Color::srgb(0.35, 0.65, 1.).to_linear();
    let edge = [edge_lin.red, edge_lin.green, edge_lin.blue, 0.22];
    let core = [2.5, 2.5, 2.5, 1.];
    let mut positions = Vec::with_capacity(vert_count);
    let mut colors = Vec::with_capacity(vert_count);
    for _ in 0..vert_count / 8 {
        for color in [edge, core, core, edge, edge, core, core, edge] {
            positions.push([0., 0., 0.]);
            colors.push(color);
        }
    }
    let mut indices = Vec::with_capacity((MAX_BRANCHES + 1) * IMPACT_SEGMENTS * 36);
    for path in 0..=MAX_BRANCHES {
        for seg in 0..IMPACT_SEGMENTS {
            let row = u16::try_from(path * PATH_VERTICES + seg * 8).unwrap_or(u16::MAX);
            for col in [0, 1, 2, 4, 5, 6] {
                let base = row + col;
                indices.extend([base, base + 8, base + 1, base + 1, base + 8, base + 9]);
            }
        }
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 0., 1.]; vert_count])
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; vert_count])
    .with_inserted_indices(Indices::U16(indices))
}

#[derive(Component)]
pub(in super::super) struct TeslaBolt;

#[derive(Component)]
pub(in super::super) struct TeslaImpactSlot(usize);

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
    q_towers: Query<Entity, (With<TeslaTower>, With<TowerReady>)>,
    q_bolts: Query<&RelEntity, With<TeslaBolt>>,
) {
    let Some(assets) = assets else { return };
    for tower_id in q_towers.iter() {
        if q_bolts.iter().any(|rel_id| rel_id.0 == tower_id) {
            continue;
        }
        let bolt_mesh = meshes.add(bolt_mesh());
        cmds.entity(tower_id).with_children(|p| {
            p.spawn((
                Name::new("Tesla Branched Bolt"),
                Mesh3d(bolt_mesh),
                MeshMaterial3d(assets.bolt_mat.clone()),
                Transform::from_xyz(0., 0., BOLT_ORIGIN_Z),
                Visibility::Hidden,
                NotShadowCaster,
                NoFrustumCulling,
                TeslaBolt,
                RelEntity(tower_id),
            ))
            .with_children(|c| {
                for slot in 0..MAX_BRANCHES {
                    c.spawn((
                        Name::new("Tesla Impact Sparks"),
                        ParticleEffect::new(assets.sparks.clone()),
                        TeslaImpactSparks,
                        TeslaImpactSlot(slot),
                    ));
                    c.spawn((
                        Name::new("Tesla Impact Smoke"),
                        ParticleEffect::new(assets.smoke.clone()),
                        TeslaImpactSmoke,
                        TeslaImpactSlot(slot),
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
                        TeslaImpactSlot(slot),
                    ));
                }
            });
        });
    }
}

fn select_arc_targets(
    mut candidates: Vec<(Entity, Vec3)>,
    pulse: usize,
) -> [Option<Vec3>; MAX_BRANCHES] {
    let mut targets = [None; MAX_BRANCHES];
    if candidates.is_empty() {
        return targets;
    }
    candidates.sort_unstable_by_key(|(id, _)| *id);
    let (_, anchor) = candidates.swap_remove(pulse % candidates.len());
    candidates.sort_unstable_by(|(id_a, a), (id_b, b)| {
        a.distance_squared(anchor)
            .total_cmp(&b.distance_squared(anchor))
            .then_with(|| id_a.cmp(id_b))
    });
    for (slot, point) in targets
        .iter_mut()
        .zip(std::iter::once(anchor).chain(candidates.into_iter().map(|(_, pos)| pos)))
    {
        *slot = Some(point);
    }
    targets
}

fn branch_junction(targets: &[Option<Vec3>; MAX_BRANCHES]) -> Vec3 {
    let mut center = Vec3::ZERO;
    let mut count = 0.;
    for point in targets.iter().flatten() {
        center += *point;
        count += 1.;
    }
    if count <= 1. {
        return center;
    }
    center / count * 0.78
}

fn write_bolt_path(positions: &mut [[f32; 3]], start: Vec3, end: Vec3, width: f32, seed: f32) {
    let delta = end - start;
    let dir = delta.try_normalize().unwrap_or(Vec3::Z);
    let side = dir.cross(Vec3::Z).try_normalize().unwrap_or(Vec3::X);
    let up = side.cross(dir);
    let jitter = BOLT_JITTER.min(delta.length() * 0.18);
    for (seg, strip) in positions.as_chunks_mut::<8>().0.iter_mut().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let t = seg as f32 / IMPACT_SEGMENTS as f32;
        let envelope = (t * (1. - t) * 4.).sqrt();
        let jag_side = (hash01(t * 87.72 + seed) - 0.5) * jitter * envelope;
        let jag_up = (hash01(t * 47.64 + seed * 1.3) - 0.5) * jitter * envelope;
        let point = start + delta * t + side * jag_side + up * jag_up;
        let half = width * (1. - t * 0.45);
        for (vertex, offset) in strip.iter_mut().zip([
            -side * half,
            -side * half * 0.25,
            side * half * 0.25,
            side * half,
            -up * half,
            -up * half * 0.25,
            up * half * 0.25,
            up * half,
        ]) {
            *vertex = (point + offset).to_array();
        }
    }
}

fn update_bolt_positions(
    positions: &mut [[f32; 3]],
    targets: &[Option<Vec3>; MAX_BRANCHES],
    seed: f32,
) {
    positions.fill([0.; 3]);
    if targets.iter().all(Option::is_none) {
        return;
    }
    let junction = branch_junction(targets);
    let mut paths = positions.as_chunks_mut::<PATH_VERTICES>().0.iter_mut();
    if let Some(trunk) = paths.next() {
        write_bolt_path(trunk, Vec3::ZERO, junction, BOLT_WIDTH, seed);
    }
    if targets.iter().flatten().count() <= 1 {
        return;
    }
    for (branch, target) in paths.zip(targets) {
        if let Some(target) = target {
            write_bolt_path(branch, junction, *target, BOLT_WIDTH * 0.55, seed + 17.);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branches_share_a_late_junction_and_indices_stay_in_their_path() {
        let mut mesh = bolt_mesh();
        let Indices::U16(indices) = mesh.indices().unwrap() else {
            panic!("expected u16 indices");
        };
        for triangle in indices.chunks_exact(3) {
            let path = usize::from(triangle.first().copied().unwrap()) / PATH_VERTICES;
            assert!(triangle.iter().all(|i| {
                usize::from(*i) < PATH_VERTICES * (MAX_BRANCHES + 1)
                    && usize::from(*i) / PATH_VERTICES == path
            }));
        }
        let VertexAttributeValues::Float32x3(positions) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap()
        else {
            panic!("expected positions");
        };
        let targets = [
            Some(Vec3::new(0.2, -0.02, -0.1)),
            Some(Vec3::new(0.2, 0., -0.1)),
            Some(Vec3::new(0.2, 0.02, -0.1)),
        ];
        update_bolt_positions(positions, &targets, 7.);
        let junction = branch_junction(&targets);
        assert!(junction.abs_diff_eq(Vec3::new(0.156, 0., -0.078), 1e-6));
        let center =
            |strip: &[[f32; 3]]| strip.iter().map(|p| Vec3::from_array(*p)).sum::<Vec3>() / 8.;
        for (path, target) in positions
            .chunks_exact(PATH_VERTICES)
            .zip(std::iter::once(Some(junction)).chain(targets))
        {
            assert!(center(path.get(..8).unwrap()).abs_diff_eq(
                if target == Some(junction) {
                    Vec3::ZERO
                } else {
                    junction
                },
                1e-6,
            ));
            assert!(
                center(path.get(PATH_VERTICES - 8..).unwrap()).abs_diff_eq(target.unwrap(), 1e-6)
            );
            assert!(path.iter().flatten().all(|v| v.is_finite()));
        }
        update_bolt_positions(
            positions,
            &[targets.first().copied().flatten(), None, None],
            8.,
        );
        assert!(
            positions
                .get(PATH_VERTICES..)
                .unwrap()
                .iter()
                .all(|p| *p == [0.; 3])
        );
        update_bolt_positions(positions, &[None; MAX_BRANCHES], 9.);
        assert!(positions.iter().all(|p| *p == [0.; 3]));
    }

    #[test]
    fn discharge_visits_all_enemies_with_at_most_three_nearby_tips() {
        let mut world = World::new();
        let candidates: Vec<_> = (0_u16..7)
            .map(|i| (world.spawn_empty().id(), Vec3::X * f32::from(i)))
            .collect();
        let mut anchors = Vec::new();
        for pulse in 0..candidates.len() {
            let targets = select_arc_targets(candidates.clone(), pulse);
            let anchor = targets.first().copied().flatten().unwrap();
            anchors.push(anchor);
            let mut reversed = candidates.clone();
            reversed.reverse();
            assert_eq!(targets, select_arc_targets(reversed, pulse));
            assert_eq!(targets.iter().flatten().count(), MAX_BRANCHES);
            assert!(targets.iter().flatten().all(|p| p.distance(anchor) <= 2.));
        }
        assert!(candidates.iter().all(|(_, pos)| anchors.contains(pos)));
        assert_eq!(select_arc_targets(Vec::new(), 0), [None; MAX_BRANCHES]);
    }

    #[test]
    fn one_bolt_is_reused_deactivated_and_recreated_for_a_restored_tower() {
        let mut app = App::new();
        app.init_resource::<Time>()
            .init_resource::<Assets<Mesh>>()
            .insert_resource(TeslaEffectAssets {
                bolt_mat: default(),
                sparks: default(),
                smoke: default(),
            })
            .add_systems(Update, (maintain_arcs_system, update_arcs_system).chain());
        let enemy = app
            .world_mut()
            .spawn((
                Enemy::new(1, crate::game::enemy::EnemyKind::Normal),
                GlobalTransform::from_translation(Vec3::new(0.1, 0., 0.05)),
            ))
            .id();
        let tower = app
            .world_mut()
            .spawn((
                TeslaTower,
                TowerReady,
                EnemiesWithinReach(std::iter::once(enemy).collect()),
            ))
            .id();
        app.update();
        let bolt = app
            .world_mut()
            .query_filtered::<Entity, With<TeslaBolt>>()
            .single(app.world())
            .unwrap();
        assert_eq!(
            app.world().get::<Visibility>(bolt),
            Some(&Visibility::Inherited)
        );
        app.world_mut()
            .get_mut::<EnemiesWithinReach>(tower)
            .unwrap()
            .0
            .clear();
        app.update();
        assert_eq!(
            app.world().get::<Visibility>(bolt),
            Some(&Visibility::Hidden)
        );
        assert!(
            app.world_mut()
                .query::<&EffectSpawner>()
                .iter(app.world())
                .all(|s| !s.active)
        );
        assert_eq!(
            app.world_mut()
                .query_filtered::<Entity, With<TeslaBolt>>()
                .single(app.world())
                .unwrap(),
            bolt
        );
        app.world_mut().despawn(tower);
        assert!(app.world().get_entity(bolt).is_err());
        app.world_mut()
            .spawn((TeslaTower, TowerReady, EnemiesWithinReach::default()));
        app.update();
        assert!(
            app.world_mut()
                .query_filtered::<Entity, With<TeslaBolt>>()
                .single(app.world())
                .is_ok()
        );
    }
}

pub(in super::super) fn update_arcs_system(
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q_bolts: Query<
        (
            Entity,
            &RelEntity,
            &GlobalTransform,
            &Mesh3d,
            &mut Visibility,
        ),
        With<TeslaBolt>,
    >,
    q_towers: Query<&EnemiesWithinReach, (With<TeslaTower>, With<TowerReady>)>,
    q_targets: Query<&GlobalTransform, With<Enemy>>,
    mut q_sparks: Query<
        (
            &mut Transform,
            &mut EffectSpawner,
            &ChildOf,
            &TeslaImpactSlot,
        ),
        (
            With<TeslaImpactSparks>,
            Without<TeslaImpactSmoke>,
            Without<TeslaImpactFlash>,
        ),
    >,
    mut q_smoke: Query<
        (
            &mut Transform,
            &mut EffectSpawner,
            &ChildOf,
            &TeslaImpactSlot,
        ),
        (With<TeslaImpactSmoke>, Without<TeslaImpactFlash>),
    >,
    mut q_flash: Query<
        (&mut Transform, &mut PointLight, &ChildOf, &TeslaImpactSlot),
        With<TeslaImpactFlash>,
    >,
) {
    let pulse = usize::try_from(time.elapsed().as_millis() / 120).unwrap_or(0);
    let frame = (time.elapsed_secs() * 18.).floor();
    for (bolt_id, rel_id, bolt_gtf, mesh_handle, mut vis) in q_bolts.iter_mut() {
        let world_to_bolt = bolt_gtf.affine().inverse();
        let candidates = q_towers
            .get(rel_id.0)
            .map(|reach| {
                reach
                    .0
                    .iter()
                    .filter_map(|id| {
                        q_targets
                            .get(*id)
                            .ok()
                            .map(|gtf| (*id, world_to_bolt.transform_point3(gtf.translation())))
                    })
                    .collect()
            })
            .unwrap_or_default();
        let targets = select_arc_targets(candidates, pulse);
        *vis = if targets.iter().any(Option::is_some) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if let Some(mut mesh) = meshes.get_mut(&mesh_handle.0)
            && let Ok(VertexAttributeValues::Float32x3(pos)) =
                mesh.try_attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            update_bolt_positions(
                pos,
                &targets,
                frame + bolt_gtf.translation().length() * 137.,
            );
        }
        for (mut tf, mut spawner, child_of, slot) in q_sparks.iter_mut() {
            if child_of.parent() == bolt_id {
                let point = targets.get(slot.0).copied().flatten();
                tf.translation = point.unwrap_or_default();
                spawner.active = point.is_some();
            }
        }
        for (mut tf, mut spawner, child_of, slot) in q_smoke.iter_mut() {
            if child_of.parent() == bolt_id {
                let point = targets.get(slot.0).copied().flatten();
                tf.translation = point.unwrap_or_default();
                spawner.active = point.is_some();
            }
        }
        for (mut tf, mut light, child_of, slot) in q_flash.iter_mut() {
            if child_of.parent() == bolt_id {
                let point = targets.get(slot.0).copied().flatten();
                tf.translation = point.unwrap_or_default();
                let sin = (time.elapsed_secs() * 48.).sin();
                light.intensity = if point.is_some() {
                    (sin + 1.) * 160.
                } else {
                    0.
                };
            }
        }
    }
}
