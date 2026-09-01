use bevy::math::{Vec3, Vec4};
use bevy_hanabi::prelude::*;

const BURST_PERIOD: f32 = 0.1;
const CAPACITY: u32 = 64;
const VEL_ORIGIN: Vec3 = Vec3::new(0., -0.02, 0.);

pub(super) fn asset() -> EffectAsset {
    let mut module = Module::default();
    let center = module.lit(Vec3::ZERO);
    let radius = module.lit(0.002);
    let vel_origin = module.lit(VEL_ORIGIN);
    let speed = module.lit(0.7);
    let age = module.lit(0.);
    let lifetime = module.lit(0.04);
    let drag = module.lit(4.);
    let round = RoundModifier::ellipse(&mut module);

    let mut color = Gradient::new();
    color.add_key(0.0, Vec4::new(12., 9., 3., 1.));
    color.add_key(0.35, Vec4::new(6., 3., 1., 0.8));
    color.add_key(1.0, Vec4::ZERO);

    let mut size = Gradient::new();
    size.add_key(0.0, Vec3::splat(0.05));
    size.add_key(1.0, Vec3::splat(0.005));

    EffectAsset::new(
        CAPACITY,
        SpawnerSettings::burst(24.0.into(), BURST_PERIOD.into()).with_starts_active(false),
        module,
    )
    .with_name("mg_muzzle_flash")
    .with_simulation_space(SimulationSpace::Local)
    .with_alpha_mode(AlphaMode::Add)
    .init(SetPositionSphereModifier {
        center,
        radius,
        dimension: ShapeDimension::Volume,
    })
    .init(SetVelocitySphereModifier {
        center: vel_origin,
        speed,
    })
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
