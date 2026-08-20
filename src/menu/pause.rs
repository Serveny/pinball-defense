use super::{MenuLayout, actions::MenuAction, tools::menu_btn};
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::text::{FontSize, FontSourceTemplate};

pub fn layout(mut cmds: Commands, assets: Res<PinballDefenseAssets>) {
    cmds.spawn_scene(bsn! {
        Node {
            display: Display::Grid,
            width: Val::Percent(100.),
            max_width: Val::Px(300.),
            height: Val::Percent(100.),
            grid_template_rows: vec![GridTrack::px(80.), GridTrack::auto()],
            align_content: AlignContent::Stretch,
        }
        BackgroundColor({GameColor::BACKGROUND})
        MenuLayout
    })
    .with_children(|p| {
        spawn_headline("Pause", p, &assets);
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
             TextFont { font: {font}, font_size: FontSize::Px(80.0) }
             TextColor(Color::srgb_u8(255, 254, 236)))
        ]
    });
}

fn spawn_buttons(p: &mut ChildSpawnerCommands, assets: &PinballDefenseAssets) {
    p.spawn((Node {
        display: Display::Flex,
        align_items: AlignItems::Center,
        align_content: AlignContent::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        flex_wrap: FlexWrap::NoWrap,
        row_gap: Val::Percent(5.),
        padding: UiRect::horizontal(Val::Percent(5.)),
        ..default()
    },))
        .with_children(|p| {
            let margin = UiRect::default();
            let con_margin = UiRect::bottom(Val::Px(10.));
            menu_btn::spawn(MenuAction::Continue, p, assets, con_margin);
            menu_btn::spawn(MenuAction::Controls, p, assets, margin);
            menu_btn::spawn(MenuAction::Graphics, p, assets, margin);
            menu_btn::spawn(MenuAction::Sound, p, assets, margin);
            menu_btn::spawn(MenuAction::Quit, p, assets, UiRect::top(Val::Px(10.)));
        });
}
