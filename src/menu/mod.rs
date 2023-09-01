use self::pause::pause_menu_layout;
use crate::game::ResumeGameEvent;
use crate::prelude::*;
use std::fmt;

mod pause;

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
            .add_systems(OnEnter(MenuState::PauseMenu), pause_menu_layout)
            .add_systems(
                Update,
                (button_system).run_if(in_state(MenuState::PauseMenu)),
            )
            .add_systems(OnEnter(MenuState::None), clean_up);
    }
}

fn clean_up(mut cmds: Commands, q_layout: Query<Entity, With<MenuLayout>>) {
    for id in q_layout.iter() {
        cmds.entity(id).despawn_recursive();
    }
}

//const NORMAL_BUTTON: Color = Color::rgb(57. / 255., 61. / 255., 64. / 255.);
const WHITE: Color = Color::rgb(1., 254. / 255., 236. / 255.);
const GOLD: Color = Color::rgb(188. / 255., 148. / 255., 87. / 255.);

#[derive(Component, Debug, Clone, Copy)]
enum ButtonAction {
    Continue,
    Controls,
    Settings,
}

impl fmt::Display for ButtonAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[allow(clippy::type_complexity)]
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut resume_ev: EventWriter<ResumeGameEvent>,
) {
    for (interaction, mut border_color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match action {
                ButtonAction::Continue => {
                    menu_state.set(MenuState::None);
                    resume_ev.send(ResumeGameEvent);
                }
                ButtonAction::Controls => todo!("controls menu"),
                ButtonAction::Settings => todo!("settings menu"),
            },
            Interaction::Hovered => {
                *border_color = WHITE.into();
            }
            Interaction::None => {
                *border_color = GOLD.into();
            }
        }
    }
}

#[derive(Component)]
struct MenuLayout;
