use crate::collision_handler::{BuildTowerEvent, LightOnEvent, TowerFoundationCollisionStartEvent};
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::world::PinballWorld;
use crate::GameState;
use bevy_tweening::lens::{TransformPositionLens, TransformRotationLens};
use bevy_tweening::{Animator, Delay, EaseFunction, Tween, Tweenable};
use std::f32::consts::PI;
use std::time::Duration;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTowerEvent>().add_systems(
            Update,
            (
                light_on_contact_system,
                rotate_tower_head_system,
                light_off_system,
                progress_bar_count_up_system,
                progress_bar_scale_system,
                flash_light_system,
                build_tower_system,
                spawn_tower_system,
            )
                .run_if(in_state(GameState::Ingame)),
        );
    }
}

#[derive(Component)]
pub struct TowerBase;

#[derive(Component)]
pub struct ContactLight;

#[derive(Component)]
pub struct TowerHead;

#[derive(Component)]
pub struct MachineGunTowerMount;

#[derive(Component)]
pub struct MachineGunTowerHead;

#[derive(Component)]
pub struct MachineGunTowerBarrel;

#[derive(Component)]
pub struct MicrowaveTower;

#[derive(Component)]
pub struct MachineGunTower;

#[derive(Component)]
pub struct TeslaTower;

#[derive(Component, Clone, Copy)]
pub enum TowerType {
    MachineGun,
    Tesla,
    Microwave,
}

fn tower_material() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::BEIGE,
        perceptual_roughness: 0.6,
        metallic: 0.6,
        reflectance: 0.1,
        ..default()
    }
}

#[derive(Component)]
pub struct TowerFoundation;

#[derive(Component)]
pub struct TowerFoundationLid;

#[derive(Component)]
struct TowerFoundationTop;

#[derive(Component)]
struct TowerFoundationBottom;

#[derive(Component, Default)]
struct TowerFoundationProgressBar(f32);

#[derive(Component)]
struct RelParent(Entity);

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
            Collider::cylinder(0.1, 0.06),
            ColliderDebugColor(Color::RED),
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
struct SelectedTowerFoundation;

fn progress_bar_count_up_system(
    mut cmds: Commands,
    mut evs: EventReader<TowerFoundationCollisionStartEvent>,
    mut q_progress: Query<(&RelParent, &mut TowerFoundationProgressBar)>,
    q_light: Query<(&Parent, Entity), With<ContactLight>>,
    q_selected: Query<&SelectedTowerFoundation>,
) {
    for ev in evs.iter() {
        for (rel_parent, mut progress) in q_progress.iter_mut() {
            let parent_id = rel_parent.0;
            if parent_id == ev.0 {
                if progress.0 < 1. {
                    progress.0 += 1.;
                    if progress.0 >= 1. {
                        add_flash_light(&mut cmds, &q_light, parent_id);
                        set_selected_tower_foundation(&mut cmds, parent_id, &q_selected);
                    }
                }
                break;
            }
        }
    }
}

fn add_flash_light(
    cmds: &mut Commands,
    q_light: &Query<(&Parent, Entity), With<ContactLight>>,
    parent_id: Entity,
) {
    cmds.entity(
        q_light
            .iter()
            .find_map(|(parent, light_id)| if_true!(parent.get() == parent_id, light_id))
            .expect("Parent should have ContactLight as child"),
    )
    .insert(FlashLight);
    println!("open tower menu");
}

fn set_selected_tower_foundation(
    cmds: &mut Commands,
    parent_id: Entity,
    q_selected: &Query<&SelectedTowerFoundation>,
) {
    if q_selected.is_empty() {
        cmds.entity(parent_id).insert(SelectedTowerFoundation);
    }
}

fn progress_bar_scale_system(
    mut q_progress: Query<(&mut Transform, &TowerFoundationProgressBar)>,
    time: Res<Time>,
) {
    for (mut trans, progress) in q_progress.iter_mut() {
        if trans.scale.z < progress.0 {
            trans.scale.z += time.delta_seconds() * 0.5;
        }
    }
}

