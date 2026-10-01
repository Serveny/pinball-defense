use super::super::TowerReady;
use super::super::target::{ConeAim, ConeFov, SightRadius, Targets};
use super::super::types::microwave::{MW_DISH_PIVOT, MW_EMITTER_LOCAL, MicrowaveTower};
use crate::game::enemy::Enemy;
use crate::prelude::*;
use crate::utils::RelEntity;
use bevy::asset::RenderAssetUsages;
use bevy::camera::visibility::NoFrustumCulling;
use bevy::light::NotShadowCaster;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::platform::collections::HashSet;
use bevy_hanabi::Gradient;
use bevy_hanabi::prelude::*;
use std::f32::consts::TAU;

const ROAD_Z: f32 = 0.002;
const FIELD_LIFT: f32 = 0.004;
const FIELD_SEGMENTS: u16 = 96;
const WAVE_SAMPLES: u32 = 192;
const RESONANCE_SAMPLES: u32 = 48;

#[derive(Resource)]
pub(in super::super) struct MicrowaveEffectAssets {
    waves: Handle<EffectAsset>,
    resonance: Handle<EffectAsset>,
    field: Handle<StandardMaterial>,
}

impl FromWorld for MicrowaveEffectAssets {
    fn from_world(world: &mut World) -> Self {
        let field = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                alpha_mode: bevy::prelude::AlphaMode::Add,
                cull_mode: None,
                ..default()
            });
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();
        Self {
            waves: effects.add(wave_asset(false)),
            resonance: effects.add(wave_asset(true)),
            field,
        }
    }
}

