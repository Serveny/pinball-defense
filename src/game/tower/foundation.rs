use crate::game::ball::CollisionWithBallEvent;
use crate::game::cfg::CONFIG;
use crate::game::events::collision::COLLIDE_ONLY_WITH_BALL;
use crate::game::events::tween_completed::DESPAWN_ENTITY_EVENT_ID;
use crate::game::level::{LevelUpEvent, PointsEvent};
use crate::game::light::{contact_light_bundle, disable_flash_light, FlashLight, LightOnCollision};
use crate::game::pinball_menu::{PinballMenuTrigger, TowerMenuExecuteEvent};
use crate::game::progress_bar;
use crate::game::progress_bar::ProgressBarCountUpEvent;
use crate::game::world::PinballWorld;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotationLens},
    Animator, Delay, EaseFunction, Tween,
};
use std::{f32::consts::PI, time::Duration};

#[derive(Component)]
pub(super) struct TowerFoundation;

#[derive(Component)]
pub(super) struct TowerFoundationLid;

#[derive(Component)]
pub(super) struct TowerFoundationTop;

#[derive(Component)]
pub(super) struct TowerFoundationBottom;

pub(super) fn on_spawn_system(
    mut cmds: Commands,
    mut on_level_up: EventReader<LevelUpEvent>,
    mut q_mark: Query<(Entity, &mut FoundationBuildMark, &Transform)>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    assets: Res<PinballDefenseGltfAssets>,
    q_pb_word: Query<Entity, With<PinballWorld>>,
    g_sett: Res<GraphicsSettings>,
) {
    for _ in on_level_up.iter() {
        if let Some((mark_id, mut mark, trans)) = q_mark.iter_mut().find(|mark| mark.1.is_available)
        {
            let pos = trans.translation;

            // Despawn mark
            mark.is_available = false;
            set_despawn_animation(&mut cmds, mark_id, pos, 1.);

            // Spawn foundation
            cmds.entity(q_pb_word.single()).with_children(|p| {
                spawn(p, &mut mats, &assets, &g_sett, pos);
            });
        }
    }
}

fn spawn(
    parent: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    parent.spawn(ring(assets, pos)).with_children(|p| {
        let rel_id = p.parent_entity();
        p.spawn(contact_light_bundle(g_sett, Color::GREEN));
        p.spawn(lid_top(assets));
        p.spawn(lid_bottom(assets)).with_children(|p| {
            let bar_trans = Transform::from_translation(Vec3::new(-0.06, 0., 0.));
            progress_bar::spawn(p, assets, mats, rel_id, bar_trans, Color::GREEN, 0.);
        });
    });
}

fn ring(assets: &PinballDefenseGltfAssets, pos: Vec3) -> impl Bundle {
    (
        Name::new("Tower Foundation"),
        PbrBundle {
            mesh: assets.foundation_ring.clone(),
            material: assets.foundation_ring_material.clone(),
            transform: Transform::from_translation(pos),
            ..default()
        },
        Sensor,
        Collider::ball(0.07),
        ColliderDebugColor(Color::GREEN),
        COLLIDE_ONLY_WITH_BALL,
        ActiveEvents::COLLISION_EVENTS,
        TowerFoundation,
        LightOnCollision,
        PinballMenuTrigger::Tower,
        Animator::new(spawn_animation(pos)),
    )
}

fn spawn_animation(pos: Vec3) -> Tween<Transform> {
    Tween::new(
        EaseFunction::QuadraticIn,
        std::time::Duration::from_secs(2),
        TransformPositionLens {
            start: Vec3::new(pos.x, pos.y, pos.z - 0.02),
            end: pos,
        },
    )
}

fn lid_top(assets: &PinballDefenseGltfAssets) -> impl Bundle {
    (
        Name::new("Tower Foundation Top"),
        PbrBundle {
            mesh: assets.foundation_lid_top.clone(),
            material: assets.foundation_lid_material.clone(),
            transform: Transform::from_translation(Vec3::new(-0.06, 0., 0.)),
            ..default()
        },
        TowerFoundationTop,
        TowerFoundationLid,
    )
}

fn lid_bottom(assets: &PinballDefenseGltfAssets) -> impl Bundle {
    (
        Name::new("Tower Foundation Bottom"),
        PbrBundle {
            mesh: assets.foundation_lid_bottom.clone(),
            material: assets.foundation_lid_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.06, 0., 0.)),
            ..default()
        },
        TowerFoundationBottom,
        TowerFoundationLid,
    )
}

