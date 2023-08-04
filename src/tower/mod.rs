use self::foundation::{build_tower_system, set_next_selected_system};
use self::light::{contact_light_on_system, flash_light_system, light_off_system};
use self::machine_gun::spawn_tower_machine_gun;
use self::microwave::spawn_tower_microwave;
use self::progress_bar::{progress_bar_scale_system, progress_count_up, ProgressBar};
use self::tesla::spawn_tower_tesla;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use crate::utils::collision_events::TowerBaseCollisionStartEvent;
use crate::utils::RelParent;
use crate::world::PinballWorld;
use crate::GameState;
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::{Delay, EaseFunction, Sequence, Tween};
use std::time::Duration;

pub mod base;
pub mod foundation;
pub mod light;
mod machine_gun;
mod microwave;
mod progress_bar;
mod tesla;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTowerEvent>().add_systems(
            Update,
            (
                tower_on_collision_system,
                rotate_tower_head_system,
                light_off_system,
                foundation::on_collision_system,
                progress_bar_scale_system,
                flash_light_system,
                build_tower_system,
                spawn_tower_system,
                set_next_selected_system,
                contact_light_on_system,
            )
                .run_if(in_state(GameState::Ingame)),
        );
    }
}

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

fn tower_on_collision_system(
    mut cmds: Commands,
    mut evs: EventReader<TowerBaseCollisionStartEvent>,
    mut q_progress: Query<(&RelParent, &mut ProgressBar)>,
) {
    for ev in evs.iter() {
        tower_pogress(ev.0, &mut cmds, &mut q_progress);
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

fn tower_pogress(
    parent_id: Entity,
    cmds: &mut Commands,
    q_progress: &mut Query<(&RelParent, &mut ProgressBar)>,
) {
    if progress_count_up(parent_id, 0.05, q_progress) >= 1. {
        //cmds.entity(parent_id).insert(ReadyToBuild);
    }
}
