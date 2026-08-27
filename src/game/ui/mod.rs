use super::{GameState, camera::PinballCamera};
use crate::AppState;
use crate::menu::MenuState;
use crate::prelude::*;
use crate::utils::RelEntity;

mod controls;
mod floating_text;
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
            .init_resource::<controls::ControlsUiFade>()
            .add_systems(OnEnter(UiState::Controls), (controls::spawn, reset_controls_fade))
            .add_systems(
                Update,
                (
                    progress_bar::sync_progress_to_entities,
                    progress_bar::ensure_bars_on_load,
                )
                    .run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (
                    (controls::keys_to_pos_system, controls::on_resize_system)
                        .run_if(in_state(UiState::Controls)),
                    controls::update_keys_pos_system.run_if(in_state(UiState::Controls)),
                    (
                        update_pos_system,
                        progress_bar::despawn_system,
                        progress_bar::show_on_hit_system,
                        progress_bar::activate_animation_system,
                        progress_bar::hide_when_fill_complete_system,
                        progress_bar::show_progress_system,
                        progress_bar::hide_after_timeout_system,
                        progress_bar::reset_on_upgrade_system,
                    )
                        .chain()
                        .run_if(in_state(GameState::Ingame)),
                    (
                        floating_text::spawn_system,
                        floating_text::update_system,
                    )
                        .chain()
                        .run_if(in_state(GameState::Ingame)),
                    toggle_ingame_ui_visibility,
                    controls::switch_input_kind_system.run_if(in_state(UiState::Controls)),
                    controls::auto_hide_system.run_if(in_state(UiState::Controls)),
                ),
            )
            .add_systems(OnExit(UiState::Controls), controls::despawn)
            .add_systems(OnExit(AppState::Game), clean_up);
    }
}

fn clean_up(
    mut cmds: Commands,
    mut ui_state: ResMut<NextState<UiState>>,
    q_bars: Query<Entity, With<PosToRelEntity>>,
    q_floating: Query<Entity, With<floating_text::FloatingPoints>>,
) {
    ui_state.set(UiState::None);
    for bar_id in q_bars.iter() {
        cmds.entity(bar_id).despawn();
    }
    for fp_id in q_floating.iter() {
        cmds.entity(fp_id).despawn();
    }
}

fn reset_controls_fade(mut fade: ResMut<controls::ControlsUiFade>) {
    fade.reset();
}

fn toggle_ingame_ui_visibility(
    menu_state: Res<State<MenuState>>,
    mut q_controls: Query<&mut Visibility, With<controls::ControlsUi>>,
) {
    let target = match *menu_state.get() {
        MenuState::None => Visibility::Inherited,
        _ => Visibility::Hidden,
    };
    for mut vis in &mut q_controls {
        *vis = target;
    }
}

#[derive(Component, Clone, Default)]
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
