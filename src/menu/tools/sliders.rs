use crate::menu::actions::MenuAction;
use crate::menu::{GOLD, GRAY, WHITE};
use crate::prelude::*;
use bevy::ui::RelativeCursorPosition;

#[derive(Component)]
pub struct Slider;

// init_val must be between 0 and 1
pub fn spawn_slider(action: MenuAction, p: &mut ChildBuilder) {
    p.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                ..default()
            },
            ..default()
        },
        Slider,
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
        p.spawn(knob(action));
    });
}

#[derive(Component)]
pub struct SliderKnob;

fn knob(action: MenuAction) -> impl Bundle {
    let size_px = 40.;
    (
        ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                left: Val::Percent(action.val() * 100.),
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
        SliderKnob,
        action,
    )
}
#[allow(clippy::type_complexity)]
pub fn slider_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &Parent,
            &mut BorderColor,
            &mut Style,
            &mut MenuAction,
        ),
        (With<Button>, With<SliderKnob>),
    >,
    q_parent: Query<&RelativeCursorPosition, With<Slider>>,
    mut action_ev: EventWriter<MenuAction>,
) {
    for (interaction, parent, mut border_color, mut style, mut action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Ok(rel_pos) = q_parent.get(parent.get()) {
                    if let Some(rel_pos) = rel_pos.normalized {
                        if (0.0..1.0).contains(&rel_pos.x) {
                            style.left = Val::Percent(rel_pos.x * 100.);
                            action.set_val(rel_pos.x);
                            action_ev.send(*action);
                        }
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
