use super::{MenuState, SettingsMenuState};
use crate::game::ResumeGameEvent;
use crate::prelude::*;
use crate::settings::SoundSettings;
use bevy::app::AppExit;
use std::fmt;
//const NORMAL_BUTTON: Color = Color::rgb(57. / 255., 61. / 255., 64. / 255.);

#[derive(Event, Component, Debug, Clone, Copy)]
pub enum MenuAction {
    Continue,
    Controls,
    Graphics,
    Sound,
    EditSound(SoundAction),
    Quit,
}

impl MenuAction {
    pub fn set_val(&mut self, val: f32) {
        match self {
            MenuAction::EditSound(sound) => match sound {
                SoundAction::MusicVolume(v) => *v = val,
                SoundAction::FxVolume(v) => *v = val,
            },
            MenuAction::Continue => (),
            _ => (),
        }
    }
    pub fn val(&self) -> f32 {
        match self {
            MenuAction::EditSound(sound) => match sound {
                SoundAction::MusicVolume(v) => *v,
                SoundAction::FxVolume(v) => *v,
            },
            MenuAction::Continue => 0.,
            _ => 0.,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SoundAction {
    MusicVolume(f32),
    FxVolume(f32),
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
    mut sound_settings: ResMut<SoundSettings>,
) {
    for action in evr.iter() {
        use MenuAction as MA;
        match action {
            MA::Continue => {
                menu_state.set(MenuState::None);
                resume_ev.send(ResumeGameEvent);
            }
            MA::Controls => settings_state.set(SettingsMenuState::Controls),
            MA::Graphics => settings_state.set(SettingsMenuState::Graphics),
            MA::Sound => settings_state.set(SettingsMenuState::Sound),
            MA::Quit => exit_ev.send(AppExit),

            // Settings
            MA::EditSound(sound_action) => match sound_action {
                SoundAction::FxVolume(vol) => sound_settings.fx_volume = *vol,
                SoundAction::MusicVolume(vol) => sound_settings.music_volume = *vol,
            },
        }
    }
}
