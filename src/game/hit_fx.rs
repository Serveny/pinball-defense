use super::ball::{BALL_RADIUS, CollisionWithBallEvent, PinBall};
use super::enemy::Enemy;
use super::tower::{Tower, foundation::TowerFoundation};
use super::{GameState, ball, enemy};
use crate::AppState;
use crate::prelude::*;
use bevy_hanabi::{
    AccelModifier, AlphaMode, Attribute, ColorOverLifetimeModifier, EffectAsset, EffectProperties,
    EffectSimulation, EffectSpawner, Gradient, LinearDragModifier, Module, OrientMode,
    OrientModifier, ParticleEffect, ScreenSpaceSizeModifier, SetAttributeModifier,
    SetPositionSphereModifier, ShapeDimension, SimulationCondition, SimulationSpace,
    SizeOverLifetimeModifier, SpawnerSettings,
};

pub(super) struct HitFxPlugin;

impl Plugin for HitFxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HitFxAssets>()
            .add_systems(
                Update,
                (
                    spawn_hit_fx_system
                        .after(ball::on_collision_with_ball_system)
                        .after(ball::enemy_ball_overlap_system)
                        .before(enemy::on_health_empty_system),
                    despawn_hit_fx_system,
                )
                    .run_if(in_state(GameState::Ingame)),
            )
            .add_systems(OnExit(AppState::Game), clear_hit_fx_system)
            .add_systems(OnExit(GameState::GameOver), clear_hit_fx_system);
    }
}

#[derive(Resource)]
struct HitFxAssets {
    tower: Handle<EffectAsset>,
    enemy: Handle<EffectAsset>,
}

const PARTICLE_LIFETIME: f32 = 0.45;

impl FromWorld for HitFxAssets {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();
        Self {
            tower: effects.add(hit_asset(true)),
            enemy: effects.add(hit_asset(false)),
        }
    }
}

fn hit_asset(tower: bool) -> EffectAsset {
    let mut module = Module::default();
    let center = module.lit(Vec3::ZERO);
    let radius = module.lit(0.002);
    let min_angle = module.lit(-std::f32::consts::FRAC_PI_2);
    let max_angle = module.lit(std::f32::consts::FRAC_PI_2);
    let spread = module.uniform(min_angle, max_angle);
    let hit_angle = module.add_property("hit_angle", 0_f32.into());
    let hit_angle = module.prop(hit_angle);
    let angle = module.add(hit_angle, spread);
    let x = module.cos(angle);
    let y = module.sin(angle);
    let z = module.lit(0.);
    let direction = module.vec3(x, y, z);
    let speed = module.lit(if tower { 0.45 / 3. } else { 0.32 / 3. });
    let velocity = module.mul(direction, speed);
    let age = module.lit(0.);
    let lifetime = module.lit(PARTICLE_LIFETIME);
    let drag = module.lit(3.);
    let accel = AccelModifier::constant(&mut module, Vec3::Z * 0.08);

    let mut color = Gradient::new();
    let tint = if tower {
        Vec4::new(0.05, 0.55, 0.2, 1.)
    } else {
        Vec4::new(1.2, 0.14, 0.05, 1.)
    };
    color.add_key(0., tint * Vec4::new(2., 2., 2., 1.));
    color.add_key(0.15, tint);
    color.add_key(0.65, tint * Vec4::new(0.8, 0.8, 0.8, 0.8));
    color.add_key(1., Vec4::ZERO);

    let mut size = Gradient::new();
    let initial_size = if tower {
        Vec3::new(12., 8. / 3., 1.)
    } else {
        Vec3::new(16. / 3., 16. / 3., 1.)
    };
    size.add_key(0., initial_size);
    size.add_key(0.3, initial_size);
    size.add_key(1., initial_size * Vec3::new(0.3, 0.3, 1.));

    EffectAsset::new(
        32,
        SpawnerSettings::once(if tower { 14.0 } else { 12.0 }.into()),
        module,
    )
    .with_name(if tower { "tower_hit" } else { "enemy_hit" })
    .with_simulation_space(SimulationSpace::Global)
    .with_simulation_condition(SimulationCondition::Always)
    .with_alpha_mode(AlphaMode::Blend)
    .init(SetPositionSphereModifier {
        center,
        radius,
        dimension: ShapeDimension::Volume,
    })
    .init(SetAttributeModifier::new(Attribute::VELOCITY, velocity))
    .init(SetAttributeModifier::new(Attribute::AGE, age))
    .init(SetAttributeModifier::new(Attribute::LIFETIME, lifetime))
    .update(LinearDragModifier::new(drag))
    .update(accel)
    .render(OrientModifier::new(if tower {
        OrientMode::AlongVelocity
    } else {
        OrientMode::ParallelCameraDepthPlane
    }))
    .render(ColorOverLifetimeModifier::new(color))
    .render(SizeOverLifetimeModifier {
        gradient: size,
        screen_space_size: false,
    })
    // Hanabi 0.19 ignores the screen_space_size field; apply pixel scaling explicitly.
    .render(ScreenSpaceSizeModifier)
}

