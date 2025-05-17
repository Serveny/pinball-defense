use super::{MenuState, SettingsMenuState};
use crate::game::ResumeGameEvent;
use crate::prelude::*;
use bevy::app::AppExit;
use std::fmt;
//const NORMAL_BUTTON: Color = Color::rgb(57. / 255., 61. / 255., 64. / 255.);

#[derive(Component, Event, Debug, Clone, Copy)]
pub enum MenuAction {
    Continue,
    Controls,
    Graphics,
    Sound,
    Quit,
}

impl fmt::Display for MenuAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

pub fn on_menu_action(
    mut evr: EventReader<MenuAction>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut exit_ev: EventWriter<AppExit>,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
    mut resume_ev: EventWriter<ResumeGameEvent>,
) {
    for action in evr.read() {
        use MenuAction as MA;
        match action {
            MA::Continue => {
                menu_state.set(MenuState::None);
                resume_ev.write(ResumeGameEvent);
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
