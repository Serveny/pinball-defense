use super::{camera::PinballCamera, GameState};
use crate::prelude::*;
use crate::utils::RelEntity;

mod controls;
pub mod progress_bar;

#[derive(States, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UiState {
    #[default]
    None,
    Controls,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UiState>()
            .add_systems(OnEnter(UiState::Controls), controls::spawn)
            .add_systems(
                Update,
                (
                    (controls::keys_to_pos_system, controls::on_resize_system)
                        .run_if(in_state(UiState::Controls)),
                    (
                        update_pos_system,
                        progress_bar::despawn_system,
                        progress_bar::show_progress_system,
                        progress_bar::activate_animation_system,
                    )
                        .run_if(in_state(GameState::Ingame)),
                ),
            )
            .add_systems(OnExit(UiState::Controls), controls::despawn);
    }
}

#[derive(Component)]
struct PosToRelEntity;

fn update_pos_system(
    mut q_bar: Query<(&mut Node, &RelEntity), With<PosToRelEntity>>,
    q_trans: Query<(Entity, &Transform)>,
    q_cam: Query<(&GlobalTransform, &Camera), With<PinballCamera>>,
) {
    let Ok((cam_trans, cam)) = q_cam.single() else {
        return;
    };
    for (mut node, rel_id) in q_bar.iter_mut() {
        let Ok((_, obj_trans)) = q_trans.get(rel_id.0) else {
            continue;
        };
        let screen_pos = project_3d_to_2d_screen(obj_trans.translation, cam_trans, cam);
        node.left = Val::Px(screen_pos.x);
        node.top = Val::Px(screen_pos.y);
    }
}

fn project_3d_to_2d_screen(obj_pos: Vec3, cam_trans: &GlobalTransform, cam: &Camera) -> Vec2 {
    cam.world_to_viewport(cam_trans, obj_pos)
        .unwrap_or_default()
}
