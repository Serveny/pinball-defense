use self::load_game::SaveList;
use self::settings::{SettingsMenuLayout, SettingsMenuState};
use self::settings::{on_changed_graphics_settings, on_changed_sound_settings};
use crate::AppState;
use crate::game::KeyboardControls;
use crate::prelude::*;
use crate::settings::{GraphicsSettings, SoundSettings};
use bevy::input_focus::{FocusCause, InputFocus};
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigation;

mod actions;
mod confirm_popup;
pub mod gamepad;
mod load_game;
mod main_menu;
mod pause;
mod settings;
mod settings_menu;
pub mod tools;
mod utils;

pub use actions::MenuAction;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum MenuState {
    #[default]
    None,
    MainMenu,
    Settings,
    PauseMenu,
    LoadGame,
    SaveGame,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .init_state::<SettingsMenuState>()
            .init_resource::<SettingsReturnMenu>()
            .init_resource::<SavedInPauseMenu>()
            .add_message::<MenuAction>()
            .add_plugins(bevy::input_focus::directional_navigation::DirectionalNavigationPlugin)
            .add_systems(OnEnter(AppState::MainMenu), enter_main_menu)
            .add_systems(
                OnEnter(MenuState::MainMenu),
                (
                    clear_menu_layout,
                    main_menu::layout.after(clear_menu_layout),
                ),
            )
            .add_systems(
                OnEnter(MenuState::Settings),
                (
                    clear_menu_layout,
                    settings_menu::layout.after(clear_menu_layout),
                ),
            )
            .add_systems(
                OnEnter(MenuState::PauseMenu),
                (clear_menu_layout, pause::layout.after(clear_menu_layout)),
            )
            .add_systems(
                OnEnter(MenuState::LoadGame),
                (
                    clear_menu_layout,
                    load_game::load_game_layout.after(clear_menu_layout),
                ),
            )
            .add_systems(
                OnEnter(MenuState::SaveGame),
                (
                    clear_menu_layout,
                    load_game::save_game_layout.after(clear_menu_layout),
                ),
            )
            .add_systems(
                Update,
                (
                    ensure_menu_camera,
                    actions::on_menu_action,
                    tools::menu_btn::system,
                    tools::menu_btn::focus_system,
                    tools::focus_hover_system,
                    gamepad::navigation_system,
                    gamepad::activate_system,
                    gamepad::back_system,
                    tools::hover_focus_system,
                    focus_first_widget,
                    tools::sliders::update_thumb_position,
                    tools::sliders::update_thumb_style,
                    tools::checkbox::update_mark_visibility,
                    tools::scrollbar::update_visibility,
                    confirm_popup::restrict_navigation,
                )
                    .run_if(in_menu),
            )
            .add_systems(OnEnter(MenuState::None), clean_up)
            .add_systems(
                OnTransition {
                    exited: MenuState::None,
                    entered: MenuState::PauseMenu,
                },
                reset_saved_in_pause_menu,
            );
        add_settings_state_systems(app);
        app.add_systems(
            Update,
            on_changed_graphics_settings.run_if(in_state(SettingsMenuState::Graphics)),
        )
        .add_systems(
            Update,
            on_changed_sound_settings.run_if(in_state(SettingsMenuState::Sound)),
        );
    }
}

fn add_settings_state_systems(app: &mut App) {
    app.add_systems(
        OnEnter(SettingsMenuState::Sound),
        (
            clear_focus.before(settings::clean_up),
            settings::clean_up,
            settings::layout::<SoundSettings>.after(settings::clean_up),
        ),
    )
    .add_systems(
        OnEnter(SettingsMenuState::Graphics),
        (
            clear_focus.before(settings::clean_up),
            settings::clean_up,
            settings::layout::<GraphicsSettings>.after(settings::clean_up),
        ),
    )
    .add_systems(
        OnEnter(SettingsMenuState::KeyboardControls),
        (
            clear_focus.before(settings::clean_up),
            settings::clean_up,
            settings::layout::<KeyboardControls>.after(settings::clean_up),
        ),
    )
    .add_systems(OnExit(SettingsMenuState::Sound), settings::clean_up)
    .add_systems(OnExit(SettingsMenuState::Graphics), settings::clean_up)
    .add_systems(
        OnExit(SettingsMenuState::KeyboardControls),
        settings::clean_up,
    );
}

