use crate::collision_handler::{LightOnEvent, TowerFoundationCollisionStartEvent};
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::GameState;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                light_on_contact_system,
                rotate_tower_head_system,
                light_off_system,
                progress_bar_count_up_system,
                progress_bar_scale_system,
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

fn tower_material() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::BEIGE,
        perceptual_roughness: 0.4,
        metallic: 0.6,
        reflectance: 0.5,
        ..default()
    }
}

#[derive(Component)]
pub struct TowerFoundation;

#[derive(Component)]
struct TowerFoundationTop;

#[derive(Component)]
struct TowerFoundationBottom;

#[derive(Component, Default)]
struct TowerFoundationProgressBar(f32);

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
                material: materials.add(tower_material()),
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
            parent
                .spawn(PbrBundle {
                    mesh: assets.tower_foundation_top.clone(),
                    material: materials.add(tower_material()),
                    transform: Transform::from_translation(Vec3::new(-0.06, 0., 0.)),
                    ..default()
                })
                .insert(TowerFoundationTop)
                .insert(Name::new("Tower Foundation Top"));
            parent
                .spawn(PbrBundle {
                    mesh: assets.tower_foundation_bottom.clone(),
                    material: materials.add(tower_material()),
                    transform: Transform::from_translation(Vec3::new(0.06, 0., 0.)),
                    ..default()
                })
                .insert(TowerFoundationBottom)
                .insert(Name::new("Tower Foundation Bottom"));
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
                    transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                    ..default()
                })
                .insert(Name::new("Tower Foundation Progress Bar Frame"));

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
                .insert(Name::new("Tower Foundation Progress Bar"));
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

fn progress_bar_count_up_system(
    mut evs: EventReader<TowerFoundationCollisionStartEvent>,
    mut q_progress: Query<(&Parent, &mut TowerFoundationProgressBar)>,
) {
    for ev in evs.iter() {
        for (parent, mut progress) in q_progress.iter_mut() {
            if parent.get() == ev.0 {
                if progress.0 < 1. {
                    progress.0 += 0.1;
                    if progress.0 >= 1. {
                        // TODO
                        println!("open tower menu");
                    }
                }
                break;
            }
        }
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
        .insert(MicrowaveTower)
        .insert(Name::new("Tesla Tower"))
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
