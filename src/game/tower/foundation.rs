use super::light::{disable_flash_light, spawn_contact_light, FlashLight, LightOnCollision};
use super::tower_material;
use crate::game::ball::CollisionWithBallEvent;
use crate::game::events::collision::COLLIDE_ONLY_WITH_BALL;
use crate::game::events::tween_completed::DESPAWN_ENTITY_EVENT_ID;
use crate::game::pinball_menu::{PinballMenuTrigger, TowerMenuExecuteEvent};
use crate::game::progress_bar;
use crate::game::progress_bar::ProgressBarCountUpEvent;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;
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
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    parent
        .spawn((
            Name::new("Tower Foundation"),
            PbrBundle {
                mesh: assets.tower_foundation_ring.clone(),
                material: materials.add(StandardMaterial {
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
            Collider::cylinder(0.1, 0.07),
            ColliderDebugColor(Color::GREEN),
            COLLIDE_ONLY_WITH_BALL,
            ActiveEvents::COLLISION_EVENTS,
            TowerFoundation,
            LightOnCollision,
            PinballMenuTrigger::Tower,
        ))
        .with_children(|parent| {
            let rel_id = parent.parent_entity();
            spawn_contact_light(parent, g_sett, Color::GREEN);
            parent.spawn((
                Name::new("Tower Foundation Top"),
                PbrBundle {
                    mesh: assets.tower_foundation_top.clone(),
                    material: materials.add(tower_material()),
                    transform: Transform::from_translation(Vec3::new(-0.06, 0., 0.)),
                    ..default()
                },
                TowerFoundationTop,
                TowerFoundationLid,
            ));
            parent
                .spawn((
                    Name::new("Tower Foundation Bottom"),
                    PbrBundle {
                        mesh: assets.tower_foundation_bottom.clone(),
                        material: materials.add(tower_material()),
                        transform: Transform::from_translation(Vec3::new(0.06, 0., 0.)),
                        ..default()
                    },
                    TowerFoundationBottom,
                    TowerFoundationLid,
                ))
                .with_children(|parent| {
                    progress_bar::spawn(
                        parent,
                        assets,
                        materials,
                        rel_id,
                        Transform::from_translation(Vec3::new(-0.06, 0., 0.)),
                        Color::GREEN,
                        0.,
                    );
                });
        });
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
                start: Quat::from_rotation_z(0.),
                end: Quat::from_rotation_z(signum * PI / 2.),
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
            end: Vec3::new(pos.x, pos.y - 0.1, pos.z),
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
    q_tower_foundation: Query<Entity, With<TowerFoundation>>,
) {
    for CollisionWithBallEvent(id, flag) in ball_coll_ev.iter() {
        if *flag == CollisionEventFlags::SENSOR && q_tower_foundation.contains(*id) {
            prog_bar_ev.send(ProgressBarCountUpEvent(*id, 1.));
        }
    }
}
