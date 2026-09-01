use bevy::math::{Vec3, Vec4};
use bevy_hanabi::prelude::*;

const CAPACITY: u32 = 64;

pub(super) fn asset() -> EffectAsset {
    let mut module = Module::default();
    let center = module.lit(Vec3::ZERO);
    let radius = module.lit(0.012);
    let spread_speed = module.lit(0.02);
    let velocity = module.lit(Vec3::Z * 0.04);
    let age = module.lit(0.);
    let lifetime = module.lit(0.8);
    let accel = AccelModifier::constant(&mut module, Vec3::Z * 0.04);
    let drag = module.lit(0.6);
    let round = RoundModifier::ellipse(&mut module);

    let mut color = Gradient::new();
    color.add_key(0.0, Vec4::new(0.35, 0.33, 0.3, 0.));
    color.add_key(0.15, Vec4::new(0.35, 0.33, 0.3, 0.6));
    color.add_key(1.0, Vec4::new(0.3, 0.28, 0.26, 0.));

    let mut size = Gradient::new();
    size.add_key(0.0, Vec3::splat(0.02));
    size.add_key(1.0, Vec3::splat(0.09));

    EffectAsset::new(
        CAPACITY,
        SpawnerSettings::rate(12.0.into()).with_starts_active(false),
        module,
    )
    .with_name("mg_barrel_smoke")
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