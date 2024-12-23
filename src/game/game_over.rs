use super::GameState;
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::color::palettes::css::GOLD;

pub fn spawn(mut cmds: Commands, assets: Res<PinballDefenseAssets>) {
    cmds.spawn(container()).with_children(|p| {
        p.spawn(headline("GAME OVER", &assets));
        spawn_restart_btn(p, &assets);
    });
}

#[derive(Component)]
pub struct GameOverScreen;

fn container() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba_u8(23, 24, 26, 120).into()),
        GameOverScreen,
    )
}

fn headline(text: &str, assets: &PinballDefenseAssets) -> impl Bundle {
    (
        Text(text.to_string()),
        TextFont {
            font: assets.menu_font.clone(),
            font_size: 100.0,
            ..default()
        },
        TextColor(Color::srgb_u8(255, 254, 236)),
    )
}

// If more buttons needed, change this to an enum
#[derive(Component)]
pub struct ActionBtn;

fn spawn_restart_btn(p: &mut ChildBuilder, assets: &PinballDefenseAssets) {
    p.spawn((
        Name::new("Button"),
        Button::default(),
        Node {
            width: Val::Px(400.),
            height: Val::Px(65.),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(GOLD.into()),
        BackgroundColor(Color::NONE.into()),
        ActionBtn,
    ))
    .with_children(|p| {
        p.spawn((
            Text("New Game".to_string()),
            TextFont {
                font: assets.menu_font.clone(),
                font_size: 40.0,
                ..default()
            },
            TextColor(GameColor::WHITE),
        ));
    });
}

pub(super) fn btn_system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &ActionBtn),
        (Changed<Interaction>, With<ActionBtn>, With<ActionBtn>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut border_color, _action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => game_state.set(GameState::Init),
            Interaction::Hovered => {
                *border_color = GameColor::WHITE.into();
            }
            Interaction::None => {
                *border_color = GameColor::GOLD.into();
            }
        }
    }
}
