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
    mut menu_state: ResMut<NextState<MenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut exit_ev: MessageWriter<AppExit>,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
    mut resume_ev: MessageWriter<ResumeGameEvent>,
) {
    for action in evr.read() {
        use MenuAction as MA;
        match action {
            MA::Continue => {
                menu_state.set(MenuState::None);
                resume_ev.write(ResumeGameEvent);
            }
            MA::NewGame | MA::LoadGame => {
                menu_state.set(MenuState::None);
                app_state.set(AppState::Game);
            }
            MA::Settings => menu_state.set(MenuState::Settings),
            MA::Back => {
                settings_state.set(SettingsMenuState::None);
                menu_state.set(MenuState::MainMenu);
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
