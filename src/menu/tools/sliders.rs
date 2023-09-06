use super::{Active, PropIndex};
use crate::menu::settings::SettingsMenuState;
use crate::menu::{GOLD, GRAY, WHITE};
use crate::prelude::*;
use crate::settings::{GraphicsSettings, SoundSettings};
use crate::utils::reflect::set_field;
use bevy::ui::RelativeCursorPosition;

#[derive(Component)]
pub struct Slider;

// init_val must be between 0 and 1
pub fn spawn(p: &mut ChildBuilder, prop_i: usize, init_val: f32) {
    p.spawn((
        Name::new("Slider"),
        Slider,
        NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        },
        RelativeCursorPosition::default(),
    ))
    .with_children(|p| {
        p.spawn(NodeBundle {
            background_color: GOLD.into(),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                left: Val::Px(0.),
                height: Val::Px(10.),
                width: Val::Percent(100.),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            ..default()
        });
        p.spawn(knob(prop_i, init_val));
    });
}

#[derive(Component)]
pub struct SliderKnob;

fn knob(prop_i: usize, init_val: f32) -> impl Bundle {
    let size_px = 40.;
    (
        Name::new("Slider Knob"),
        SliderKnob,
        PropIndex(prop_i),
        Active,
        ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                left: Val::Percent(init_val * 100.),
                width: Val::Px(size_px),
                height: Val::Px(size_px),
                margin: UiRect {
                    left: Val::Px(-size_px / 2.),
                    top: Val::Auto,
                    bottom: Val::Auto,
                    right: Val::Px(0.),
                },
                border: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            border_color: GOLD.into(),
            background_color: WHITE.into(),
            ..default()
        },
    )
}
#[allow(clippy::type_complexity)]
pub fn system(
    mut interaction_query: Query<
        (
            &Interaction,
            &Parent,
            &mut BorderColor,
            &mut Style,
            &PropIndex,
        ),
        (With<SliderKnob>, With<Active>),
    >,
    mut g_sett: ResMut<GraphicsSettings>,
    mut s_sett: ResMut<SoundSettings>,
    menu_state: Res<State<SettingsMenuState>>,
    q_parent: Query<&RelativeCursorPosition, With<Slider>>,
) {
    for (interaction, parent, mut border_color, mut style, prop_i) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Ok(rel_pos) = q_parent.get(parent.get()) {
                    if let Some(rel_pos) = rel_pos.normalized {
                        let val = rel_pos.x.clamp(0., 1.);
                        style.left = Val::Percent(val * 100.);
                        match **menu_state {
                            SettingsMenuState::Sound => {
                                set_field(
                                    &mut s_sett as &mut SoundSettings,
                                    prop_i.0,
                                    Box::new(val),
                                );
                            }
                            SettingsMenuState::Graphics => {
                                set_field(
                                    &mut g_sett as &mut GraphicsSettings,
                                    prop_i.0,
                                    Box::new(val),
                                );
                            }
                            _ => (),
                        };
                    }
                }
            }
            Interaction::Hovered => {
                *border_color = GRAY.into();
            }
            Interaction::None => {
                *border_color = GOLD.into();
            }
        }
    }
}
