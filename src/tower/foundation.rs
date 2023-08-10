use super::light::{
    add_flash_light, disable_flash_light, spawn_contact_light, ContactLight, FlashLight,
    LightOnCollision,
};
use super::tower_material;
use crate::ball::CollisionWithBallEvent;
use crate::events::collision::collider_only_interact_with_ball;
use crate::events::tween_completed::DESPAWN_ENTITY_EVENT_ID;
use crate::prelude::*;
use crate::progress_bar::{ProgressBarCountUpEvent, ProgressBarFullEvent};
use crate::settings::GraphicsSettings;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotationLens},
    Animator, Delay, EaseFunction, Tween,
};
use std::{f32::consts::PI, time::Duration};

pub type QuerySelected<'w, 's, 'a> =
    Query<'w, 's, (Entity, &'a Transform), With<SelectedTowerFoundation>>;

#[derive(Component)]
pub(super) struct TowerFoundation;

#[derive(Component)]
pub(super) struct TowerFoundationLid;

#[derive(Component)]
pub(super) struct TowerFoundationTop;

#[derive(Component)]
pub(super) struct TowerFoundationBottom;

pub fn spawn_foundation(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    parent
        .spawn((
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
            collider_only_interact_with_ball(),
            ActiveEvents::COLLISION_EVENTS,
            TowerFoundation,
            LightOnCollision,
            Name::new("Tower Foundation"),
        ))
        .with_children(|parent| {
            let rel_id = parent.parent_entity();
            spawn_contact_light(parent, g_sett, Color::GREEN);
            parent.spawn((
                PbrBundle {
                    mesh: assets.tower_foundation_top.clone(),
                    material: materials.add(tower_material()),
                    transform: Transform::from_translation(Vec3::new(-0.06, 0., 0.)),
                    ..default()
                },
                TowerFoundationTop,
                TowerFoundationLid,
                Name::new("Tower Foundation Top"),
            ));
            parent
                .spawn((
                    PbrBundle {
                        mesh: assets.tower_foundation_bottom.clone(),
                        material: materials.add(tower_material()),
                        transform: Transform::from_translation(Vec3::new(0.06, 0., 0.)),
                        ..default()
                    },
                    TowerFoundationBottom,
                    TowerFoundationLid,
                    Name::new("Tower Foundation Bottom"),
                ))
                .with_children(|parent| {
                    crate::progress_bar::spawn(
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
    selected_id: Entity,
    signum: f32,
) {
    if lid_parent_id == selected_id {
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

#[derive(Component)]
pub(super) struct ReadyToBuild;

pub(super) fn set_next_selected_system(
    mut cmds: Commands,
    q_selected: Query<With<SelectedTowerFoundation>>,
    q_ready: Query<Entity, With<ReadyToBuild>>,
    q_light: Query<(&Parent, Entity), With<ContactLight>>,
) {
    if q_selected.is_empty() {
        if let Some(entity_id) = q_ready.iter().next() {
            add_flash_light(&mut cmds, &q_light, entity_id);
            set_selected_tower_foundation(&mut cmds, entity_id);
        }
    }
}

#[derive(Component)]
pub struct SelectedTowerFoundation;

fn set_selected_tower_foundation(cmds: &mut Commands, parent_id: Entity) {
    cmds.entity(parent_id)
        .remove::<ReadyToBuild>()
        .insert(SelectedTowerFoundation);
}

#[derive(Event)]
pub struct DespawnFoundationEvent;

pub(super) fn despawn_system(
    evs: EventReader<DespawnFoundationEvent>,
    mut cmds: Commands,
    mut q_light: Query<(Entity, &Parent, &mut Visibility), With<FlashLight>>,
    q_selected: Query<(Entity, &Transform), With<SelectedTowerFoundation>>,
    q_lids_bottom: Query<(Entity, &Parent), With<TowerFoundationBottom>>,
    q_lids_top: Query<(Entity, &Parent), With<TowerFoundationTop>>,
) {
    if !evs.is_empty() {
        if let Ok((selected_id, sel_trans)) = q_selected.get_single() {
            log!("🥲 Despawn foundation {:?}", selected_id);

            // Open lids
            q_lids_bottom.for_each(|(lid_id, lid_parent)| {
                set_lid_open_animation(&mut cmds, lid_id, lid_parent.get(), selected_id, -1.);
            });
            q_lids_top.for_each(|(lid_id, lid_parent)| {
                set_lid_open_animation(&mut cmds, lid_id, lid_parent.get(), selected_id, 1.);
            });

            // Despawn foundation
            cmds.entity(selected_id)
                .remove::<SelectedTowerFoundation>()
                .remove::<Collider>();
            set_foundation_despawn_animation(&mut cmds, selected_id, sel_trans.translation);

            // Disable selected tower light
            disable_flash_light(&mut cmds, &mut q_light, selected_id);
        }
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

pub(super) fn set_ready_to_build_system(
    mut cmds: Commands,
    mut evr: EventReader<ProgressBarFullEvent>,
    q_foundation: Query<With<TowerFoundation>>,
) {
    for ev in evr.iter() {
        if q_foundation.contains(ev.0) {
            cmds.entity(ev.0).insert(ReadyToBuild);
        }
    }
}