#[derive(Component)]
struct HitFx(Timer);

fn spawn_hit_fx_system(
    mut cmds: Commands,
    mut hits: MessageReader<CollisionWithBallEvent>,
    assets: Res<HitFxAssets>,
    balls: Query<&Transform, With<PinBall>>,
    targets: Query<(&Transform, Has<Enemy>), Or<(With<Enemy>, With<Tower>, With<TowerFoundation>)>>,
) {
    for CollisionWithBallEvent(ball_id, target_id) in hits.read() {
        let (Ok(ball), Ok((target, is_enemy))) = (balls.get(*ball_id), targets.get(*target_id))
        else {
            continue;
        };
        let outward = (ball.translation - target.translation)
            .xy()
            .try_normalize()
            .unwrap_or(Vec2::X);
        let contact =
            ball.translation - outward.extend(0.) * BALL_RADIUS + Vec3::Z * (BALL_RADIUS + 0.025);
        let (name, effect) = if is_enemy {
            ("Enemy Hit", assets.enemy.clone())
        } else {
            ("Tower Hit", assets.tower.clone())
        };
        cmds.spawn((
            Name::new(name),
            ParticleEffect::new(effect),
            Transform::from_translation(contact),
            EffectProperties::default()
                .with_properties([("hit_angle".to_owned(), outward.y.atan2(outward.x).into())]),
            HitFx(Timer::from_seconds(
                PARTICLE_LIFETIME + 0.05,
                TimerMode::Once,
            )),
        ));
    }
}

fn despawn_hit_fx_system(
    mut cmds: Commands,
    time: Res<Time<EffectSimulation>>,
    mut effects: Query<(Entity, &EffectSpawner, &mut HitFx)>,
) {
    for (entity, spawner, mut effect) in &mut effects {
        if spawner.has_completed() {
            effect.0.tick(time.delta());
            if effect.0.is_finished() {
                cmds.entity(entity).try_despawn();
            }
        }
    }
}

