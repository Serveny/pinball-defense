use super::super::actions::MenuAction;
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::text::{FontSize, FontSourceTemplate};

#[derive(Component, Clone, Default)]
pub struct MenuButton;

#[derive(Component, Clone, Default)]
pub struct MenuButtonData {
    pub action: MenuAction,
    pub style: ButtonStyle,
}

#[derive(Component, Clone, Copy, Default)]
pub enum ButtonStyle {
    #[default]
    Primary,
    Secondary,
}

impl ButtonStyle {
    fn resting_color(&self) -> Color {
        match self {
            ButtonStyle::Primary => GameColor::GOLD,
            ButtonStyle::Secondary => GameColor::GRAY,
        }
    }
}

pub fn scene(
    action: MenuAction,
    style: ButtonStyle,
    assets: &PinballDefenseAssets,
    margin: UiRect,
) -> impl Scene {
    let label = action.label();
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    let color = style.resting_color();
    bsn! {
        #Button
        Button
        MenuButton
        MenuButtonData { action: {action}, style: {style} }
        Node {
            width: Val::Percent(100.),
            height: Val::Px(65.),
            border: UiRect::bottom(Val::Px(2.0)),
            margin: {margin},
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BorderColor::from(color)
        BackgroundColor(Color::NONE)
        Children [
            (Text({label})
             TextFont { font: {font}, font_size: FontSize::Px(40.0) }
             TextColor(color))
        ]
    }
}

pub fn system(
    children: Query<&Children>,
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &MenuButtonData, Entity),
        (Changed<Interaction>, With<Button>, With<MenuButton>),
    >,
    mut text_query: Query<&mut TextColor>,
    mut action_ev: MessageWriter<MenuAction>,
) {
    for (interaction, mut border_color, data, entity) in &mut interaction_query {
        let resting = data.style.resting_color();
        let target = match *interaction {
            Interaction::Pressed => {
                action_ev.write(data.action.clone());
                resting
            }
            Interaction::Hovered => GameColor::WHITE,
            Interaction::None => resting,
        };
        *border_color = target.into();
        if let Ok(children) = children.get(entity) {
            for child in children {
                if let Ok(mut text_color) = text_query.get_mut(*child) {
                    *text_color = target.into();
                }
            }
        }
    }
}
