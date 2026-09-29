use super::EnemyKind;
use crate::prelude::*;
use bevy::math::{Vec3, Vec4};
use bevy_hanabi::{
    AccelModifier, AlphaMode, Attribute, ColorOverLifetimeModifier, EffectAsset, Gradient,
    LinearDragModifier, Module, ParticleEffect, RoundModifier, SetAttributeModifier,
    SetPositionSphereModifier, SetVelocitySphereModifier, ShapeDimension, SimulationSpace,
    SizeOverLifetimeModifier, SpawnerSettings,
};

const DESPAWN_SECS: f32 = 1.5;

#[derive(Resource)]
pub(super) struct ExplosionAssets {
    normal: Handle<EffectAsset>,
    tank: Handle<EffectAsset>,
    speeder_burst: Handle<EffectAsset>,
    speeder_trail: Handle<EffectAsset>,
}

impl FromWorld for ExplosionAssets {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();
        Self {
            normal: effects.add(burst_asset("enemy_explosion", 32., 0.12, 0.35)),
            tank: effects.add(burst_asset("tank_explosion", 96., 0.22, 0.6)),
            speeder_burst: effects.add(burst_asset("speeder_explosion", 24., 0.1, 0.3)),
            speeder_trail: effects.add(trail_asset()),
        }
    }
}

fn burst_asset(name: &str, count: f32, radius: f32, speed: f32) -> EffectAsset {
    let mut module = Module::default();
    let center = module.lit(Vec3::ZERO);
    let pos_radius = module.lit(0.01);
    let spread_speed = module.lit(speed);
    let age = module.lit(0.);
    let lifetime = module.lit(0.6);
    let drag = module.lit(1.5);
    let accel = AccelModifier::constant(&mut module, Vec3::Z * 0.05);
    let round = RoundModifier::ellipse(&mut module);

    let mut color = Gradient::new();
    color.add_key(0.0, Vec4::new(10., 6., 1.5, 1.));
    color.add_key(0.2, Vec4::new(5., 1.8, 0.4, 0.9));
    color.add_key(0.6, Vec4::new(0.35, 0.33, 0.31, 0.5));
    color.add_key(1.0, Vec4::new(0.3, 0.28, 0.26, 0.));

    let mut size = Gradient::new();
    size.add_key(0.0, Vec3::splat(radius * 0.3));
    size.add_key(0.15, Vec3::splat(radius));
    size.add_key(1.0, Vec3::splat(radius * 0.4));

    EffectAsset::new(256, SpawnerSettings::once(count.into()), module)
        .with_name(name)
        .with_simulation_space(SimulationSpace::Global)
        .with_alpha_mode(AlphaMode::Blend)
        .init(SetPositionSphereModifier {
            center,
            radius: pos_radius,
            dimension: ShapeDimension::Volume,
        })
        .init(SetVelocitySphereModifier {
            center,
            speed: spread_speed,
        })
        .init(SetAttributeModifier::new(Attribute::AGE, age))
        .init(SetAttributeModifier::new(Attribute::LIFETIME, lifetime))
        .update(LinearDragModifier::new(drag))
        .update(accel)
        .render(round)
        .render(ColorOverLifetimeModifier::new(color))
        .render(SizeOverLifetimeModifier {
            gradient: size,
            screen_space_size: false,
        })
}

const TRAIL_LIFETIME: f32 = 0.4;

fn trail_asset() -> EffectAsset {
    let mut module = Module::default();
    let center = module.lit(Vec3::ZERO);
    let radius = module.lit(0.004);
    let speed = module.lit(0.01);
    let age = module.lit(0.);
    let lifetime = module.lit(TRAIL_LIFETIME);
    let drag = module.lit(1.);
    let round = RoundModifier::ellipse(&mut module);

    let mut color = Gradient::new();
    color.add_key(0.0, Vec4::new(0.9, 0.5, 0.1, 0.8));
    color.add_key(0.3, Vec4::new(0.6, 0.25, 0.05, 0.6));
    color.add_key(1.0, Vec4::new(0.2, 0.18, 0.16, 0.));

    let mut size = Gradient::new();
    size.add_key(0.0, Vec3::splat(0.015));
    size.add_key(1.0, Vec3::splat(0.03));

    // ponytail: one emitter reused per death-position (moved along road) approximates a trail
    EffectAsset::new(64, SpawnerSettings::rate(40.0.into()), module)
        .with_name("speeder_trail")
        .with_simulation_space(SimulationSpace::Global)
        .with_alpha_mode(AlphaMode::Blend)
        .init(SetPositionSphereModifier {
            center,
            radius,
            dimension: ShapeDimension::Volume,
        })
        .init(SetVelocitySphereModifier { center, speed })
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

#[derive(Component)]
pub(super) struct ExplosionFx {
    timer: Timer,
}

pub(super) fn spawn_explosion(
    cmds: &mut Commands,
    assets: &ExplosionAssets,
    pos: Vec3,
    kind: EnemyKind,
) {
    let burst = match kind {
        EnemyKind::Normal => assets.normal.clone(),
        EnemyKind::Tank => assets.tank.clone(),
        EnemyKind::Speeder => assets.speeder_burst.clone(),
    };
    cmds.spawn((
        Name::new("Enemy Explosion"),
        ParticleEffect::new(burst),
        Transform::from_translation(pos),
        ExplosionFx {
            timer: Timer::from_seconds(DESPAWN_SECS, TimerMode::Once),
        },
    ));
    if kind == EnemyKind::Speeder {
        cmds.spawn((
            Name::new("Speeder Trail"),
            ParticleEffect::new(assets.speeder_trail.clone()),
            Transform::from_translation(pos),
            ExplosionFx {
                timer: Timer::from_seconds(TRAIL_LIFETIME + 0.2, TimerMode::Once),
            },
        ));
    }
}

pub(super) fn despawn_explosion_system(
    mut cmds: Commands,
    time: Res<Time>,
    mut q_fx: Query<(Entity, &mut ExplosionFx)>,
) {
    for (fx_id, mut fx) in q_fx.iter_mut() {
        fx.timer.tick(time.delta());
        if fx.timer.is_finished() {
            cmds.entity(fx_id).try_despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explosion_assets_generate_shader() {
        for asset in [
            burst_asset("t", 32., 0.12, 0.35),
            trail_asset(),
        ] {
            assert!(bevy_hanabi::EffectShaderSources::generate(&asset, None, 0).is_ok());
        }
    }
}
