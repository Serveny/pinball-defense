use super::super::actions::MenuAction;
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::color::palettes::css::GOLD;

#[derive(Component)]
pub struct MenuButton;

pub fn spawn(
    action: MenuAction,
    spawner: &mut ChildSpawnerCommands,
    assets: &PinballDefenseAssets,
    margin: UiRect,
) {
    spawner
        .spawn((
            Name::new("Button"),
            MenuButton,
            action,
            Button::default(),
            Node {
                width: Val::Percent(100.),
                height: Val::Px(65.),
                border: UiRect::bottom(Val::Px(2.0)),
                margin,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(GOLD.into()),
            BackgroundColor(Color::NONE.into()),
        ))
        .with_children(|spawner| {
            spawner.spawn((
                Text(action.to_string()),
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
        (&Interaction, &mut BorderColor, &MenuAction),
        (Changed<Interaction>, With<Button>, With<MenuButton>),
    >,
    mut action_ev: EventWriter<MenuAction>,
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
