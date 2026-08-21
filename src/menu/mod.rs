use self::settings::{on_changed_graphics_settings, on_changed_sound_settings};
use self::{actions::MenuAction, settings::SettingsMenuState};
use crate::AppState;
use crate::game::KeyboardControls;
use crate::prelude::*;
use crate::settings::{GraphicsSettings, SoundSettings};

mod actions;
mod main_menu;
mod pause;
mod settings;
mod settings_menu;
mod tools;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum MenuState {
    #[default]
    None,
    MainMenu,
    Settings,
    PauseMenu,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .init_state::<SettingsMenuState>()
            .add_message::<MenuAction>()
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
                Update,
                (
                    actions::on_menu_action,
                    tools::menu_btn::system,
                    tools::sliders::update_thumb_position,
                    tools::sliders::update_thumb_style,
                    tools::checkbox::update_mark_visibility,
                    tools::keybox::system,
                )
                    .run_if(in_menu),
            )
            .add_systems(OnEnter(MenuState::None), clean_up)
            .add_systems(
                OnEnter(SettingsMenuState::Sound),
                (
                    settings::clean_up,
                    settings::layout::<SoundSettings>.after(settings::clean_up),
                ),
            )
            .add_systems(
                OnEnter(SettingsMenuState::Graphics),
                (
                    settings::clean_up,
                    settings::layout::<GraphicsSettings>.after(settings::clean_up),
                ),
            )
            .add_systems(
                OnEnter(SettingsMenuState::KeyboardControls),
                (
                    settings::clean_up,
                    settings::layout::<KeyboardControls>.after(settings::clean_up),
                ),
            )
            .add_systems(
                Update,
                on_changed_graphics_settings.run_if(in_state(SettingsMenuState::Graphics)),
            )
            .add_systems(
                Update,
                on_changed_sound_settings.run_if(in_state(SettingsMenuState::Sound)),
            );
    }
}

fn in_menu(state: Res<State<MenuState>>) -> bool {
    matches!(
        *state.get(),
        MenuState::MainMenu | MenuState::Settings | MenuState::PauseMenu
    )
}

fn enter_main_menu(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::MainMenu);
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
) {
    settings_state.set(SettingsMenuState::None);
    for id in q_layout.iter() {
        cmds.entity(id).despawn();
    }
}

#[derive(Component, Clone, Default)]
struct MenuLayout;
