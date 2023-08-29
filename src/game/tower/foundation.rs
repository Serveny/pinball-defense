use super::light::{contact_light_bundle, disable_flash_light, FlashLight, LightOnCollision};
use super::tower_material;
use crate::game::ball::CollisionWithBallEvent;
use crate::game::cfg::CONFIG;
use crate::game::events::collision::COLLIDE_ONLY_WITH_BALL;
use crate::game::events::tween_completed::DESPAWN_ENTITY_EVENT_ID;
use crate::game::level::PointsEvent;
use crate::game::pinball_menu::{PinballMenuTrigger, TowerMenuExecuteEvent};
use crate::game::progress_bar;
use crate::game::progress_bar::ProgressBarCountUpEvent;
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

pub fn spawn(
    parent: &mut ChildBuilder,
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    parent.spawn(ring(mats, assets, pos)).with_children(|p| {
        let rel_id = p.parent_entity();
        p.spawn(contact_light_bundle(g_sett, Color::GREEN));
        p.spawn(lid_top(mats, assets));
        p.spawn(lid_bottom(mats, assets)).with_children(|p| {
            let bar_trans = Transform::from_translation(Vec3::new(-0.06, 0., 0.));
            progress_bar::spawn(p, assets, mats, rel_id, bar_trans, Color::GREEN, 0.);
        });
    });
}

fn ring(
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
    pos: Vec3,
) -> impl Bundle {
    (
        Name::new("Tower Foundation"),
        PbrBundle {
            mesh: assets.foundation_ring.clone(),
            material: mats.add(StandardMaterial {
                base_color: Color::BLACK,
                perceptual_roughness: 1.,
                metallic: 0.0,
                reflectance: 0.0,
                ..default()
            }),
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
    )
}

fn lid_top(mats: &mut Assets<StandardMaterial>, assets: &PinballDefenseGltfAssets) -> impl Bundle {
    (
        Name::new("Tower Foundation Top"),
        PbrBundle {
            mesh: assets.foundation_lid_top.clone(),
            material: mats.add(tower_material()),
            transform: Transform::from_translation(Vec3::new(-0.06, 0., 0.)),
            ..default()
        },
        TowerFoundationTop,
        TowerFoundationLid,
    )
}

fn lid_bottom(
    mats: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseGltfAssets,
) -> impl Bundle {
    (
        Name::new("Tower Foundation Bottom"),
        PbrBundle {
            mesh: assets.foundation_lid_bottom.clone(),
            material: mats.add(tower_material()),
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

fn set_foundation_despawn_animation(cmds: &mut Commands, foundation_id: Entity, pos: Vec3) {
    let delay = Delay::new(Duration::from_secs(3));
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

pub(super) fn despawn_system(
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
        set_foundation_despawn_animation(&mut cmds, foundation_id, pos);

        // Disable selected tower light
        disable_flash_light(&mut cmds, &mut q_light, foundation_id);
    }
}

pub(super) fn progress_system(
    mut prog_bar_ev: EventWriter<ProgressBarCountUpEvent>,
    mut ball_coll_ev: EventReader<CollisionWithBallEvent>,
    mut points_ev: EventWriter<PointsEvent>,
    q_tower_foundation: Query<Entity, With<TowerFoundation>>,
) {
    for CollisionWithBallEvent(id, flag) in ball_coll_ev.iter() {
        if *flag == CollisionEventFlags::SENSOR && q_tower_foundation.contains(*id) {
            prog_bar_ev.send(ProgressBarCountUpEvent(*id, CONFIG.foundation_hit_progress));
            points_ev.send(PointsEvent::FoundationHit);
        }
    }
}
