use super::{Active, PropIndex};
use crate::menu::settings::SettingsMenuState;
use crate::menu::{GOLD, WHITE};
use crate::prelude::*;
use crate::settings::{GraphicsSettings, SoundSettings};
use crate::utils::reflect::toggle_field_bool;

#[derive(Component)]
pub struct Checkbox;

#[derive(Component)]
pub struct CheckboxMark;

pub fn spawn(p: &mut ChildBuilder, prop_i: usize, init_val: bool) {
    p.spawn((
        Name::new("Checkbox"),
        Checkbox,
        ButtonBundle {
            style: Style {
                width: Val::Px(40.),
                height: Val::Px(40.),
                border: UiRect::all(Val::Px(5.0)),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            border_color: GOLD.into(),
            background_color: Color::NONE.into(),
            ..default()
        },
        PropIndex(prop_i),
        Active,
    ))
    .with_children(|p| {
        p.spawn((
            NodeBundle {
                background_color: GOLD.into(),
                style: Style {
                    width: Val::Px(20.),
                    height: Val::Px(20.),
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                visibility: match init_val {
                    true => Visibility::Inherited,
                    false => Visibility::Hidden,
                },
                ..default()
            },
            CheckboxMark,
        ));
    });
}

#[allow(clippy::type_complexity)]
pub fn system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BorderColor, &PropIndex),
        (Changed<Interaction>, With<Checkbox>, With<Active>),
    >,
    mut q_mark: Query<(&mut Visibility, &mut BackgroundColor, &Parent), With<CheckboxMark>>,
    mut g_sett: ResMut<GraphicsSettings>,
    mut s_sett: ResMut<SoundSettings>,
    menu_state: Res<State<SettingsMenuState>>,
) {
    for (checkbox_id, interaction, mut border_color, prop_i) in &mut interaction_query {
        if let Some((mut visi, mut bg_color, _)) = q_mark
            .iter_mut()
            .find(|(_, _, parent)| parent.get() == checkbox_id)
        {
            match *interaction {
                Interaction::Pressed => {
                    let val = match **menu_state {
                        SettingsMenuState::Sound => {
                            toggle_field_bool(&mut s_sett as &mut SoundSettings, prop_i.0)
                        }
                        SettingsMenuState::Graphics => {
                            toggle_field_bool(&mut g_sett as &mut GraphicsSettings, prop_i.0)
                        }
                        _ => false,
                    };
                    *visi = match val {
                        true => Visibility::Inherited,
                        false => Visibility::Hidden,
                    };
                }
                Interaction::Hovered => {
                    *border_color = WHITE.into();
                    *bg_color = WHITE.into();
                }
                Interaction::None => {
                    *border_color = GOLD.into();
                    *bg_color = GOLD.into();
                }
            }
        }
    }
}