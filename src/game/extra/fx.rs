use super::{ActiveEffects, ExtraFieldKind};
use crate::game::ball::PinBall;
use crate::game::enemy::Enemy;
use crate::game::IngameTime;
use crate::prelude::*;
use bevy::math::{Vec3, Vec4};
use bevy_hanabi::{
    AccelModifier, AlphaMode, Attribute, ColorOverLifetimeModifier, EffectAsset, EffectSpawner,
    Gradient, LinearDragModifier, Module, ParticleEffect, RoundModifier, SetAttributeModifier,
    SetPositionSphereModifier, SetVelocitySphereModifier, SetVelocityTangentModifier,
    ShapeDimension, SimulationSpace, SizeOverLifetimeModifier, SpawnerSettings,
};

const RED_TRAIL: Vec4 = Vec4::new(3., 0.3, 0.2, 1.);
const ORANGE_TRAIL: Vec4 = Vec4::new(3., 1.3, 0.2, 1.);

#[derive(Resource)]
pub(super) struct ExtraFxAssets {
    insta_trail: Handle<EffectAsset>,
    double_damage_trail: Handle<EffectAsset>,
    snow: Handle<EffectAsset>,
}

impl FromWorld for ExtraFxAssets {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();
        Self {
            insta_trail: effects.add(trail_asset("ball_insta_kill_trail", RED_TRAIL)),
            double_damage_trail: effects.add(trail_asset("ball_double_damage_trail", ORANGE_TRAIL)),
            snow: effects.add(snow_asset()),
        }
    }
}

fn trail_asset(name: &str, base: Vec4) -> EffectAsset {
    let mut module = Module::default();
    let center = module.lit(Vec3::ZERO);
    let radius = module.lit(0.008);
    let speed = module.lit(0.02);
    let age = module.lit(0.);
    let lifetime = module.lit(0.35);
    let drag = module.lit(2.);
    let round = RoundModifier::ellipse(&mut module);

    let mut color = Gradient::new();
    color.add_key(0., base);
    color.add_key(0.5, base * Vec4::new(0.6, 0.6, 0.6, 0.7));
    color.add_key(1., Vec4::ZERO);

    let mut size = Gradient::new();
    size.add_key(0., Vec3::splat(0.022));
    size.add_key(1., Vec3::splat(0.004));

    EffectAsset::new(
        256,
        SpawnerSettings::rate(200.0.into()).with_starts_active(false),
        module,
    )
    .with_name(name)
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
    .update(LinearDragModifier::new(drag))
    .render(round)
    .render(ColorOverLifetimeModifier::new(color))
    .render(SizeOverLifetimeModifier {
        gradient: size,
        screen_space_size: false,
    })
}

fn snow_asset() -> EffectAsset {
    let mut module = Module::default();
    let origin = module.lit(Vec3::ZERO);
    let radius = module.lit(0.035);
    let swirl = module.lit(0.12);
    let axis = module.lit(Vec3::Z);
    let age = module.lit(0.);
    let lifetime = module.lit(0.6);
    let drag = module.lit(1.2);
    let gravity = AccelModifier::constant(&mut module, Vec3::Z * -0.12);
    let round = RoundModifier::ellipse(&mut module);

    let mut color = Gradient::new();
    color.add_key(0., Vec4::new(0.7, 0.85, 1., 0.));
    color.add_key(0.2, Vec4::new(0.8, 0.95, 1.2, 0.6));
    color.add_key(1., Vec4::new(0.6, 0.8, 1., 0.));

    let mut size = Gradient::new();
    size.add_key(0., Vec3::splat(0.004));
    size.add_key(1., Vec3::splat(0.008));

    EffectAsset::new(
        256,
        SpawnerSettings::rate(80.0.into()).with_starts_active(false),
        module,
    )
    .with_name("freeze_snow")
    .with_simulation_space(SimulationSpace::Local)
    .with_alpha_mode(AlphaMode::Blend)
    .init(SetPositionSphereModifier {
        center: origin,
        radius,
        dimension: ShapeDimension::Volume,
    })
    .init(SetVelocityTangentModifier {
        origin,
        axis,
        speed: swirl,
    })
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

#[derive(Component)]
pub(in super::super) struct TrailFx(ExtraFieldKind);

#[derive(Component)]
pub(in super::super) struct SnowFx;

#[derive(Component)]
pub(in super::super) struct FxAttached;

pub(super) fn spawn_ball_trails_system(
    assets: Res<ExtraFxAssets>,
    mut cmds: Commands,
    q_balls: Query<Entity, (With<PinBall>, Without<FxAttached>)>,
) {
    for ball_id in q_balls.iter() {
        cmds.entity(ball_id).insert(FxAttached).with_children(|ball| {
            for (name, handle, kind) in [
                (
                    "InstaKill Trail",
                    assets.insta_trail.clone(),
                    ExtraFieldKind::InstaKill,
                ),
                (
                    "Double Damage Trail",
                    assets.double_damage_trail.clone(),
                    ExtraFieldKind::DoubleDamage,
                ),
            ] {
                ball.spawn((
                    Name::new(name),
                    ParticleEffect::new(handle),
                    TrailFx(kind),
                    Transform::default(),
                ));
            }
        });
    }
}

pub(super) fn spawn_enemy_snow_system(
    assets: Res<ExtraFxAssets>,
    mut cmds: Commands,
    q_enemies: Query<Entity, (With<Enemy>, Without<FxAttached>)>,
) {
    for enemy_id in q_enemies.iter() {
        cmds.entity(enemy_id).insert(FxAttached).with_children(|enemy| {
            enemy.spawn((
                Name::new("Freeze Snow"),
                ParticleEffect::new(assets.snow.clone()),
                SnowFx,
                Transform::default(),
            ));
        });
    }
}

pub(super) fn toggle_fx_system(
    effects: Res<ActiveEffects>,
    ig_time: Res<IngameTime>,
    mut q_trails: Query<(&mut EffectSpawner, &TrailFx)>,
    mut q_snow: Query<&mut EffectSpawner, (With<SnowFx>, Without<TrailFx>)>,
) {
    let now = **ig_time;
    let insta = effects.is_active(now, ExtraFieldKind::InstaKill);
    let double = effects.is_active(now, ExtraFieldKind::DoubleDamage);
    let frozen = effects.is_active(now, ExtraFieldKind::SlowDown);
    for (mut spawner, trail) in q_trails.iter_mut() {
        let active = match trail.0 {
            ExtraFieldKind::InstaKill => insta,
            ExtraFieldKind::DoubleDamage => double && !insta,
            _ => false,
        };
        if spawner.active != active {
            spawner.active = active;
        }
    }
    for mut spawner in q_snow.iter_mut() {
        if spawner.active != frozen {
            spawner.active = frozen;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trail_asset_compiles() {
        let _ = trail_asset("test", Vec4::new(1., 0., 0., 1.));
        let _ = snow_asset();
    }
}