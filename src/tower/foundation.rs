use super::{
    add_flash_light, tower_material, ContactLight, FlashLight, LightOnCollision, SpawnTowerEvent,
};
use crate::pinball_menu::{PinballMenu, PinballMenuElement, SpawnPinballMenuEvent};
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::utils::collision_events::BuildTowerEvent;
use crate::utils::RelParent;
use crate::utils::{
    collision_events::TowerFoundationCollisionStartEvent,
    tween_completed_events::DESPAWN_ENTITY_EVENT_ID,
};
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotationLens},
    Animator, Delay, EaseFunction, Tween,
};
use std::{f32::consts::PI, time::Duration};

#[derive(Component)]
pub struct TowerFoundation;

#[derive(Component)]
pub struct TowerFoundationLid;

#[derive(Component)]
pub struct TowerFoundationTop;

#[derive(Component)]
pub struct TowerFoundationBottom;

#[derive(Component, Default)]
pub struct TowerFoundationProgressBar(f32);

pub fn spawn_tower_foundation(
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
            ActiveEvents::COLLISION_EVENTS,
        ))
        .insert(TowerFoundation)
        .insert(LightOnCollision)
        .insert(Name::new("Tower Foundation"))
        .with_children(|parent| {
            let rel_id = parent.parent_entity();
            parent
                .spawn(PbrBundle {
                    mesh: assets.tower_foundation_top.clone(),
                    material: materials.add(tower_material()),
                    transform: Transform::from_translation(Vec3::new(-0.06, 0., 0.)),
                    ..default()
                })
                .insert(TowerFoundationTop)
                .insert(TowerFoundationLid)
                .insert(Name::new("Tower Foundation Top"));
            parent
                .spawn(PbrBundle {
                    mesh: assets.tower_foundation_bottom.clone(),
                    material: materials.add(tower_material()),
                    transform: Transform::from_translation(Vec3::new(0.06, 0., 0.)),
                    ..default()
                })
                .insert(TowerFoundationBottom)
                .insert(TowerFoundationLid)
                .insert(Name::new("Tower Foundation Bottom"))
                .with_children(|parent| {
                    parent
                        .spawn(PbrBundle {
                            mesh: assets.tower_foundation_progress_bar_frame.clone(),
                            material: materials.add(StandardMaterial {
                                base_color: Color::BLACK,
                                perceptual_roughness: 0.4,
                                metallic: 0.6,
                                reflectance: 0.5,
                                ..default()
                            }),
                            transform: Transform::from_translation(Vec3::new(-0.06, 0., 0.)),
                            ..default()
                        })
                        .insert(Name::new("Tower Foundation Progress Bar Frame"))
                        .with_children(|parent| {
                            parent
                                .spawn(PbrBundle {
                                    mesh: assets.tower_foundation_progress_bar.clone(),
                                    material: materials.add(StandardMaterial {
                                        base_color: Color::GREEN,
                                        perceptual_roughness: 0.4,
                                        metallic: 0.6,
                                        reflectance: 0.5,
                                        ..default()
                                    }),
                                    transform: Transform {
                                        translation: Vec3::new(0.003, 0.003, 0.034),
                                        scale: Vec3::new(1., 1., 0.),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(TowerFoundationProgressBar::default())
                                .insert(RelParent(rel_id))
                                .insert(Name::new("Tower Foundation Progress Bar"));
                        });
                });
            parent
                .spawn(PointLightBundle {
                    transform: Transform::from_xyz(0., 0.005, 0.),
                    point_light: PointLight {
                        intensity: 0.,
                        color: Color::GREEN,
                        shadows_enabled: g_sett.is_shadows,
                        radius: 0.01,
                        range: 0.5,
                        ..default()
                    },
                    ..default()
                })
                .insert(ContactLight);
        });
}

#[derive(Component)]
pub struct SelectedTowerFoundation;

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

pub(super) fn progress_bar_count_up_system(
    mut cmds: Commands,
    mut evs: EventReader<TowerFoundationCollisionStartEvent>,
    mut q_progress: Query<(&RelParent, &mut TowerFoundationProgressBar)>,
) {
    for ev in evs.iter() {
        for (rel_parent, mut progress) in q_progress.iter_mut() {
            let parent_id = rel_parent.0;
            if parent_id == ev.0 {
                if progress.0 < 1. {
                    progress.0 += 0.5;
                    if progress.0 >= 1. {
                        cmds.entity(parent_id).insert(ReadyToBuild);
                    }
                }
                break;
            }
        }
    }
}

pub(super) fn set_next_selected_system(
    mut cmds: Commands,
    mut ev_spm: EventWriter<SpawnPinballMenuEvent>,
    q_ready: Query<Entity, With<ReadyToBuild>>,
    q_selected: Query<&SelectedTowerFoundation>,
    q_light: Query<(&Parent, Entity), With<ContactLight>>,
) {
    if q_selected.is_empty() {
        if let Some(entity_id) = q_ready.iter().next() {
            add_flash_light(&mut cmds, &q_light, entity_id);
            set_selected_tower_foundation(&mut cmds, entity_id, &q_selected);
            ev_spm.send(SpawnPinballMenuEvent);
        }
    }
}

fn set_selected_tower_foundation(
    cmds: &mut Commands,
    parent_id: Entity,
    q_selected: &Query<&SelectedTowerFoundation>,
) {
    if q_selected.is_empty() {
        cmds.entity(parent_id)
            .insert(SelectedTowerFoundation)
            .remove::<ReadyToBuild>();
    }
}

pub(super) fn progress_bar_scale_system(
    mut q_progress: Query<(&mut Transform, &TowerFoundationProgressBar)>,
    time: Res<Time>,
) {
    for (mut trans, progress) in q_progress.iter_mut() {
        if trans.scale.z < progress.0 {
            trans.scale.z += time.delta_seconds() * 0.5;
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn build_tower_system(
    mut evs: EventReader<BuildTowerEvent>,
    mut spawn_tower_ev: EventWriter<SpawnTowerEvent>,
    mut cmds: Commands,
    q_selected: Query<(Entity, &Transform), With<SelectedTowerFoundation>>,
    q_lids_bottom: Query<(Entity, &Parent), With<TowerFoundationBottom>>,
    q_lids_top: Query<(Entity, &Parent), With<TowerFoundationTop>>,
    q_pbm: Query<Entity, With<PinballMenu>>,
    q_pbme: Query<(Entity, &Transform), With<PinballMenuElement>>,
    mut q_light: Query<(Entity, &Parent, &mut PointLight), With<ContactLight>>,
) {
    for ev in evs.iter() {
        if let Ok((selected_id, sel_trans)) = q_selected.get_single() {
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

            // Spawn new tower
            let pos = sel_trans.translation;
            spawn_tower_ev.send(SpawnTowerEvent(ev.0, Vec3::new(pos.x, -0.025, pos.z)));

            // Despawn menu
            q_pbme.for_each(|(entity, trans)| {
                cmds.entity(entity)
                    .insert(Animator::new(crate::pinball_menu::despawn_animation(
                        trans.rotation.y,
                    )));
            });
            disable_light(&mut cmds, &mut q_light, selected_id);
            let delay: Delay<Transform> =
                Delay::new(Duration::from_secs(2)).with_completed_event(DESPAWN_ENTITY_EVENT_ID);
            cmds.entity(q_pbm.single()).insert(Animator::new(delay));
        }
    }
}

fn disable_light(
    cmds: &mut Commands,
    q_light: &mut Query<(Entity, &Parent, &mut PointLight), With<ContactLight>>,
    parent_id: Entity,
) {
    let (entity, _, mut pl) = q_light
        .iter_mut()
        .find(|(_, p, _)| p.get() == parent_id)
        .expect("Here should be the selected parend 🫢");
    pl.intensity = 0.;
    cmds.entity(entity).remove::<FlashLight>();
}
