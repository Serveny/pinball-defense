#![allow(clippy::indexing_slicing, clippy::unwrap_used)]

use super::*;
use std::time::Duration;

fn model(kinds: &[Clip]) -> EnemyModel {
    let mut graph = AnimationGraph::new();
    let clips = kinds
        .iter()
        .map(|&kind| ModelClip {
            kind,
            node: graph.add_clip(Handle::default(), 1.0, graph.root),
            duration: 2.0,
        })
        .collect();
    EnemyModel {
        scene: Handle::default(),
        graph: Handle::default(),
        clips,
        config: ModelConfig {
            clips: Vec::new(),
            scale: 2.0,
            ground_offset: -0.04,
            orientation: Quat::IDENTITY,
            walk_speed: 0.1,
            curve_speed: 0.5,
            turn_speed: 1.0,
        },
    }
}

fn models(model: EnemyModel) -> EnemyModels {
    EnemyModels(HashMap::from([(EnemyKind::Normal, model)]))
}

fn visual(owner: Entity) -> EnemyVisual {
    EnemyVisual {
        owner,
        kind: EnemyKind::Normal,
        previous_position: Vec3::ZERO,
        heading: 0.0,
        phase: 0.0,
        speed: 0.0,
        yaw_speed: 0.0,
    }
}

fn weight(weights: &[(Clip, f32)], kind: Clip) -> f32 {
    weights
        .iter()
        .find(|(clip, _)| *clip == kind)
        .map_or(0.0, |(_, value)| *value)
}

#[test]
fn loaded_enemy_attaches_one_visual_and_preserves_position() {
    let mut app = App::new();
    app.insert_resource(models(model(&[Clip::Walk])))
        .add_systems(Update, attach_models);
    let position = Vec3::new(0.3, 0.4, 0.002);
    let enemy = app
        .world_mut()
        .spawn((
            Enemy::new(1, EnemyKind::Normal),
            Transform::from_translation(position),
        ))
        .id();
    app.update();
    app.update();
    let world = app.world_mut();
    assert!(world.get::<ModelAttached>(enemy).is_some());
    assert_eq!(world.get::<Transform>(enemy).unwrap().translation, position);
    let children = world.get::<Children>(enemy).unwrap();
    assert_eq!(children.len(), 1);
    let child = children[0];
    let attached = world.get::<EnemyVisual>(child).unwrap();
    assert_eq!(attached.owner, enemy);
    assert_eq!(attached.previous_position, position);
    assert!(world.get::<WorldAssetRoot>(child).is_some());
}

#[test]
fn independent_motion_scales_phase_and_pauses_without_advancing() {
    let mut app = App::new();
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(0.5));
    app.insert_resource(time)
        .insert_resource(models(model(&[Clip::Walk])))
        .add_systems(Update, update_motion);
    let slow = app
        .world_mut()
        .spawn((
            Enemy::new(1, EnemyKind::Normal),
            Transform::from_xyz(0.1, 0.0, 0.0),
        ))
        .id();
    let fast = app
        .world_mut()
        .spawn((
            Enemy::new(1, EnemyKind::Normal),
            Transform::from_xyz(0.2, 0.0, 0.0),
        ))
        .id();
    let slow_visual = app
        .world_mut()
        .spawn((visual(slow), Transform::default()))
        .id();
    let fast_visual = app
        .world_mut()
        .spawn((visual(fast), Transform::default()))
        .id();
    app.update();
    assert!((app.world().get::<EnemyVisual>(slow_visual).unwrap().phase - 0.25).abs() < 1e-6);
    assert!((app.world().get::<EnemyVisual>(fast_visual).unwrap().phase - 0.5).abs() < 1e-6);
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(Duration::ZERO);
    app.update();
    assert!((app.world().get::<EnemyVisual>(slow_visual).unwrap().phase - 0.25).abs() < 1e-6);
    assert!((app.world().get::<EnemyVisual>(fast_visual).unwrap().phase - 0.5).abs() < 1e-6);
}

#[test]
fn shared_model_keeps_player_blends_independent() {
    let mut app = App::new();
    let model = model(&[Clip::Walk, Clip::Idle]);
    let walk = model.clips[0].node;
    let idle = model.clips[1].node;
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs(1));
    app.insert_resource(time)
        .insert_resource(models(model))
        .add_systems(Update, animate);
    let owner = app.world_mut().spawn_empty().id();
    let mut walking = visual(owner);
    walking.speed = 0.2;
    walking.phase = 0.25;
    let moving = app.world_mut().spawn(walking).id();
    let stopped = app.world_mut().spawn(visual(owner)).id();
    let mut players = Vec::new();
    for entity in [moving, stopped] {
        let mut player = AnimationPlayer::default();
        player.play(walk).repeat().pause().set_weight(1.0);
        player.play(idle).repeat().pause().set_weight(0.0);
        players.push(
            app.world_mut()
                .spawn((LocomotionPlayer(entity), player))
                .id(),
        );
    }
    app.update();
    let mut moving = app
        .world_mut()
        .get_mut::<AnimationPlayer>(players[0])
        .unwrap();
    assert!(moving.play(walk).weight() > 0.99);
    assert!(moving.play(idle).weight() < 0.01);
    let mut stopped = app
        .world_mut()
        .get_mut::<AnimationPlayer>(players[1])
        .unwrap();
    assert!(stopped.play(walk).weight() < 0.01);
    assert!(stopped.play(idle).weight() > 0.99);
}

