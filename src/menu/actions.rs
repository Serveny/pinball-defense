use super::confirm_popup::{self, ConfirmPopup};
use super::tools::menu_btn::MenuButtonData;
use super::{MenuLayout, MenuState, SavedInPauseMenu, SettingsMenuState, SettingsReturnMenu};
use crate::AppState;
use crate::game::ResumeGameEvent;
use crate::game::{load_game, save_game, spawn_save_screenshot};
use crate::prelude::*;
use bevy::app::AppExit;
use bevy::input_focus::{FocusCause, InputFocus};

#[derive(Message, Component, Debug, Clone, Default, PartialEq)]
pub enum MenuAction {
    #[default]
    Continue,
    NewGame,
    LoadGame,
    SaveGame,
    Save(String),
    Load(String),
    Settings,
    Back,
    Controls,
    Graphics,
    Sound,
    Quit,
    BackToMainMenu,
    ConfirmBackToMainMenu,
    CancelBackToMainMenu,
}

impl MenuAction {
    pub fn label(&self) -> &'static str {
        match self {
            MenuAction::Continue => "Continue",
            MenuAction::NewGame => "New Game",
            MenuAction::LoadGame => "Load Game",
            MenuAction::SaveGame => "Save Game",
            MenuAction::Save(_) => "Save",
            MenuAction::Load(_) => "Load",
            MenuAction::Settings => "Settings",
            MenuAction::Back => "Back",
            MenuAction::Controls => "Controls",
            MenuAction::Graphics => "Graphics",
            MenuAction::Sound => "Sound",
            MenuAction::Quit => "Quit",
            MenuAction::BackToMainMenu => "Main Menu",
            MenuAction::ConfirmBackToMainMenu => "Back to Main Menu",
            MenuAction::CancelBackToMainMenu => "Cancel",
        }
    }
}

pub fn on_menu_action(
    mut evr: MessageReader<MenuAction>,
    menu_state: Res<State<MenuState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut exit_ev: MessageWriter<AppExit>,
    settings_state: Res<State<SettingsMenuState>>,
    mut next_settings_state: ResMut<NextState<SettingsMenuState>>,
    mut settings_return: ResMut<SettingsReturnMenu>,
    mut resume_ev: MessageWriter<ResumeGameEvent>,
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    q_popup: Query<Entity, With<ConfirmPopup>>,
    mut q_layout: Query<&mut Visibility, With<MenuLayout>>,
    q_btn: Query<(Entity, &MenuButtonData)>,
    saved_in_pause_menu: Option<Res<SavedInPauseMenu>>,
    mut focus: ResMut<InputFocus>,
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
            MA::Save(path) => {
                save_game(&mut cmds, path.clone());
                for mut vis in &mut q_layout {
                    *vis = Visibility::Hidden;
                }
                spawn_save_screenshot(&mut cmds, path.clone().into());
            }
            MA::Load(path) => {
                load_game(&mut cmds, path.clone());
                next_menu_state.set(MenuState::None);
                app_state.set(AppState::Game);
            }
            MA::Settings => {
                settings_return.0 = menu_state.get().clone();
                next_menu_state.set(MenuState::Settings);
            }
            MA::Back => {
                if settings_state.get() != &SettingsMenuState::None {
                    let return_action = match settings_state.get() {
                        SettingsMenuState::KeyboardControls => MenuAction::Controls,
                        SettingsMenuState::Graphics => MenuAction::Graphics,
                        SettingsMenuState::Sound => MenuAction::Sound,
                        SettingsMenuState::None => unreachable!(),
                    };
                    next_settings_state.set(SettingsMenuState::None);
                    if let Some((entity, _)) = q_btn.iter().find(|(_, d)| d.action == return_action) {
                        focus.set(entity, FocusCause::Navigated);
                    }
                } else {
                    let target = match menu_state.get() {
                        MenuState::LoadGame => MenuState::MainMenu,
                        MenuState::SaveGame => MenuState::PauseMenu,
                        MenuState::Settings => settings_return.0.clone(),
                        _ => MenuState::MainMenu,
                    };
                    next_menu_state.set(target);
                }
            }
            MA::Controls => next_settings_state.set(SettingsMenuState::KeyboardControls),
            MA::Graphics => next_settings_state.set(SettingsMenuState::Graphics),
            MA::Sound => next_settings_state.set(SettingsMenuState::Sound),
            MA::Quit => {
                exit_ev.write(AppExit::Success);
            }
            MA::BackToMainMenu => {
                if menu_state.get() == &MenuState::PauseMenu {
                    if saved_in_pause_menu.is_some() {
                        next_menu_state.set(MenuState::None);
                        app_state.set(AppState::MainMenu);
                    } else {
                        confirm_popup::spawn(&mut cmds, &assets);
                    }
                }
            }
            MA::ConfirmBackToMainMenu => {
                confirm_popup::despawn(&mut cmds, &q_popup);
                next_menu_state.set(MenuState::None);
                app_state.set(AppState::MainMenu);
            }
            MA::CancelBackToMainMenu => {
                confirm_popup::despawn(&mut cmds, &q_popup);
            }
        }
    }
}
