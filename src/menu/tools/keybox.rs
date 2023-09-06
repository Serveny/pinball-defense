use super::{Active, PropIndex};
use crate::menu::{GOLD, WHITE};
use crate::prelude::*;

#[derive(Component)]
pub struct Keybox;

pub fn spawn(
    p: &mut ChildBuilder,
    assets: &PinballDefenseAssets,
    prop_i: usize,
    init_val: KeyCode,
) {
    p.spawn((
        Name::new("Key"),
        Keybox,
        ButtonBundle {
            style: Style {
                width: Val::Px(130.),
                height: Val::Px(55.),
                border: UiRect::all(Val::Px(5.0)),
                margin: UiRect::all(Val::Auto),
                padding: UiRect::all(Val::Auto),
                display: Display::Flex,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            border_color: GOLD.into(),
            background_color: Color::GRAY.into(),
            ..default()
        },
        PropIndex(prop_i),
        Active,
    ))
    .with_children(|p| {
        p.spawn(TextBundle::from_section(
            format!("{init_val:?}"),
            TextStyle {
                font: assets.menu_font.clone(),
                font_size: 40.0,
                color: WHITE,
            },
        ));
    });
}

#[allow(clippy::type_complexity)]
pub fn system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<Button>, With<Keybox>),
    >,
) {
    for (interaction, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => (),
            Interaction::Hovered => {
                *border_color = WHITE.into();
            }
            Interaction::None => {
                *border_color = GOLD.into();
            }
        }
    }
}