fn clear_hit_fx_system(mut cmds: Commands, effects: Query<Entity, With<HitFx>>) {
    for entity in &effects {
        cmds.entity(entity).try_despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::super::enemy::EnemyKind;
    use super::*;
    use bevy_hanabi::{EffectShaderSources, ScalarValue, Value};
    use moonshine_save::prelude::Save;

    #[test]
    fn hit_assets_generate_shaders() {
        for tower in [false, true] {
            assert!(EffectShaderSources::generate(&hit_asset(tower), None, 0).is_ok());
        }
    }

    #[test]
    fn hit_fan_faces_the_ball_side_of_the_target() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Assets<EffectAsset>>()
            .init_resource::<HitFxAssets>()
            .add_message::<CollisionWithBallEvent>()
            .add_systems(Update, spawn_hit_fx_system);
        let target_position = Vec3::new(0.7, 0.9, 0.);
        let target = app
            .world_mut()
            .spawn((
                Tower::new(target_position),
                Transform::from_translation(target_position),
            ))
            .id();
        for (side, expected_angle) in [
            (Vec2::X, 0.),
            (Vec2::NEG_X, std::f32::consts::PI),
            (Vec2::Y, std::f32::consts::FRAC_PI_2),
            (Vec2::NEG_Y, -std::f32::consts::FRAC_PI_2),
            (Vec2::ONE.normalize(), std::f32::consts::FRAC_PI_4),
            (Vec2::ZERO, 0.),
        ] {
            let ball = app
                .world_mut()
                .spawn((
                    PinBall,
                    Transform::from_translation(target_position + (side * 0.08).extend(-0.02)),
                ))
                .id();
            app.world_mut()
                .write_message(CollisionWithBallEvent(ball, target));
            app.update();
            let effects: Vec<_> = app
                .world_mut()
                .query_filtered::<(Entity, &EffectProperties), With<HitFx>>()
                .iter(app.world())
                .map(|(entity, properties)| (entity, properties.get_stored("hit_angle")))
                .collect();
            assert_eq!(effects.len(), 1);
            for (entity, angle) in effects {
                assert!(matches!(
                    angle,
                    Some(Value::Scalar(ScalarValue::Float(angle)))
                        if (angle - expected_angle).abs() < 0.00001
                ));
                app.world_mut().despawn(entity);
            }
            app.world_mut().despawn(ball);
        }
    }

    #[test]
    fn cleanup_waits_for_emission_and_particle_lifetime() {
        let mut app = App::new();
        app.init_resource::<Time<EffectSimulation>>()
            .add_systems(Update, despawn_hit_fx_system);
        let settings = SpawnerSettings::once(8.0.into());
        let entity = app
            .world_mut()
            .spawn((
                EffectSpawner::new(&settings),
                HitFx(Timer::from_seconds(
                    PARTICLE_LIFETIME + 0.05,
                    TimerMode::Once,
                )),
            ))
            .id();
        app.world_mut()
            .resource_mut::<Time<EffectSimulation>>()
            .advance_by(std::time::Duration::from_secs(2));
        app.update();
        assert_eq!(
            app.world().get::<HitFx>(entity).map(|fx| fx.0.elapsed()),
            Some(std::time::Duration::ZERO)
        );

        let mut rng = rand::SeedableRng::seed_from_u64(1);
        let emitted = app
            .world_mut()
            .get_mut::<EffectSpawner>(entity)
            .map(|mut spawner| (spawner.tick(0.016, &mut rng), spawner.has_completed()));
        assert_eq!(emitted, Some((8, true)));
        app.world_mut()
            .resource_mut::<Time<EffectSimulation>>()
            .advance_by(std::time::Duration::ZERO);
        app.update();
        assert!(app.world().get::<HitFx>(entity).is_some());
        app.world_mut()
            .resource_mut::<Time<EffectSimulation>>()
            .advance_by(std::time::Duration::from_secs_f32(0.3));
        app.update();
        assert!(app.world().get::<HitFx>(entity).is_some());
        app.world_mut()
            .resource_mut::<Time<EffectSimulation>>()
            .advance_by(std::time::Duration::from_secs_f32(0.3));
        app.update();
        assert!(app.world().get::<HitFx>(entity).is_none());
    }

    #[test]
    fn hits_select_effects_ignore_walls_and_survive_target_despawn() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::state::app::StatesPlugin))
            .init_state::<AppState>()
            .init_state::<GameState>()
            .init_resource::<Assets<EffectAsset>>()
            .init_resource::<Time<EffectSimulation>>()
            .add_message::<CollisionWithBallEvent>()
            .add_plugins(HitFxPlugin);
        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::Game);
        app.world_mut()
            .resource_mut::<NextState<GameState>>()
            .set(GameState::Ingame);
        app.update();

        let ball = app
            .world_mut()
            .spawn((PinBall, Transform::from_xyz(0.08, 0., -0.02)))
            .id();
        let tower = app
            .world_mut()
            .spawn((Tower::new(Vec3::ZERO), Transform::default()))
            .id();
        let enemy = app
            .world_mut()
            .spawn((Enemy::new(1, EnemyKind::Normal), Transform::default()))
            .id();
        let wall = app.world_mut().spawn(Transform::default()).id();
        let missing = app.world_mut().spawn_empty().id();
        app.world_mut().despawn(missing);
        for target in [tower, enemy, wall, missing] {
            app.world_mut()
                .write_message(CollisionWithBallEvent(ball, target));
        }
        app.update();

        let effects: Vec<_> = app
            .world_mut()
            .query_filtered::<(&Name, &Transform, Has<Save>, &ParticleEffect), With<HitFx>>()
            .iter(app.world())
            .map(|(name, transform, saved, effect)| {
                (
                    name.as_str().to_owned(),
                    transform.translation,
                    saved,
                    effect.handle.clone(),
                )
            })
            .collect();
        assert_eq!(effects.len(), 2);
        let assets = app.world().resource::<HitFxAssets>();
        assert_ne!(assets.tower, assets.enemy);
        for (name, position, saved, handle) in &effects {
            assert!(name == "Tower Hit" || name == "Enemy Hit");
            assert_eq!(
                handle,
                if name == "Tower Hit" {
                    &assets.tower
                } else {
                    &assets.enemy
                }
            );
            assert!(position.distance(Vec3::new(0.06, 0., 0.025)) < 0.0001);
            assert!(!saved);
        }
        assert!(effects.iter().any(|effect| effect.0 == "Tower Hit"));
        assert!(effects.iter().any(|effect| effect.0 == "Enemy Hit"));

        app.world_mut().despawn(enemy);
        app.world_mut().despawn(tower);
        app.update();
        assert_eq!(
            app.world_mut().query::<&HitFx>().iter(app.world()).count(),
            2
        );

        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::MainMenu);
        app.update();
        assert_eq!(
            app.world_mut().query::<&HitFx>().iter(app.world()).count(),
            0
        );
    }
}
