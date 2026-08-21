use super::{MenuState, SettingsMenuState};
use crate::AppState;
use crate::game::ResumeGameEvent;
use crate::prelude::*;
use bevy::app::AppExit;

#[derive(Message, Component, Debug, Clone, Copy, Default)]
pub enum MenuAction {
    #[default]
    Continue,
    NewGame,
    LoadGame,
    SaveGame,
    Settings,
    Back,
    Controls,
    Graphics,
    Sound,
    Quit,
}

impl MenuAction {
    pub fn label(&self) -> &'static str {
        match self {
            MenuAction::Continue => "Continue",
            MenuAction::NewGame => "New Game",
            MenuAction::LoadGame => "Load Game",
            MenuAction::SaveGame => "Save Game",
            MenuAction::Settings => "Settings",
            MenuAction::Back => "Back",
            MenuAction::Controls => "Controls",
            MenuAction::Graphics => "Graphics",
            MenuAction::Sound => "Sound",
            MenuAction::Quit => "Quit",
        }
    }
}

pub fn on_menu_action(
    mut evr: MessageReader<MenuAction>,
    menu_state: Res<State<MenuState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut exit_ev: MessageWriter<AppExit>,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
    mut resume_ev: MessageWriter<ResumeGameEvent>,
) {
    for action in evr.read() {
        use MenuAction as MA;
        match action {
            MA::Continue => {
                next_menu_state.set(MenuState::None);
                resume_ev.write(ResumeGameEvent);
            }
            MA::NewGame => {
                next_menu_state.set(MenuState::None);
                app_state.set(AppState::Game);
            }
            MA::LoadGame => next_menu_state.set(MenuState::LoadGame),
            MA::SaveGame => next_menu_state.set(MenuState::SaveGame),
            MA::Settings => next_menu_state.set(MenuState::Settings),
            MA::Back => {
                settings_state.set(SettingsMenuState::None);
                let target = match menu_state.get() {
                    MenuState::LoadGame => MenuState::MainMenu,
                    MenuState::SaveGame => MenuState::PauseMenu,
                    _ => MenuState::MainMenu,
                };
                next_menu_state.set(target);
            }
            MA::Controls => settings_state.set(SettingsMenuState::KeyboardControls),
            MA::Graphics => settings_state.set(SettingsMenuState::Graphics),
            MA::Sound => settings_state.set(SettingsMenuState::Sound),
            MA::Quit => {
                exit_ev.write(AppExit::Success);
            }
        }
    }
}
