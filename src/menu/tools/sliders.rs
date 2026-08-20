use super::{Active, PropIndex};
use crate::menu::settings::SettingsMenuState;
use crate::prelude::*;
use crate::settings::{GraphicsSettings, SoundSettings};
use crate::utils::GameColor;
use crate::utils::reflect::set_field;
use bevy::ui::RelativeCursorPosition;

#[derive(Component, Clone, Default)]
pub struct Slider;

#[derive(Component, Clone, Default)]
pub struct SliderKnob;

// init_val must be between 0 and 1
pub fn spawn(p: &mut ChildSpawnerCommands, prop_i: usize, init_val: f32) {
    p.spawn_empty().queue_apply_scene(bsn! {
        #Slider
        Slider
        Node {
            position_type: PositionType::Relative,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            min_width: Val::Px(120.),
        }
        RelativeCursorPosition
        Children [
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    height: Val::Px(10.),
                    width: Val::Percent(100.),
                    margin: UiRect::all(Val::Auto),
                }
                BackgroundColor({GameColor::GOLD})
            ),
            {knob(prop_i, init_val)}
        ]
    });
}

fn knob(prop_i: usize, init_val: f32) -> impl SceneList {
    let size_px: f32 = 40.;
    bsn! {
        Name::new("Slider Knob")
        SliderKnob
        PropIndex({prop_i})
        Active
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.),
            left: Val::Percent({init_val * 100.}),
            width: Val::Px(size_px),
            height: Val::Px(size_px),
            margin: UiRect {
                left: Val::Px({-size_px / 2.}),
                top: Val::Auto,
                bottom: Val::Auto,
                right: Val::Px(0.),
            },
            border: UiRect::all(Val::Px(4.0)),
        }
        BorderColor::from(GameColor::GOLD)
        BackgroundColor({GameColor::WHITE})
        Button
    }
}

pub fn system(
    mut interaction_query: Query<
        (
            &Interaction,
            &ChildOf,
            &mut BorderColor,
            &mut Node,
            &PropIndex,
        ),
        (With<SliderKnob>, With<Active>),
    >,
    mut g_sett: ResMut<GraphicsSettings>,
    mut s_sett: ResMut<SoundSettings>,
    menu_state: Res<State<SettingsMenuState>>,
    q_spawner: Query<&RelativeCursorPosition, With<Slider>>,
) {
    for (interaction, child_of, mut border_color, mut style, prop_i) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Ok(rel_pos) = q_spawner.get(child_of.parent()) {
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
                *border_color = GameColor::GRAY.into();
            }
            Interaction::None => {
                *border_color = GameColor::GOLD.into();
            }
        }
    }
}