#[derive(Component)]
struct FlashLight;

fn flash_light_system(mut q_light: Query<&mut PointLight, With<FlashLight>>, time: Res<Time>) {
    for mut light in q_light.iter_mut() {
        light.intensity = ((time.elapsed_seconds() % 8. * 16.).sin() + 1.) * LIGHT_INTENSITY * 0.5;
    }
}
fn spawn_tower_base(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
) {
    parent
        .spawn((
            PbrBundle {
                mesh: assets.tower_base.clone(),
                material: materials.add(tower_material()),
                ..default()
            },
            //Ccd::enabled(),
            RigidBody::KinematicPositionBased,
            ColliderDebugColor(Color::RED),
            Collider::cylinder(0.025, 0.06),
            Restitution::coefficient(1.),
            ActiveEvents::COLLISION_EVENTS,
        ))
        .insert(TowerBase)
        .insert(LightOnCollision)
        .insert(Name::new("Tower Base"))
        .with_children(|parent| {
            parent
                .spawn(PointLightBundle {
                    transform: Transform::from_xyz(0., 0.005, 0.),
                    point_light: PointLight {
                        intensity: 0.,
                        color: Color::RED,
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

fn create_tower_spawn_animator(pos: Vec3) -> Tween<Transform> {
    Tween::new(
        EaseFunction::ExponentialInOut,
        std::time::Duration::from_secs(4),
        TransformPositionLens {
            start: Vec3::new(pos.x, pos.y - 0.1, pos.z),
            end: pos,
        },
    )
    //.with_completed(|entity, delay| {
    //delay.duration().as_secs()
    //}
}

pub fn spawn_tower_microwave(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    parent
        .spawn(SpatialBundle {
            transform: Transform::from_translation(pos),
            ..default()
        })
        .insert(MicrowaveTower)
        .insert(Name::new("Microwave Tower"))
        .insert(Animator::new(create_tower_spawn_animator(pos)))
        .with_children(|parent| {
            spawn_tower_base(parent, materials, assets, g_sett);
            parent
                .spawn(PbrBundle {
                    mesh: assets.tower_microwave_top.clone(),
                    material: materials.add(tower_material()),
                    transform: Transform::from_xyz(0., 0.04, 0.),
                    ..default()
                })
                .insert(TowerHead);
        });
}

pub fn spawn_tower_machine_gun(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    let tower_material = materials.add(tower_material());
    let mg_barrel = |parent: &mut ChildBuilder| {
        parent
            .spawn(PbrBundle {
                mesh: assets.tower_mg_barrel.clone(),
                material: tower_material.clone(),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            })
            .insert(MachineGunTowerBarrel);
    };
    let mg_head = |parent: &mut ChildBuilder| {
        parent
            .spawn(PbrBundle {
                mesh: assets.tower_mg_head.clone(),
                material: tower_material.clone(),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            })
            .insert(MachineGunTowerHead)
            .with_children(|parent| mg_barrel(parent));
    };
    let mg_mounting = |parent: &mut ChildBuilder| {
        parent
            .spawn(PbrBundle {
                mesh: assets.tower_mg_mounting.clone(),
                material: tower_material.clone(),
                transform: Transform::from_xyz(0., 0.023, 0.),
                ..default()
            })
            .insert(MachineGunTowerMount)
            .insert(TowerHead)
            .with_children(|parent| mg_head(parent));
    };
    parent
        .spawn(SpatialBundle {
            transform: Transform::from_translation(pos),
            ..default()
        })
        .insert(MachineGunTower)
        .insert(Name::new("Machine Gun Tower"))
        .insert(Animator::new(create_tower_spawn_animator(pos)))
        .with_children(|parent| {
            spawn_tower_base(parent, materials, assets, g_sett);
            mg_mounting(parent);
        });
}

pub fn spawn_tower_tesla(
    parent: &mut ChildBuilder,
    materials: &mut Assets<StandardMaterial>,
    assets: &PinballDefenseAssets,
    g_sett: &GraphicsSettings,
    pos: Vec3,
) {
    parent
        .spawn(SpatialBundle {
            transform: Transform::from_translation(pos),
            ..default()
        })
        .insert(TeslaTower)
        .insert(Name::new("Tesla Tower"))
        .insert(Animator::new(create_tower_spawn_animator(pos)))
        .with_children(|parent| {
            spawn_tower_base(parent, materials, assets, g_sett);
            parent
                .spawn(PbrBundle {
                    mesh: assets.tower_tesla_top.clone(),
                    material: materials.add(tower_material()),
                    transform: Transform::from_xyz(0., 0.02, 0.),
                    ..default()
                })
                .insert(TowerHead);
        });
}

#[derive(Component)]
pub struct LightOnCollision;

const LIGHT_INTENSITY: f32 = 48.;

fn light_on_contact_system(
    mut evs: EventReader<LightOnEvent>,
    mut q_light: Query<(&mut PointLight, &Parent), With<ContactLight>>,
) {
    for ev in evs.iter() {
        for (mut light, parent) in q_light.iter_mut() {
            if ev.0 == parent.get() {
                light.intensity = LIGHT_INTENSITY;
                break;
            }
        }
    }
}

fn light_off_system(mut q_light: Query<&mut PointLight, With<ContactLight>>, time: Res<Time>) {
    for mut light in q_light.iter_mut() {
        let time = time.delta_seconds() * 64.;
        light.intensity = (light.intensity - time).clamp(0., LIGHT_INTENSITY);
    }
}

fn rotate_tower_head_system(time: Res<Time>, mut q_heads: Query<&mut Transform, With<TowerHead>>) {
    for mut trans in q_heads.iter_mut() {
        trans.rotate(Quat::from_rotation_y(time.delta_seconds()));
    }
}

#[derive(Event)]
struct SpawnTowerEvent(TowerType, Vec3);

fn spawn_tower_system(
    mut cmds: Commands,
    mut evs: EventReader<SpawnTowerEvent>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    assets: Res<PinballDefenseAssets>,
    q_pb_word: Query<Entity, With<PinballWorld>>,
    g_sett: Res<GraphicsSettings>,
) {
    for ev in evs.iter() {
        let pos = ev.1;
        cmds.entity(q_pb_word.single()).with_children(|parent| {
            match ev.0 {
                TowerType::MachineGun => {
                    spawn_tower_machine_gun(parent, &mut mats, &assets, &g_sett, pos)
                }
                TowerType::Tesla => spawn_tower_tesla(parent, &mut mats, &assets, &g_sett, pos),
                TowerType::Microwave => {
                    spawn_tower_microwave(parent, &mut mats, &assets, &g_sett, pos)
                }
            };
        });
    }
}

fn build_tower_system(
    mut evs: EventReader<BuildTowerEvent>,
    mut spawn_tower_ev: EventWriter<SpawnTowerEvent>,
    mut cmds: Commands,
    q_selected: Query<(Entity, &Transform), With<SelectedTowerFoundation>>,
    q_lids_bottom: Query<(Entity, &Parent), With<TowerFoundationBottom>>,
    q_lids_top: Query<(Entity, &Parent), With<TowerFoundationTop>>,
) {
    for ev in evs.iter() {
        if let Ok((selected_id, sel_trans)) = q_selected.get_single() {
            q_lids_bottom.for_each(|(lid_id, lid_parent)| {
                set_rotation_animation(&mut cmds, lid_id, lid_parent.get(), selected_id, -1.);
            });
            q_lids_top.for_each(|(lid_id, lid_parent)| {
                set_rotation_animation(&mut cmds, lid_id, lid_parent.get(), selected_id, 1.);
            });
            let pos = sel_trans.translation;
            spawn_tower_ev.send(SpawnTowerEvent(ev.0, Vec3::new(pos.x, -0.025, pos.z)));
            cmds.entity(selected_id).remove::<SelectedTowerFoundation>();
        }
    }
}

fn set_rotation_animation(
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
