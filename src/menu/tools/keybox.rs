use super::{Active, PropIndex};
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::color::palettes::css::GRAY;

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
        Button::default(),
        Node {
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
        BorderColor(GameColor::GOLD),
        BackgroundColor(GRAY.into()),
        PropIndex(prop_i),
        Active,
    ))
    .with_children(|p| {
        p.spawn((
            Text(format!("{init_val:?}").replace("Key", "").to_string()),
            TextFont {
                font: assets.menu_font.clone(),
                font_size: 40.0,
                ..default()
            },
            TextColor(GameColor::WHITE),
        ));
    });
}

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
                *border_color = GameColor::WHITE.into();
            }
            Interaction::None => {
                *border_color = GameColor::GOLD.into();
            }
        }
    }
}
