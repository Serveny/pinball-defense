use super::{actions::MenuAction, tools::buttons::spawn_menu_button, MenuLayout, BACKGROUND};
use crate::prelude::*;

pub fn layout(mut cmds: Commands, assets: Res<PinballDefenseAssets>) {
    cmds.spawn((
        NodeBundle {
            background_color: BACKGROUND.into(),
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.),
                max_width: Val::Px(300.),
                height: Val::Percent(100.),
                grid_template_rows: vec![GridTrack::px(80.), GridTrack::auto()],
                align_content: AlignContent::Stretch,
                ..default()
            },
            ..default()
        },
        MenuLayout,
    ))
    .with_children(|p| {
        spawn_headline("Pause", p, &assets);
        spawn_buttons(p, &assets);
    });
}

fn spawn_headline(text: &str, p: &mut ChildBuilder, assets: &PinballDefenseAssets) {
    p.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    },))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: assets.menu_font.clone(),
                    font_size: 80.0,
                    color: Color::rgb_u8(255, 254, 236),
                },
            ));
        });
}

fn spawn_buttons(p: &mut ChildBuilder, assets: &PinballDefenseAssets) {
    p.spawn((NodeBundle {
        style: Style {
            display: Display::Flex,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            flex_wrap: FlexWrap::NoWrap,
            row_gap: Val::Percent(5.),
            padding: UiRect::horizontal(Val::Percent(5.)),
            ..default()
        },
        ..default()
    },))
        .with_children(|p| {
            let margin = UiRect::default();
            let con_margin = UiRect::bottom(Val::Px(10.));
            spawn_menu_button(MenuAction::Continue, p, assets, con_margin);
            spawn_menu_button(MenuAction::Controls, p, assets, margin);
            spawn_menu_button(MenuAction::Graphics, p, assets, margin);
            spawn_menu_button(MenuAction::Sound, p, assets, margin);
            spawn_menu_button(MenuAction::Quit, p, assets, UiRect::top(Val::Px(10.)));
        });
}