fn set_lid_open_animation(
    cmds: &mut Commands,
    lid_id: Entity,
    lid_parent_id: Entity,
    foundation_id: Entity,
    signum: f32,
) {
    if lid_parent_id == foundation_id {
        let tween = Tween::new(
            EaseFunction::QuadraticIn,
            std::time::Duration::from_secs(2),
            TransformRotationLens {
                start: Quat::from_rotation_y(0.),
                end: Quat::from_rotation_y(-signum * PI / 2.),
            },
        );
        cmds.entity(lid_id).insert(Animator::new(tween));
    }
}

fn set_despawn_animation(cmds: &mut Commands, foundation_id: Entity, pos: Vec3, time_secs: f32) {
    let delay = Delay::new(Duration::from_secs_f32(time_secs));
    let tween = Tween::new(
        EaseFunction::QuadraticIn,
        std::time::Duration::from_secs(2),
        TransformPositionLens {
            start: pos,
            end: Vec3::new(pos.x, pos.y, pos.z - 0.1),
        },
    )
    .with_completed_event(DESPAWN_ENTITY_EVENT_ID);

    let sequence = delay.then(tween);
    cmds.entity(foundation_id).insert(Animator::new(sequence));
}

pub(super) fn on_despawn_system(
    mut cmds: Commands,
    mut tower_menu_execute_ev: EventReader<TowerMenuExecuteEvent>,
    mut q_light: Query<(Entity, &Parent, &mut Visibility), With<FlashLight>>,
    q_foundation: Query<&Transform, With<TowerFoundation>>,
    q_lids_bottom: Query<(Entity, &Parent), With<TowerFoundationBottom>>,
    q_lids_top: Query<(Entity, &Parent), With<TowerFoundationTop>>,
) {
    for ev in tower_menu_execute_ev.iter() {
        let foundation_id = ev.foundation_id;
        let pos = q_foundation
            .get(foundation_id)
            .expect("😥 Here should be a foundation.")
            .translation;

        // Open lids
        q_lids_bottom.for_each(|(lid_id, lid_parent)| {
            set_lid_open_animation(&mut cmds, lid_id, lid_parent.get(), foundation_id, -1.);
        });
        q_lids_top.for_each(|(lid_id, lid_parent)| {
            set_lid_open_animation(&mut cmds, lid_id, lid_parent.get(), foundation_id, 1.);
        });

        // Despawn foundation
        log!("🥲 Despawn foundation {:?}", foundation_id);
        cmds.entity(foundation_id).remove::<Collider>();
        set_despawn_animation(&mut cmds, foundation_id, pos, 3.);

        // Disable selected tower light
        disable_flash_light(&mut cmds, &mut q_light, foundation_id);
    }
}

pub(super) fn on_progress_system(
    mut prog_bar_ev: EventWriter<ProgressBarCountUpEvent>,
    mut ball_coll_ev: EventReader<CollisionWithBallEvent>,
    mut points_ev: EventWriter<PointsEvent>,
    q_tower_foundation: Query<Entity, With<TowerFoundation>>,
) {
    for CollisionWithBallEvent(id, flag) in ball_coll_ev.iter() {
        if *flag == CollisionEventFlags::SENSOR && q_tower_foundation.contains(*id) {
            prog_bar_ev.send(ProgressBarCountUpEvent::new(
                *id,
                CONFIG.foundation_hit_progress,
            ));
            points_ev.send(PointsEvent::FoundationHit);
        }
    }
}

#[derive(Component)]
pub struct FoundationBuildMark {
    i: usize,
    is_available: bool,
}

impl FoundationBuildMark {
    pub fn new(i: usize) -> Self {
        Self {
            i,
            is_available: true,
        }
    }
}

impl std::fmt::Display for FoundationBuildMark {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Build Mark {} - available: {}",
            self.i, self.is_available
        )
    }
}

pub fn build_mark(assets: &PinballDefenseGltfAssets, pos: Vec3, i: usize) -> impl Bundle {
    (
        Name::new(format!("Build Mark {i}")),
        FoundationBuildMark::new(i),
        PbrBundle {
            mesh: assets.build_mark.clone(),
            material: assets.build_mark_material.clone(),
            transform: Transform::from_translation(pos),
            ..default()
        },
    )
}
