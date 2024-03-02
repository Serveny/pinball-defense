use self::settings::{on_changed_graphics_settings, on_changed_sound_settings};
use self::{actions::MenuAction, settings::SettingsMenuState};
use crate::game::KeyboardControls;
use crate::prelude::*;
use crate::settings::{GraphicsSettings, SoundSettings};

mod actions;
mod pause;
mod settings;
mod tools;

// State used for the current menu screen
#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum MenuState {
    #[default]
    None,
    PauseMenu,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .init_state::<SettingsMenuState>()
            .add_event::<MenuAction>()
            .add_systems(OnEnter(MenuState::PauseMenu), pause::layout)
            .add_systems(
                Update,
                (
                    actions::on_menu_action,
                    tools::menu_btn::system,
                    tools::sliders::system,
                    tools::checkbox::system,
                    tools::keybox::system,
                )
                    .run_if(in_state(MenuState::PauseMenu)),
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

fn clean_up(
    mut cmds: Commands,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
    q_layout: Query<Entity, With<MenuLayout>>,
) {
    settings_state.set(SettingsMenuState::None);
    for id in q_layout.iter() {
        cmds.entity(id).despawn_recursive();
    }
}

#[derive(Component)]
struct MenuLayout;