#[allow(clippy::too_many_lines)]
fn wave_asset(resonance: bool) -> EffectAsset {
    let w = ExprWriter::new();
    let range = w.add_property("range", 0.3_f32.into());
    let half_fov = w.add_property("half_fov", 22.5_f32.to_radians().into());
    let apex = w.add_property("apex", Vec3::ZERO.into());
    let beam = w.add_property("beam", Vec3::Y.into());
    let origin = w.add_property("origin", Vec3::ZERO.into());
    let lateral = w.add_property("side", Vec3::X.into());
    let up = w.add_property("up", Vec3::Z.into());
    let samples = if resonance {
        RESONANCE_SAMPLES
    } else {
        WAVE_SAMPLES
    };
    #[allow(clippy::cast_precision_loss)]
    let angle_step = TAU / if resonance { samples as f32 } else { 64. };
    let angle = (w.attr(Attribute::PARTICLE_COUNTER) % w.lit(if resonance { samples } else { 64 }))
        .cast(ScalarType::Float)
        * w.lit(angle_step);
    let init_angle = SetAttributeModifier::new(Attribute::F32_0, angle.expr());
    let tier = (w.attr(Attribute::PARTICLE_COUNTER) / w.lit(64_u32) % w.lit(3_u32))
        .cast(ScalarType::Float);
    let init_tier =
        SetAttributeModifier::new(Attribute::F32_1, (w.lit(0.3) + tier * w.lit(0.35)).expr());
    let age = w.attr(Attribute::AGE);
    let theta = w.attr(Attribute::F32_0);
    let wobble = (theta.clone() * w.lit(7.) - age.clone() * w.lit(24.)).sin();
    let radius = if resonance {
        w.lit(0.038) + age.clone() * w.lit(0.025) + wobble * w.lit(0.002)
    } else {
        w.prop(range) * age.clone() / w.attr(Attribute::LIFETIME)
    };
    let spread = radius.clone()
        * (w.prop(half_fov).sin() / w.prop(half_fov).cos())
        * w.attr(Attribute::F32_1);
    let position = if resonance {
        (radius.clone() * theta.clone().cos()).vec3(
            radius * theta.clone().sin(),
            w.lit(0.016) + age.clone() * w.lit(0.02),
        )
    } else {
        let theta = theta.clone() + (age * w.lit(24.)).sin() * w.lit(0.025);
        w.prop(origin)
            + w.prop(beam) * radius
            + w.prop(lateral) * spread.clone() * theta.clone().cos()
            + w.prop(up) * spread.clone() * theta.sin()
    };
    let set_position = SetAttributeModifier::new(Attribute::POSITION, position.expr());
    let tangent = if resonance {
        (theta.clone().sin() * w.lit(-1.)).vec3(theta.cos(), w.lit(0.))
    } else {
        w.prop(lateral) * theta.clone().sin() * w.lit(-1.) + w.prop(up) * theta.cos()
    };
    let set_tangent = SetAttributeModifier::new(Attribute::VELOCITY, tangent.expr());
    let size = if resonance {
        w.lit(0.004)
    } else {
        let delta = w.attr(Attribute::POSITION) - w.prop(apex);
        let projection = delta.clone().dot(w.prop(beam));
        let threshold = delta.length() * w.prop(half_fov).cos();
        let mask = projection.step(threshold);
        let reach_mask = w.prop(range).step(
            w.attr(Attribute::POSITION)
                .x()
                .vec2(w.attr(Attribute::POSITION).y())
                .length(),
        );
        let above_ground = w.attr(Attribute::POSITION).z().step(w.lit(0.));
        let below_emitter =
            (w.prop(origin).z() + w.lit(0.005)).step(w.attr(Attribute::POSITION).z());
        (spread * w.lit(angle_step * 1.35)).max(w.lit(0.002))
            * mask
            * reach_mask
            * above_ground
            * below_emitter
    };
    let width = size
        .clone()
        .min(w.lit(if resonance { 0.004 } else { 0.0025 }));
    let set_size = SetAttributeModifier::new(Attribute::SIZE2, size.vec2(width).expr());
    let init_age = SetAttributeModifier::new(Attribute::AGE, w.lit(0.).expr());
    let init_life = SetAttributeModifier::new(
        Attribute::LIFETIME,
        w.lit(if resonance { 0.45 } else { 0.95 }).expr(),
    );
    let mut module = w.finish();
    let round = RoundModifier::ellipse(&mut module);
    let mut color = Gradient::new();
    color.add_key(0., Vec4::new(2.4, 0.65, 0.12, 0.));
    color.add_key(0.12, Vec4::new(2.4, 0.9, 0.2, 0.65));
    color.add_key(1., Vec4::new(0.8, 0.2, 0.5, 0.));
    #[allow(clippy::cast_precision_loss)]
    let count = samples as f32;
    EffectAsset::new(
        samples * 8,
        SpawnerSettings::burst(count.into(), 0.16.into()).with_starts_active(false),
        module,
    )
    .with_name(if resonance {
        "microwave_resonance"
    } else {
        "microwave_wavefronts"
    })
    .with_simulation_space(SimulationSpace::Local)
    .with_motion_integration(MotionIntegration::None)
    .with_alpha_mode(bevy_hanabi::AlphaMode::Add)
    .init(init_age)
    .init(init_life)
    .init(init_angle)
    .init(init_tier)
    .init(set_position)
    .init(set_tangent)
    .init(set_size)
    .update(set_position)
    .update(set_tangent)
    .update(set_size)
    .render(OrientModifier::new(OrientMode::AlongVelocity))
    .render(round)
    .render(ColorOverLifetimeModifier::new(color))
}

#[derive(Component)]
pub(in super::super) struct MicrowaveFxAttached;

#[derive(Component)]
pub(in super::super) struct MicrowaveField;

#[derive(Component)]
pub(in super::super) struct MicrowaveResonance;

