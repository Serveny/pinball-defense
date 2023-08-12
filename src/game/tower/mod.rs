use self::foundation::DespawnFoundationEvent;
use super::GameState;
use crate::game::world::QueryWorld;
use crate::prelude::*;
use crate::settings::GraphicsSettings;
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::{Delay, EaseFunction, Sequence, Tween};
use std::time::Duration;
pub use types::TowerType;
use types::*;

mod animations;
pub mod base;
pub mod foundation;
pub mod light;
mod target;
mod types;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTowerEvent>()
            .add_event::<DespawnFoundationEvent>()
            .add_systems(
                Update,
                (
                    animations::rotate_always_system,
                    animations::rotate_to_target_system,
                    base::progress_system,
                    foundation::despawn_system,
                    foundation::progress_system,
                    foundation::set_next_selected_system,
                    foundation::set_ready_to_build_system,
                    light::contact_light_on_system,
                    light::flash_light_system,
                    light::light_off_system,
                    target::aim_first_enemy_system,
                    target::enemy_within_reach_system,
                    target::remove_despawned_enemies_from_ewr_system,
                    target::target_pos_by_afe_system,
                    spawn_tower_system,
                )
                    .run_if(in_state(GameState::Ingame)),
            );
    }
}
#[derive(Component)]
pub struct Tower;

#[derive(Component)]
pub struct TowerHead;

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

#[derive(Event)]
pub struct SpawnTowerEvent(pub TowerType, pub Vec3);

fn spawn_tower_system(
    mut cmds: Commands,
    mut evs: EventReader<SpawnTowerEvent>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    assets: Res<PinballDefenseAssets>,
    q_pbw: QueryWorld,
    g_sett: Res<GraphicsSettings>,
) {
    for ev in evs.iter() {
        cmds.entity(q_pbw.single()).with_children(|parent| {
            let pos = ev.1;
            match ev.0 {
                TowerType::Gun => gun::spawn(parent, &mut mats, &assets, &g_sett, pos),
                TowerType::Tesla => tesla::spawn(parent, &mut mats, &assets, &g_sett, pos),
                TowerType::Microwave => microwave::spawn(parent, &mut mats, &assets, &g_sett, pos),
            };
        });
    }
}