#[test]
fn frost_restores_each_material_original_color() {
    let mut app = App::new();
    let mut materials = Assets::<StandardMaterial>::default();
    let colors = [Color::srgb(0.2, 0.1, 0.05), Color::srgb(0.8, 0.02, 0.01)];
    let handles: Vec<_> = colors
        .iter()
        .map(|&color| {
            materials.add(StandardMaterial {
                base_color: color,
                ..default()
            })
        })
        .collect();
    app.insert_resource(materials)
        .insert_resource(ActiveEffects {
            slow_until: 1.0,
            ..default()
        })
        .insert_resource(IngameTime::default())
        .add_systems(Update, tint_models);
    app.world_mut().spawn(ModelMaterials(
        handles.iter().cloned().zip(colors).collect(),
    ));
    app.update();
    for handle in &handles {
        assert_eq!(
            app.world()
                .resource::<Assets<StandardMaterial>>()
                .get(handle)
                .unwrap()
                .base_color,
            FREEZE_TINT
        );
    }
    **app.world_mut().resource_mut::<IngameTime>() = 1.0;
    app.update();
    for (handle, color) in handles.iter().zip(colors) {
        assert_eq!(
            app.world()
                .resource::<Assets<StandardMaterial>>()
                .get(handle)
                .unwrap()
                .base_color,
            color
        );
    }
}

#[test]
fn missing_optional_clips_fall_back_to_walk() {
    let model = model(&[Clip::Walk]);
    for (speed, yaw) in [(0.0, 0.0), (0.0, 2.0), (0.2, 2.0), (0.2, -2.0)] {
        let weights = clip_weights(&model, speed, yaw);
        assert!((weight(&weights, Clip::Walk) - 1.0).abs() < 1e-6);
        assert!((weights.iter().map(|(_, value)| value).sum::<f32>() - 1.0).abs() < 1e-6);
    }
}

#[test]
fn curve_weights_follow_turn_sign_and_relative_speed() {
    let model = model(&[
        Clip::Walk,
        Clip::WalkLeft,
        Clip::WalkRight,
        Clip::Idle,
        Clip::TurnLeft,
        Clip::TurnRight,
    ]);
    let left = clip_weights(&model, 0.2, 0.25);
    let right = clip_weights(&model, 0.2, -0.25);
    let fast = clip_weights(&model, 0.4, 0.25);
    assert!((weight(&left, Clip::WalkLeft) - 0.5).abs() < 1e-6);
    assert!((weight(&right, Clip::WalkRight) - 0.5).abs() < 1e-6);
    assert!((weight(&fast, Clip::WalkLeft) - 0.25).abs() < 1e-6);
    assert!((weight(&clip_weights(&model, 0.0, 0.0), Clip::Idle) - 1.0).abs() < 1e-6);
    assert!((weight(&clip_weights(&model, 0.0, 0.2), Clip::TurnLeft) - 1.0).abs() < 1e-6);
    assert!((weight(&clip_weights(&model, 0.0, -0.2), Clip::TurnRight) - 1.0).abs() < 1e-6);
}

#[test]
fn speed_uses_distance_and_virtual_delta() {
    let (speed, heading, yaw) = motion(Vec3::ZERO, Vec3::X * 0.02, 0.0, 0.1);
    assert!((speed - 0.2).abs() < 1e-6);
    assert!(heading.abs() < 1e-6 && yaw.abs() < 1e-6);
}

#[test]
fn stopped_and_paused_preserve_heading() {
    assert_eq!(motion(Vec3::ZERO, Vec3::ZERO, 1.2, 0.1), (0.0, 1.2, 0.0));
    assert_eq!(motion(Vec3::ZERO, Vec3::X, 1.2, 0.0), (0.0, 1.2, 0.0));
}

#[test]
fn turn_takes_short_route_across_pi() {
    let direction = Vec3::new((-PI + 0.1).cos(), (-PI + 0.1).sin(), 0.0);
    let (_, _, yaw) = motion(Vec3::ZERO, direction, PI - 0.1, 0.1);
    assert!(yaw > 0.0, "short route across pi is a left turn");
    assert!(yaw * 0.1 < 0.2, "turn must not overshoot the target");
}

#[test]
fn turn_eases_over_half_a_second() {
    let target = Vec3::Y;
    let mut heading = 0.0;
    let mut elapsed = 0.0;
    let mut speed_sum = 0.0;
    while (signed_angle(FRAC_PI_2 - heading)).abs() > 0.01 {
        let (speed, next, _) = motion(Vec3::ZERO, target, heading, 1.0 / 60.0);
        assert!(speed > 0.0, "movement must continue while turning");
        speed_sum += speed;
        heading = next;
        elapsed += 1.0 / 60.0;
        assert!(elapsed < 3.0, "turn never settles onto target heading");
    }
    assert!(elapsed > 0.3, "turn snaps instead of easing");
    assert!(speed_sum > 0.0, "position must keep advancing during turn");
}