pub(in super::super) fn attach_effects_system(
    mut cmds: Commands,
    assets: Res<MicrowaveEffectAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    q_towers: Query<Entity, (With<MicrowaveTower>, Without<MicrowaveFxAttached>)>,
    q_enemies: Query<Entity, (With<Enemy>, Without<MicrowaveFxAttached>)>,
) {
    for id in &q_towers {
        cmds.entity(id)
            .insert(MicrowaveFxAttached)
            .with_children(|p| {
                p.spawn((
                    Name::new("Microwave Influence Area"),
                    Mesh3d(meshes.add(field_mesh(0.3, 0.4, Vec3::ZERO, Vec3::Y))),
                    MeshMaterial3d(assets.field.clone()),
                    ParticleEffect::new(assets.waves.clone()),
                    EffectProperties::default(),
                    Visibility::Hidden,
                    NotShadowCaster,
                    NoFrustumCulling,
                    MicrowaveField,
                ));
            });
    }
    for id in &q_enemies {
        cmds.entity(id)
            .insert(MicrowaveFxAttached)
            .with_children(|p| {
                p.spawn((
                    Name::new("Microwave Slow Resonance"),
                    ParticleEffect::new(assets.resonance.clone()),
                    Visibility::Hidden,
                    MicrowaveResonance,
                ));
            });
    }
}

fn field_mesh(range: f32, half_fov: f32, apex: Vec3, beam: Vec3) -> Mesh {
    let direction = beam.truncate().try_normalize().unwrap_or(Vec2::Y);
    let side = Vec2::new(-direction.y, direction.x);
    let ax = apex.truncate().dot(direction);
    let ay = apex.truncate().dot(side);
    let mut positions = Vec::new();
    let mut colors = Vec::new();
    for i in 0..=FIELD_SEGMENTS {
        let x = range * (2. * f32::from(i) / f32::from(FIELD_SEGMENTS) - 1.);
        let circle = (range * range - x * x).max(0.).sqrt();
        let along = x - ax;
        let projection = along * beam.truncate().length() - apex.z * beam.z;
        let width_squared = (projection / half_fov.cos()).powi(2) - along * along - apex.z * apex.z;
        let width = width_squared.max(0.).sqrt();
        let lower = (ay - width).max(-circle);
        let upper = (ay + width).min(circle);
        let valid = projection > 0. && width_squared > 0. && upper > lower;
        let (lower, upper) = if valid {
            (lower, upper)
        } else {
            let y = ay.clamp(-circle, circle);
            (y, y)
        };
        let edge = 0.003_f32.min((upper - lower) * 0.5);
        for (y, alpha) in [
            (lower, 0.35),
            (lower + edge, 0.035),
            (upper - edge, 0.035),
            (upper, 0.35),
        ] {
            let point = direction * x + side * y;
            positions.push([point.x, point.y, 0.]);
            colors.push([1.8, 0.65, 0.12, if valid { alpha } else { 0. }]);
        }
    }
    let mut indices = Vec::new();
    for i in 0..FIELD_SEGMENTS {
        for column in 0..3 {
            let base = i * 4 + column;
            indices.extend([base, base + 4, base + 1, base + 1, base + 4, base + 5]);
        }
    }
    let count = positions.len();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 0., 1.]; count])
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; count])
    .with_inserted_indices(Indices::U16(indices))
}