fn in_menu(state: Res<State<MenuState>>) -> bool {
    matches!(
        *state.get(),
        MenuState::MainMenu
            | MenuState::Settings
            | MenuState::PauseMenu
            | MenuState::LoadGame
            | MenuState::SaveGame
    )
}

// When a menu (or submenu) opens, its widgets are spawned a frame later; the
// old focus is either gone or dangling, so focus the first widget of the new
// layout, in spawn order. With the mouse hovering a widget, hover_focus_system
// keeps control instead.
pub fn focus_first_widget(
    mut focus: ResMut<InputFocus>,
    q_layout: Query<Entity, With<SettingsMenuLayout>>,
    q_save_list: Query<Entity, With<SaveList>>,
    q_widget: Query<Entity, With<AutoDirectionalNavigation>>,
    children: Query<&Children>,
) {
    let valid = focus
        .get()
        .is_some_and(|f| q_widget.contains(f) && children.contains(f));
    if valid {
        return;
    }
    let mut first = None;
    for layout in q_layout.iter().chain(q_save_list.iter()) {
        for descendant in children.iter_descendants(layout) {
            if q_widget.contains(descendant) {
                first = Some(descendant);
                break;
            }
        }
    }
    if let Some(first) = first.or_else(|| q_widget.iter().next()) {
        focus.set(first, FocusCause::Navigated);
    }
}

fn clear_focus(mut focus: ResMut<InputFocus>) {
    focus.clear();
}

fn enter_main_menu(
    mut cmds: Commands,
    mut menu_state: ResMut<NextState<MenuState>>,
    go_to_load: Option<Res<GoToLoadGame>>,
) {
    if go_to_load.is_some() {
        menu_state.set(MenuState::LoadGame);
        cmds.remove_resource::<GoToLoadGame>();
    } else {
        menu_state.set(MenuState::MainMenu);
    }
}

#[derive(Resource)]
pub struct GoToLoadGame;

fn reset_saved_in_pause_menu(mut cmds: Commands) {
    cmds.remove_resource::<SavedInPauseMenu>();
}

fn ensure_menu_camera(
    mut cmds: Commands,
    app_state: Res<State<AppState>>,
    q_cam: Query<Entity, With<MenuCamera>>,
) {
    if q_cam.iter().next().is_some() {
        return;
    }
    // During gameplay the pinball camera renders the UI, so no extra menu camera is needed.
    if app_state.get() == &AppState::Game {
        return;
    }
    cmds.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        MenuCamera,
    ));
}

fn clear_menu_layout(mut cmds: Commands, q_layout: Query<Entity, With<MenuLayout>>) {
    for id in q_layout.iter() {
        cmds.entity(id).despawn();
    }
}

fn clean_up(
    mut cmds: Commands,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
    q_layout: Query<Entity, With<MenuLayout>>,
    q_cam: Query<Entity, With<MenuCamera>>,
) {
    settings_state.set(SettingsMenuState::None);
    for id in q_layout.iter() {
        cmds.entity(id).despawn();
    }
    for id in q_cam.iter() {
        cmds.entity(id).despawn();
    }
}

#[derive(Component, Clone, Default)]
pub struct MenuLayout;

#[derive(Component, Clone, Default)]
struct MenuCamera;

#[derive(Resource, Clone)]
struct SettingsReturnMenu(MenuState);

impl Default for SettingsReturnMenu {
    fn default() -> Self {
        Self(MenuState::MainMenu)
    }
}

#[derive(Resource, Default)]
pub struct SavedInPauseMenu;
