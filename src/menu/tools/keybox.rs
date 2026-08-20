use super::{Active, PropIndex};
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::color::palettes::css::GRAY;
use bevy::text::{FontSize, FontSourceTemplate};

#[derive(Component, Clone, Default)]
pub struct Keybox;

pub fn spawn(
    p: &mut ChildSpawnerCommands,
    assets: &PinballDefenseAssets,
    prop_i: usize,
    init_val: KeyCode,
) {
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    p.spawn_empty().queue_apply_scene(bsn! {
        #Key
        Keybox
        Button
        Node {
            width: Val::Px(130.),
            height: Val::Px(55.),
            border: UiRect::all(Val::Px(5.0)),
            margin: UiRect::all(Val::Auto),
            padding: UiRect::all(Val::Auto),
            display: Display::Flex,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
        }
        BorderColor::from(GameColor::GOLD)
        BackgroundColor({GRAY})
        PropIndex({prop_i})
        Active
        Children [
            (Text({format!("{init_val:?}").replace("Key", "")})
             TextFont { font: {font}, font_size: FontSize::Px(40.0) }
             TextColor({GameColor::WHITE}))
        ]
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