pub(in super::super) fn update_effects_system(
    mut meshes: ResMut<Assets<Mesh>>,
    q_towers: Query<
        (
            &SightRadius,
            &ConeFov,
            &Targets,
            &GlobalTransform,
            Has<TowerReady>,
        ),
        With<MicrowaveTower>,
    >,
    q_aim: Query<(&GlobalTransform, &ConeAim, &RelEntity)>,
    mut q_fields: Query<
        (
            &Mesh3d,
            &mut Transform,
            &mut Visibility,
            &mut EffectProperties,
            Option<&mut EffectSpawner>,
            &ChildOf,
        ),
        With<MicrowaveField>,
    >,
    mut q_resonance: Query<
        (&mut Visibility, Option<&mut EffectSpawner>, &ChildOf),
        (With<MicrowaveResonance>, Without<MicrowaveField>),
    >,
) {
    let affected: HashSet<_> = q_towers
        .iter()
        .filter(|(_, _, _, _, ready)| *ready)
        .flat_map(|(_, _, targets, _, _)| targets.0.iter().copied())
        .collect();
    for (mesh, mut transform, mut visibility, mut properties, spawner, parent) in &mut q_fields {
        let Ok((range, fov, targets, tower_global, ready)) = q_towers.get(parent.parent()) else {
            continue;
        };
        let active = ready && !targets.0.is_empty();
        *visibility = if active {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if let Some(mut spawner) = spawner {
            spawner.active = active;
        }
        let Some((aim_global, aim, _)) = q_aim.iter().find(|(_, _, rel)| rel.0 == parent.parent())
        else {
            continue;
        };
        let inverse = tower_global.affine().inverse();
        let ground_z = ROAD_Z - tower_global.translation().z;
        let apex = inverse.transform_point3(aim_global.translation()) - Vec3::Z * ground_z;
        let beam = inverse
            .transform_vector3(aim_global.affine().transform_vector3(aim.0))
            .normalize_or_zero();
        transform.translation.z = ground_z + FIELD_LIFT;
        if active && let Some(mut mesh) = meshes.get_mut(&mesh.0) {
            *mesh = field_mesh(range.0, fov.0 / 2., apex, beam);
        }
        properties.set("range", range.0.into());
        properties.set("half_fov", (fov.0 / 2.).into());
        properties.set("apex", apex.into());
        properties.set("beam", beam.into());
        properties.set(
            "origin",
            (inverse.transform_point3(
                aim_global
                    .affine()
                    .transform_point3(MW_EMITTER_LOCAL - MW_DISH_PIVOT),
            ) - Vec3::Z * ground_z)
                .into(),
        );
        let side = beam.cross(Vec3::Z).try_normalize().unwrap_or(Vec3::X);
        properties.set("side", side.into());
        properties.set("up", side.cross(beam).into());
    }
    for (mut visibility, spawner, parent) in &mut q_resonance {
        let active = affected.contains(&parent.parent());
        *visibility = if active {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if let Some(mut spawner) = spawner {
            spawner.active = active;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::mesh::VertexAttributeValues;

    #[test]
    fn builtin_wave_and_resonance_shaders_generate() {
        for resonance in [false, true] {
            assert!(EffectShaderSources::generate(&wave_asset(resonance), None, 0).is_ok());
        }
    }

    #[test]
    fn footprint_matches_the_pitched_cone_and_upgraded_range() {
        let apex = Vec3::new(0., 0.075, 0.075);
        let beam = Vec3::new(0., 1., -0.5).normalize();
        for (range, half_fov) in [(0.3, 22.5_f32.to_radians()), (0.6, 45_f32.to_radians())] {
            let mesh = field_mesh(range, half_fov, apex, beam);
            let VertexAttributeValues::Float32x3(positions) =
                mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
            else {
                panic!("positions")
            };
            let VertexAttributeValues::Float32x4(colors) =
                mesh.attribute(Mesh::ATTRIBUTE_COLOR).unwrap()
            else {
                panic!("colors")
            };
            let mut maximum = 0_f32;
            for (position, color) in positions.iter().zip(colors) {
                if color.last().copied().unwrap() == 0. {
                    continue;
                }
                let point = Vec3::from_array(*position);
                maximum = maximum.max(point.length());
                assert!(point.length() <= range + 0.0001);
                assert!(beam.angle_between(point - apex) <= half_fov + 0.0001);
            }
            assert!(maximum > range * 0.98);
        }
    }

    #[test]
    fn effects_follow_targets_upgrades_and_restored_entities() {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .insert_resource(MicrowaveEffectAssets {
                waves: default(),
                resonance: default(),
                field: default(),
            })
            .add_systems(
                Update,
                (attach_effects_system, update_effects_system).chain(),
            );
        let enemy = app
            .world_mut()
            .spawn(Enemy::new(1, crate::game::enemy::EnemyKind::Normal))
            .id();
        let outside = app
            .world_mut()
            .spawn(Enemy::new(1, crate::game::enemy::EnemyKind::Normal))
            .id();
        let tower = app
            .world_mut()
            .spawn((
                MicrowaveTower,
                TowerReady,
                SightRadius(0.3),
                ConeFov(45_f32.to_radians()),
                Targets(std::iter::once(enemy).collect()),
                GlobalTransform::IDENTITY,
            ))
            .id();
        app.world_mut().spawn((
            GlobalTransform::from_translation(Vec3::new(0., 0.075, 0.075)),
            ConeAim(Vec3::new(0., 1., -0.5).normalize()),
            RelEntity(tower),
        ));
        app.update();
        let field = app
            .world_mut()
            .query_filtered::<Entity, With<MicrowaveField>>()
            .single(app.world())
            .unwrap();
        let aura = app
            .world_mut()
            .query_filtered::<(Entity, &ChildOf), With<MicrowaveResonance>>()
            .iter(app.world())
            .find(|(_, parent)| parent.parent() == enemy)
            .unwrap()
            .0;
        assert_eq!(
            app.world().get::<Visibility>(field),
            Some(&Visibility::Inherited)
        );
        assert_eq!(
            app.world().get::<Visibility>(aura),
            Some(&Visibility::Inherited)
        );
        let inactive = app
            .world_mut()
            .query_filtered::<(Entity, &ChildOf), With<MicrowaveResonance>>()
            .iter(app.world())
            .find(|(_, parent)| parent.parent() == outside)
            .unwrap()
            .0;
        assert_eq!(
            app.world().get::<Visibility>(inactive),
            Some(&Visibility::Hidden)
        );
        app.world_mut()
            .entity_mut(field)
            .insert(EffectSpawner::new(&SpawnerSettings::rate(1_f32.into())));
        app.world_mut()
            .entity_mut(aura)
            .insert(EffectSpawner::new(&SpawnerSettings::rate(1_f32.into())));
        app.world_mut().get_mut::<SightRadius>(tower).unwrap().0 = 0.6;
        app.world_mut().get_mut::<ConeFov>(tower).unwrap().0 = 90_f32.to_radians();
        app.update();
        let properties = app.world().get::<EffectProperties>(field).unwrap();
        assert_eq!(properties.get_stored("range"), Some(0.6_f32.into()));
        let emitter =
            Vec3::new(0., 0.075, 0.075) + (MW_EMITTER_LOCAL - MW_DISH_PIVOT) - Vec3::Z * ROAD_Z;
        assert_eq!(properties.get_stored("origin"), Some(emitter.into()));
        assert_eq!(
            properties.get_stored("half_fov"),
            Some(45_f32.to_radians().into())
        );
        app.world_mut().get_mut::<Targets>(tower).unwrap().0.clear();
        app.update();
        for id in [field, aura] {
            assert_eq!(app.world().get::<Visibility>(id), Some(&Visibility::Hidden));
            assert!(!app.world().get::<EffectSpawner>(id).unwrap().active);
        }
        app.world_mut().despawn(tower);
        assert!(app.world().get_entity(field).is_err());
        app.world_mut().despawn(enemy);
        assert!(app.world().get_entity(aura).is_err());
        app.world_mut().spawn((
            MicrowaveTower,
            SightRadius(0.6),
            ConeFov(90_f32.to_radians()),
            Targets::default(),
            GlobalTransform::IDENTITY,
        ));
        app.update();
        assert_eq!(
            app.world_mut()
                .query_filtered::<Entity, With<MicrowaveField>>()
                .iter(app.world())
                .count(),
            1
        );
        assert_eq!(
            app.world_mut()
                .query_filtered::<Entity, With<MicrowaveResonance>>()
                .iter(app.world())
                .count(),
            1
        );
    }
}
