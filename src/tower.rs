use self::foundation::{
    build_tower_system, progress_bar_count_up_system, progress_bar_scale_system,
    set_next_selected_system,
};
use self::machine_gun::spawn_tower_machine_gun;
use self::microwave::spawn_tower_microwave;
use self::tesla::spawn_tower_tesla;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::utils::collision_events::LightOnEvent;
use crate::world::PinballWorld;
use crate::GameState;
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::{Delay, EaseFunction, Sequence, Tween};
use std::time::Duration;

pub mod foundation;
mod machine_gun;
mod microwave;
mod tesla;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTowerEvent>().add_systems(
            Update,
            (
                light_on_collision_system,
                rotate_tower_head_system,
                light_off_system,
                progress_bar_count_up_system,
                progress_bar_scale_system,
                flash_light_system,
                build_tower_system,
                spawn_tower_system,
                set_next_selected_system,
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
}

#[derive(Component)]
struct FlashLight;

fn flash_light_system(mut q_light: Query<&mut PointLight, With<FlashLight>>, time: Res<Time>) {
    for mut light in q_light.iter_mut() {
        light.intensity = ((time.elapsed_seconds() * 16.).sin() + 1.) * LIGHT_INTENSITY * 0.5;
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
            Collider::cylinder(0.05, 0.06),
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

fn create_tower_spawn_animator(pos: Vec3) -> Sequence<Transform> {
    let delay = Delay::new(Duration::from_secs(1));
    let tween = Tween::new(
        EaseFunction::ExponentialInOut,
        std::time::Duration::from_secs(4),
        TransformPositionLens {
            start: tower_start_pos(pos),
            end: pos,
        },
    );
    delay.then(tween)
}

fn tower_start_pos(pos: Vec3) -> Vec3 {
    Vec3::new(pos.x, pos.y - 0.1, pos.z)
}

#[derive(Component)]
pub struct LightOnCollision;

const LIGHT_INTENSITY: f32 = 48.;

fn light_on_collision_system(
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
        cmds.entity(q_pb_word.single()).with_children(|parent| {
            let pos = ev.1;
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
