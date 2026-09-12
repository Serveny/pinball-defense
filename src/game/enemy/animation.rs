use super::{Enemy, EnemyKind, FREEZE_TINT, walk::walk_system};
use crate::game::{
    GameState, IngameTime,
    extra::{ActiveEffects, ExtraFieldKind},
};
use crate::prelude::*;
use bevy::animation::graph::AnimationNodeIndex;
use bevy::gltf::Gltf;
use bevy::world_serialization::WorldInstanceReady;
use std::collections::HashMap;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

pub(super) struct EnemyAnimationPlugin;

impl Plugin for EnemyAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyModels>()
            .add_systems(
                Update,
                prepare_models.run_if(resource_exists::<PinballDefenseAssets>),
            )
            .add_systems(
                Update,
                (attach_models, update_motion, animate, tint_models)
                    .chain()
                    .after(walk_system)
                    .after(prepare_models)
                    .run_if(in_state(GameState::Ingame)),
            );
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Clip {
    Idle,
    Walk,
    WalkLeft,
    WalkRight,
    TurnLeft,
    TurnRight,
}

struct ModelConfig {
    clips: Vec<(Clip, &'static str)>,
    scale: f32,
    ground_offset: f32,
    orientation: Quat,
    walk_speed: f32,
    curve_speed: f32,
    turn_speed: f32,
}

struct ModelClip {
    kind: Clip,
    node: AnimationNodeIndex,
    duration: f32,
}

struct EnemyModel {
    scene: Handle<WorldAsset>,
    graph: Handle<AnimationGraph>,
    clips: Vec<ModelClip>,
    config: ModelConfig,
}

#[derive(Resource, Default)]
struct EnemyModels(HashMap<EnemyKind, EnemyModel>);

fn prepare_models(
    assets: Res<PinballDefenseAssets>,
    gltfs: Res<Assets<Gltf>>,
    clips: Res<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut models: ResMut<EnemyModels>,
    mut prepared: Local<bool>,
) {
    if *prepared {
        return;
    }
    let normal_ready = prepare_normal_model(&assets, &gltfs, &clips, &mut graphs, &mut models);
    let tank_ready = prepare_tank_model(&assets, &gltfs, &clips, &mut graphs, &mut models);
    let speeder_ready = prepare_speeder_model(&assets, &gltfs, &clips, &mut graphs, &mut models);
    *prepared = normal_ready && tank_ready && speeder_ready;
}

fn prepare_normal_model(
    assets: &PinballDefenseAssets,
    gltfs: &Assets<Gltf>,
    clips: &Assets<AnimationClip>,
    graphs: &mut Assets<AnimationGraph>,
    models: &mut EnemyModels,
) -> bool {
    let Some(gltf) = gltfs.get(&assets.normal_enemy) else {
        return false;
    };
    if gltf.animations.iter().any(|h| !clips.contains(h)) {
        return false;
    }
    let config = ModelConfig {
        clips: vec![
            (Clip::Idle, "IC_Idle"),
            (Clip::Walk, "IC_Walk"),
            (Clip::WalkLeft, "IC_Walk_Left"),
            (Clip::WalkRight, "IC_Walk_Right"),
            (Clip::TurnLeft, "IC_Turn_Left"),
            (Clip::TurnRight, "IC_Turn_Right"),
        ],
        scale: 1.2,
        ground_offset: -0.044,
        orientation: Quat::from_rotation_z(FRAC_PI_2) * Quat::from_rotation_x(FRAC_PI_2),
        walk_speed: 0.036,
        curve_speed: 0.5,
        turn_speed: 1.0,
    };
    match build_model(gltf, clips, graphs, config) {
        Ok(model) => {
            models.0.insert(EnemyKind::Normal, model);
            true
        }
        Err(reason) => {
            error!("Cannot prepare normal enemy animations: {reason}");
            false
        }
    }
}

fn prepare_tank_model(
    assets: &PinballDefenseAssets,
    gltfs: &Assets<Gltf>,
    clips: &Assets<AnimationClip>,
    graphs: &mut Assets<AnimationGraph>,
    models: &mut EnemyModels,
) -> bool {
    let Some(gltf) = gltfs.get(&assets.tank_enemy) else {
        return false;
    };
    if gltf.animations.iter().any(|h| !clips.contains(h)) {
        return false;
    }
    let config = ModelConfig {
        clips: vec![(Clip::Idle, "BG_Idle"), (Clip::Walk, "BG_Walk")],
        scale: 1.8,
        ground_offset: -0.044,
        orientation: Quat::from_rotation_z(FRAC_PI_2) * Quat::from_rotation_x(FRAC_PI_2),
        walk_speed: 0.0056,
        curve_speed: 0.5,
        turn_speed: 1.0,
    };
    match build_model(gltf, clips, graphs, config) {
        Ok(model) => {
            models.0.insert(EnemyKind::Tank, model);
            true
        }
        Err(reason) => {
            error!("Cannot prepare tank enemy animations: {reason}");
            false
        }
    }
}

fn prepare_speeder_model(
    assets: &PinballDefenseAssets,
    gltfs: &Assets<Gltf>,
    clips: &Assets<AnimationClip>,
    graphs: &mut Assets<AnimationGraph>,
    models: &mut EnemyModels,
) -> bool {
    let Some(gltf) = gltfs.get(&assets.speeder_enemy) else {
        return false;
    };
    if gltf.animations.iter().any(|h| !clips.contains(h)) {
        return false;
    }
    let config = ModelConfig {
        clips: vec![(Clip::Walk, "SP_Roll")],
        scale: 1.4,
        ground_offset: -0.044,
        orientation: Quat::from_rotation_z(PI),
        walk_speed: 0.026_39,
        curve_speed: 0.5,
        turn_speed: 1.0,
    };
    match build_model(gltf, clips, graphs, config) {
        Ok(model) => {
            models.0.insert(EnemyKind::Speeder, model);
            true
        }
        Err(reason) => {
            error!("Cannot prepare speeder enemy animations: {reason}");
            false
        }
    }
}

fn build_model(
    gltf: &Gltf,
    clips: &Assets<AnimationClip>,
    graphs: &mut Assets<AnimationGraph>,
    config: ModelConfig,
) -> Result<EnemyModel, String> {
    let scene = gltf
        .default_scene
        .as_ref()
        .or_else(|| gltf.scenes.first())
        .ok_or("model has no scene")?
        .clone();
    let mut graph = AnimationGraph::new();
    let mut nodes = Vec::new();
    for &(kind, name) in &config.clips {
        let handle = gltf
            .named_animations
            .get(name)
            .ok_or_else(|| format!("missing clip {name}"))?;
        let duration = clips
            .get(handle)
            .ok_or_else(|| format!("clip {name} not loaded"))?
            .duration();
        if duration <= 0.0 {
            return Err(format!("clip {name} has no duration"));
        }
        nodes.push(ModelClip {
            kind,
            node: graph.add_clip(handle.clone(), 1.0, graph.root),
            duration,
        });
    }
    if !nodes.iter().any(|c| c.kind == Clip::Walk)
        || config.walk_speed <= 0.0
        || config.scale <= 0.0
    {
        return Err("a walk clip and positive reference speed/scale are required".into());
    }
    Ok(EnemyModel {
        scene,
        graph: graphs.add(graph),
        clips: nodes,
        config,
    })
}

#[derive(Component)]
struct ModelAttached;

#[derive(Component)]
struct EnemyVisual {
    owner: Entity,
    kind: EnemyKind,
    previous_position: Vec3,
    heading: f32,
    phase: f32,
    speed: f32,
    yaw_speed: f32,
}

#[derive(Component)]
struct LocomotionPlayer(Entity);

#[derive(Component)]
struct ModelMaterials(Vec<(Handle<StandardMaterial>, Color)>);

fn attach_models(
    mut cmds: Commands,
    models: Res<EnemyModels>,
    enemies: Query<(Entity, &Enemy, &Transform), Without<ModelAttached>>,
) {
    for (owner, enemy, transform) in &enemies {
        let Some(model) = models.0.get(&enemy.kind()) else {
            continue;
        };
        let direction = enemy.step.direction;
        let heading = direction.y.atan2(direction.x);
        cmds.entity(owner).insert(ModelAttached).with_children(|p| {
            p.spawn((
                Name::new("Enemy animated visual"),
                WorldAssetRoot(model.scene.clone()),
                Transform::from_xyz(0.0, 0.0, model.config.ground_offset)
                    .with_scale(Vec3::splat(model.config.scale))
                    .with_rotation(Quat::from_rotation_z(heading) * model.config.orientation),
                EnemyVisual {
                    owner,
                    kind: enemy.kind(),
                    previous_position: transform.translation,
                    heading,
                    phase: 0.0,
                    speed: 0.0,
                    yaw_speed: 0.0,
                },
            ))
            .observe(on_model_ready);
        });
    }
}

fn on_model_ready(
    ready: On<WorldInstanceReady>,
    mut cmds: Commands,
    visuals: Query<&EnemyVisual>,
    models: Res<EnemyModels>,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
) {
    let Ok(visual) = visuals.get(ready.entity) else {
        return;
    };
    let Some(model) = models.0.get(&visual.kind) else {
        return;
    };
    let mut copies = HashMap::new();
    let mut originals = Vec::new();
    for entity in children.iter_descendants(ready.entity) {
        if let Ok(mut player) = players.get_mut(entity) {
            for clip in &model.clips {
                player
                    .play(clip.node)
                    .repeat()
                    .pause()
                    .set_weight(if clip.kind == Clip::Walk { 1.0 } else { 0.0 });
            }
            cmds.entity(entity).insert((
                AnimationGraphHandle(model.graph.clone()),
                LocomotionPlayer(ready.entity),
            ));
        }
        if let Ok(handle) = mesh_materials.get(entity) {
            let copy = copies.entry(handle.0.id()).or_insert_with(|| {
                materials.get(&handle.0).cloned().map(|material| {
                    let color = material.base_color;
                    let copy = materials.add(material);
                    originals.push((copy.clone(), color));
                    copy
                })
            });
            if let Some(copy) = copy {
                cmds.entity(entity).insert(MeshMaterial3d(copy.clone()));
            }
        }
    }
    cmds.entity(ready.entity).insert(ModelMaterials(originals));
    cmds.entity(visual.owner)
        .remove::<(Mesh3d, MeshMaterial3d<StandardMaterial>)>();
}

fn signed_angle(angle: f32) -> f32 {
    (angle + PI).rem_euclid(TAU) - PI
}

fn motion(previous: Vec3, current: Vec3, heading: f32, dt: f32) -> (f32, f32, f32) {
    if dt <= 0.0 {
        return (0.0, heading, 0.0);
    }
    let delta = (current - previous).truncate();
    if delta.length_squared() < 1e-12 {
        return (0.0, heading, 0.0);
    }
    let turn = signed_angle(delta.y.atan2(delta.x) - heading).clamp(-8.0 * dt, 8.0 * dt);
    (delta.length() / dt, signed_angle(heading + turn), turn / dt)
}

fn update_motion(
    time: Res<Time>,
    models: Res<EnemyModels>,
    enemies: Query<&Transform, (With<Enemy>, Without<EnemyVisual>)>,
    mut visuals: Query<(&mut EnemyVisual, &mut Transform), Without<Enemy>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }
    for (mut visual, mut transform) in &mut visuals {
        let Ok(actor) = enemies.get(visual.owner) else {
            continue;
        };
        let Some(model) = models.0.get(&visual.kind) else {
            continue;
        };
        let (speed, heading, yaw) = motion(
            visual.previous_position,
            actor.translation,
            visual.heading,
            dt,
        );
        visual.previous_position = actor.translation;
        visual.speed = speed;
        visual.heading = heading;
        visual.yaw_speed = yaw;
        transform.rotation = Quat::from_rotation_z(heading) * model.config.orientation;
        let (kind, rate) = if speed >= 1e-4 {
            (
                Clip::Walk,
                speed / (model.config.walk_speed * model.config.scale),
            )
        } else if yaw.abs() > 0.01 && model.config.turn_speed > 0.0 {
            (
                if yaw > 0.0 {
                    Clip::TurnLeft
                } else {
                    Clip::TurnRight
                },
                yaw.abs() / model.config.turn_speed,
            )
        } else {
            (Clip::Idle, 1.0)
        };
        if let Some(clip) = model.clips.iter().find(|c| c.kind == kind) {
            visual.phase = (visual.phase + dt * rate / clip.duration).fract();
        }
    }
}

