use super::GameState;
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::color::palettes::css::GOLD;
use bevy::text::{FontSize, FontSourceTemplate};

pub fn spawn(mut cmds: Commands, assets: Res<PinballDefenseAssets>) {
    cmds.spawn_scene(bsn! {
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BackgroundColor({Color::srgba_u8(23, 24, 26, 120)})
        GameOverScreen
    })
    .with_children(|p| {
        spawn_headline("GAME OVER", p, &assets);
        spawn_restart_btn(p, &assets);
    });
}

#[derive(Component, Clone, Default)]
pub struct GameOverScreen;

fn spawn_headline(text: &str, p: &mut ChildSpawnerCommands, assets: &PinballDefenseAssets) {
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    p.spawn_empty().queue_apply_scene(bsn! {
        Text({text})
        TextFont { font: {font}, font_size: FontSize::Px(100.0) }
        TextColor(Color::srgb_u8(255, 254, 236))
    });
}

// If more buttons needed, change this to an enum
#[derive(Component, Clone, Default)]
pub struct ActionBtn;

fn spawn_restart_btn(p: &mut ChildSpawnerCommands, assets: &PinballDefenseAssets) {
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    p.spawn_empty().queue_apply_scene(bsn! {
        #Button
        Button
        ActionBtn
        Node {
            width: Val::Px(400.),
            height: Val::Px(65.),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BorderColor::from(GOLD)
        BackgroundColor({Color::NONE})
        Children [
            (Text("New Game")
             TextFont { font: {font}, font_size: FontSize::Px(40.0) }
             TextColor({GameColor::WHITE}))
        ]
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
