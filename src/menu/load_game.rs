use super::utils::headline;
use super::{
    MenuLayout,
    actions::MenuAction,
    tools::menu_btn::{self, ButtonStyle, MenuButton, MenuButtonData},
};
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::scene::SceneScope;
use bevy::text::{FontSize, FontSourceTemplate};

#[derive(Component, Clone, Default)]
pub struct SaveList;

pub fn load_game_layout(mut cmds: Commands, assets: Res<PinballDefenseAssets>) {
    cmds.spawn_scene(nav_menu(&assets, "Load Game"));
    cmds.spawn_scene(save_list_panel(&assets, false));
}

pub fn save_game_layout(mut cmds: Commands, assets: Res<PinballDefenseAssets>) {
    cmds.spawn_scene(nav_menu(&assets, "Save Game"));
    cmds.spawn_scene(save_list_panel(&assets, true));
}

fn nav_menu(assets: &PinballDefenseAssets, title: &str) -> impl Scene {
    let headline = headline(title, 36.0, assets);
    let back_btn = menu_btn::scene(
        MenuAction::Back,
        ButtonStyle::Secondary,
        assets,
        UiRect::bottom(Val::Px(20.)),
    );
    bsn! {
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
        Children [
            ({headline}),
            (Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::horizontal(Val::Percent(5.)),
             }
             Children [
                 (Node { flex_grow: 1.0 }),
                 ({back_btn})
             ])
        ]
    }
}

fn save_list_panel(assets: &PinballDefenseAssets, show_new_save: bool) -> impl Scene {
    let bg: Color = Color::srgba_u8(23, 24, 26, 120);
    let entry: Box<dyn Scene> = if show_new_save {
        Box::new(SceneScope(save_entry(
            "New Save",
            MenuAction::NewGame,
            assets,
        )))
    } else {
        Box::new(SceneScope(empty_hint_scene(assets)))
    };
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(300.),
            right: Val::Px(0.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_content: AlignContent::FlexStart,
        }
        BackgroundColor({bg})
        MenuLayout
        SaveList
        Children [({entry})]
    }
}

fn save_entry(label: &str, action: MenuAction, assets: &PinballDefenseAssets) -> impl Scene {
    let label = label.to_string();
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    let style = ButtonStyle::Primary;
    let color = GameColor::GOLD;
    bsn! {
        #Button
        Button
        MenuButton
        MenuButtonData { action: {action}, style: {style} }
        Node {
            width: Val::Percent(100.),
            height: Val::Px(65.),
            border: UiRect::bottom(Val::Px(2.0)),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            padding: UiRect::horizontal(Val::Px(20.)),
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

fn empty_hint_scene(assets: &PinballDefenseAssets) -> impl Scene {
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    bsn! {
        Node {
            width: Val::Percent(100.),
            height: Val::Px(65.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        Children [
            (Text("No save games available.")
             TextFont { font: {font}, font_size: FontSize::Px(28.0) }
             TextColor(Color::srgb_u8(200, 200, 200)))
        ]
    }
}
