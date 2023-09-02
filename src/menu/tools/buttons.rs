use super::super::actions::MenuAction;
use crate::menu::{GOLD, WHITE};
use crate::prelude::*;

#[derive(Component)]
struct MenuButton;

pub fn spawn_menu_button(
    action: MenuAction,
    p: &mut ChildBuilder,
    assets: &PinballDefenseAssets,
    margin: UiRect,
) {
    p.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Px(65.),
                border: UiRect::bottom(Val::Px(2.0)),
                margin,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: Color::GOLD.into(),
            background_color: Color::NONE.into(),
            ..default()
        },
        action,
        MenuButton,
    ))
    .with_children(|p| {
        p.spawn(TextBundle::from_section(
            action.to_string(),
            TextStyle {
                font: assets.menu_font.clone(),
                font_size: 40.0,
                color: Color::rgb_u8(255, 254, 236),
            },
        ));
    });
}

#[allow(clippy::type_complexity)]
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &MenuAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut action_ev: EventWriter<MenuAction>,
) {
    for (interaction, mut border_color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => action_ev.send(*action),
            Interaction::Hovered => {
                *border_color = WHITE.into();
            }
            Interaction::None => {
                *border_color = GOLD.into();
            }
        }
    }
}
