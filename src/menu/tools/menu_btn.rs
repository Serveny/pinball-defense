use super::super::actions::MenuAction;
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::color::palettes::css::GOLD;
use bevy::text::{FontSize, FontSourceTemplate};

#[derive(Component)]
pub struct MenuButton;

pub fn spawn(
    action: MenuAction,
    spawner: &mut ChildSpawnerCommands,
    assets: &PinballDefenseAssets,
    margin: UiRect,
) {
    let label = action.to_string();
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    spawner
        .spawn_empty()
        .insert((MenuButton, action))
        .queue_apply_scene(bsn! {
            #Button
            Button
            Node {
                width: Val::Percent(100.),
                height: Val::Px(65.),
                border: UiRect::bottom(Val::Px(2.0)),
                margin: {margin},
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
            }
            BorderColor::from(GOLD)
            BackgroundColor({Color::NONE})
            Children [
                (Text({label})
                 TextFont { font: {font}, font_size: FontSize::Px(40.0) }
                 TextColor({GameColor::WHITE}))
            ]
        });
}

pub fn system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &MenuAction),
        (Changed<Interaction>, With<Button>, With<MenuButton>),
    >,
    mut action_ev: MessageWriter<MenuAction>,
) {
    for (interaction, mut border_color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                action_ev.write(*action);
            }
            Interaction::Hovered => {
                *border_color = GameColor::WHITE.into();
            }
            Interaction::None => {
                *border_color = GameColor::GOLD.into();
            }
        }
    }
}
