use super::{MenuLayout, actions::MenuAction, tools::menu_btn, tools::menu_btn::ButtonStyle};
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::text::{FontSize, FontSourceTemplate};

pub fn layout(mut cmds: Commands, assets: Res<PinballDefenseAssets>) {
    cmds.spawn((Camera2d, MenuLayout));
    cmds.spawn_scene(bsn! {
        Node {
            display: Display::Grid,
            width: Val::Percent(100.),
            max_width: Val::Px(300.),
            height: Val::Percent(100.),
            grid_template_rows: vec![GridTrack::px(50.), GridTrack::fr(1.)],
            align_content: AlignContent::Stretch,
        }
        BackgroundColor({GameColor::BACKGROUND})
        MenuLayout
    })
    .with_children(|p| {
        spawn_headline("Pinball Defense", p, &assets);
        spawn_buttons(p, &assets);
    });
}

fn spawn_headline(text: &str, p: &mut ChildSpawnerCommands, assets: &PinballDefenseAssets) {
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    p.spawn_empty().queue_apply_scene(bsn! {
        Node {
            width: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        Children [
            (Text({text})
             TextFont { font: {font}, font_size: FontSize::Px(36.0) }
             TextColor(Color::srgb_u8(255, 254, 236)))
        ]
    });
}

fn spawn_buttons(p: &mut ChildSpawnerCommands, assets: &PinballDefenseAssets) {
    p.spawn((Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        padding: UiRect::horizontal(Val::Percent(5.)),
        ..default()
    },))
        .with_children(|p| {
            p.spawn((Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Percent(5.),
                flex_grow: 1.0,
                ..default()
            },))
                .with_children(|p| {
                    let margin = UiRect::default();
                    menu_btn::spawn(
                        MenuAction::NewGame,
                        ButtonStyle::Primary,
                        p,
                        assets,
                        UiRect::bottom(Val::Px(10.)),
                    );
                    menu_btn::spawn(
                        MenuAction::LoadGame,
                        ButtonStyle::Primary,
                        p,
                        assets,
                        margin,
                    );
                    menu_btn::spawn(
                        MenuAction::Settings,
                        ButtonStyle::Primary,
                        p,
                        assets,
                        margin,
                    );
                });
            menu_btn::spawn(
                MenuAction::Quit,
                ButtonStyle::Secondary,
                p,
                assets,
                UiRect::bottom(Val::Px(20.)),
            );
        });
}
