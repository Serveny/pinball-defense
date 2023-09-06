use self::{actions::MenuAction, settings::SettingsMenuState};
use crate::game::KeyboardControls;
use crate::prelude::*;
use crate::settings::{GraphicsSettings, SoundSettings};

mod actions;
mod pause;
mod settings;
mod tools;

const WHITE: Color = Color::rgb(1., 254. / 255., 236. / 255.);
const GRAY: Color = Color::rgb(65. / 255., 69. / 255., 72. / 255.);
const GOLD: Color = Color::rgb(188. / 255., 148. / 255., 87. / 255.);
const BACKGROUND: Color = Color::rgba(23. / 255., 24. / 255., 26. / 255., 120. / 255.);
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
        app.add_state::<MenuState>()
            .add_state::<SettingsMenuState>()
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