fn clip_weights(model: &EnemyModel, speed: f32, yaw: f32) -> Vec<(Clip, f32)> {
    let has = |kind| model.clips.iter().any(|c| c.kind == kind);
    if speed < 1e-4 {
        let turn = if yaw >= 0.0 {
            Clip::TurnLeft
        } else {
            Clip::TurnRight
        };
        return vec![(
            if yaw.abs() > 0.01 && has(turn) {
                turn
            } else if has(Clip::Idle) {
                Clip::Idle
            } else {
                Clip::Walk
            },
            1.0,
        )];
    }
    let rate = speed / (model.config.walk_speed * model.config.scale);
    let curve = if yaw >= 0.0 {
        Clip::WalkLeft
    } else {
        Clip::WalkRight
    };
    let amount = if has(curve) && model.config.curve_speed > 0.0 {
        (yaw.abs() / (model.config.curve_speed * rate)).clamp(0.0, 1.0)
    } else {
        0.0
    };
    vec![(Clip::Walk, 1.0 - amount), (curve, amount)]
}

fn animate(
    time: Res<Time>,
    models: Res<EnemyModels>,
    visuals: Query<&EnemyVisual>,
    mut players: Query<(&LocomotionPlayer, &mut AnimationPlayer)>,
) {
    let blend = 1.0 - (-time.delta_secs() / 0.08).exp();
    for (owner, mut player) in &mut players {
        let Ok(visual) = visuals.get(owner.0) else {
            continue;
        };
        let Some(model) = models.0.get(&visual.kind) else {
            continue;
        };
        let weights = clip_weights(model, visual.speed, visual.yaw_speed);
        for clip in &model.clips {
            let target = weights
                .iter()
                .find(|(kind, _)| *kind == clip.kind)
                .map_or(0.0, |(_, w)| *w);
            let active = player.play(clip.node);
            let weight = active.weight() + (target - active.weight()) * blend;
            active
                .set_weight(weight)
                .seek_to(visual.phase * clip.duration);
        }
    }
}

fn tint_models(
    effects: Res<ActiveEffects>,
    ig_time: Res<IngameTime>,
    originals: Query<&ModelMaterials>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let frozen = effects.is_active(**ig_time, ExtraFieldKind::SlowDown);
    for originals in &originals {
        for (handle, color) in &originals.0 {
            let target = if frozen { FREEZE_TINT } else { *color };
            if materials
                .get(handle)
                .is_some_and(|material| material.base_color != target)
                && let Some(mut material) = materials.get_mut(handle)
            {
                material.base_color = target;
            }
        }
    }
}

#[cfg(test)]
mod tests;
