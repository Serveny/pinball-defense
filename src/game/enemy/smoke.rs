use super::{EnemyKind, animation::EnemyVisual};
use crate::prelude::*;
use bevy::math::{Vec3, Vec4};
use bevy_hanabi::{
    AccelModifier, AlphaMode, Attribute, ColorOverLifetimeModifier, EffectAsset, Gradient,
    LinearDragModifier, Module, ParticleEffect, RoundModifier, SetAttributeModifier,
    SetPositionSphereModifier, SetVelocitySphereModifier, ShapeDimension, SimulationSpace,
    SizeOverLifetimeModifier, SpawnerSettings,
};

const CAPACITY: u32 = 64;

#[derive(Resource)]
pub(super) struct EnemySmokeAssets {
    smoke: Handle<EffectAsset>,
}

impl FromWorld for EnemySmokeAssets {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();
        Self {
            smoke: effects.add(asset()),
        }
    }
}

fn asset() -> EffectAsset {
    let mut module = Module::default();
    let center = module.lit(Vec3::ZERO);
    let radius = module.lit(0.006);
    let spread_speed = module.lit(0.012);
    let velocity = module.lit(Vec3::Z * 0.06);
    let age = module.lit(0.);
    let lifetime = module.lit(0.7);
    let accel = AccelModifier::constant(&mut module, Vec3::Z * 0.05);
    let drag = module.lit(0.8);
    let round = RoundModifier::ellipse(&mut module);

    let mut color = Gradient::new();
    color.add_key(0.0, Vec4::new(0.4, 0.38, 0.36, 0.));
    color.add_key(0.15, Vec4::new(0.4, 0.38, 0.36, 0.35));
    color.add_key(1.0, Vec4::new(0.35, 0.33, 0.31, 0.));

    let mut size = Gradient::new();
    size.add_key(0.0, Vec3::splat(0.012));
    size.add_key(1.0, Vec3::splat(0.045));

    EffectAsset::new(CAPACITY, SpawnerSettings::rate(7.0.into()), module)
        .with_name("enemy_smoke")
        .with_simulation_space(SimulationSpace::Global)
        .with_alpha_mode(AlphaMode::Blend)
        .init(SetPositionSphereModifier {
            center,
            radius,
            dimension: ShapeDimension::Volume,
        })
        .init(SetVelocitySphereModifier {
            center,
            speed: spread_speed,
        })
        .init(SetAttributeModifier::new(Attribute::VELOCITY, velocity))
        .init(SetAttributeModifier::new(Attribute::AGE, age))
        .init(SetAttributeModifier::new(Attribute::LIFETIME, lifetime))
        .update(accel)
        .update(LinearDragModifier::new(drag))
        .render(round)
        .render(ColorOverLifetimeModifier::new(color))
        .render(SizeOverLifetimeModifier {
            gradient: size,
            screen_space_size: false,
        })
}

#[derive(Component)]
pub(super) struct SmokeAttached;

pub(super) fn spawn_smoke_system(
    assets: Res<EnemySmokeAssets>,
    mut cmds: Commands,
    q_visuals: Query<(Entity, &EnemyVisual), Without<SmokeAttached>>,
) {
    for (visual_id, visual) in q_visuals.iter() {
        cmds.entity(visual_id)
            .insert(SmokeAttached)
            .with_children(|model| {
                model.spawn((
                    Name::new("Enemy Smoke"),
                    ParticleEffect::new(assets.smoke.clone()),
                    Transform::from_translation(match visual.kind {
                        EnemyKind::Normal => Vec3::new(0.012, 0.052, 0.),
                        EnemyKind::Tank => Vec3::new(-0.0134, 0.0694, 0.0023),
                        EnemyKind::Speeder => Vec3::new(-0.0154, -0.0054, 0.0368),
                    }),
                ));
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_asset_generates_shader() {
        assert!(bevy_hanabi::EffectShaderSources::generate(&asset(), None, 0).is_ok());
    }
}
