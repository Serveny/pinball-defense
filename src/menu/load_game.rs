use super::utils::headline;
use super::{
    MenuLayout,
    actions::MenuAction,
    tools::Focusable,
    tools::menu_btn::{self, ButtonStyle, MenuButton, MenuButtonData},
};
use crate::game::{list_saves, next_save_path, thumbnail_path};
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::text::{FontSize, FontSourceTemplate};
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigation;
use bevy::ui_widgets::ScrollArea;
use std::path::Path;

#[derive(Component, Clone, Default)]
pub struct SaveList;

pub fn load_game_layout(
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    mut image_assets: ResMut<Assets<Image>>,
) {
    cmds.spawn_scene(nav_menu(&assets, "Load Game"));
    let scroll_area = cmds.spawn_scene(save_list_panel()).id();
    super::tools::scrollbar::spawn(&mut cmds, scroll_area);
    cmds.entity(scroll_area).with_children(|p| {
        spawn_save_entries(p, &assets, &mut image_assets, MenuAction::Load);
        if list_saves().is_empty() {
            p.spawn_empty()
                .apply_scene(empty_message("No saves found", &assets));
        }
    });
}

pub fn save_game_layout(
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    mut image_assets: ResMut<Assets<Image>>,
) {
    cmds.spawn_scene(nav_menu(&assets, "Save Game"));
    let scroll_area = cmds.spawn_scene(save_list_panel()).id();
    super::tools::scrollbar::spawn(&mut cmds, scroll_area);
    cmds.entity(scroll_area).with_children(|p| {
        spawn_save_entries(p, &assets, &mut image_assets, MenuAction::Save);
        p.spawn_empty().apply_scene(save_entry(
            "New Save",
            None,
            MenuAction::Save(next_save_path()),
            &assets,
        ));
    });
}

fn spawn_save_entries(
    p: &mut ChildSpawnerCommands,
    assets: &PinballDefenseAssets,
    image_assets: &mut Assets<Image>,
    to_action: impl Fn(String) -> MenuAction,
) {
    for path in list_saves() {
        let label = save_label(&path);
        let action = to_action(path.to_string_lossy().into_owned());
        let thumb = load_thumbnail(&thumbnail_path(&path), image_assets);
        p.spawn_empty()
            .apply_scene(save_entry(&label, thumb, action, assets));
    }
}

fn load_thumbnail(path: &Path, image_assets: &mut Assets<Image>) -> Option<Handle<Image>> {
    let bytes = std::fs::read(path).ok()?;
    let dyn_img = image::ImageReader::new(std::io::Cursor::new(&bytes))
        .with_guessed_format()
        .ok()?
        .decode()
        .ok()?;
    let rgba = dyn_img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let image = Image::new(
        Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba.into_raw(),
        TextureFormat::Rgba8UnormSrgb,
        bevy::asset::RenderAssetUsages::default(),
    );
    Some(image_assets.add(image))
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

fn save_list_panel() -> impl Scene {
    let bg: Color = Color::srgba_u8(23, 24, 26, 120);
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(300.),
            right: Val::Px(0.),
            top: Val::Px(0.),
            bottom: Val::Px(0.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_content: AlignContent::FlexStart,
            overflow: Overflow::scroll_y(),
        }
        BackgroundColor({bg})
        MenuLayout
        SaveList
        ScrollArea
    }
}

fn save_label(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Save")
        .to_string()
}

fn empty_message(text: &str, assets: &PinballDefenseAssets) -> impl Scene {
    let text = text.to_string();
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    bsn! {
        Node {
            width: Val::Percent(100.),
            height: Val::Px(65.),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            padding: UiRect::horizontal(Val::Px(20.)),
        }
        Children [
            (Text({text})
             TextFont { font: {font}, font_size: FontSize::Px(40.0) }
             TextColor(GameColor::GRAY))
        ]
    }
}

fn save_entry(
    label: &str,
    thumb: Option<Handle<Image>>,
    action: MenuAction,
    assets: &PinballDefenseAssets,
) -> impl Scene {
    let label = label.to_string();
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    let style = ButtonStyle::Primary;
    let color = GameColor::GOLD;
    let thumb = thumb.unwrap_or_default();
    bsn! {
        #Button
        Button
        MenuButton
        MenuButtonData { action: {action}, style: {style} }
        AutoDirectionalNavigation
        Focusable
        Node {
            width: Val::Percent(100.),
            height: Val::Px(65.),
            border: UiRect::bottom(Val::Px(2.0)),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            padding: UiRect::horizontal(Val::Px(20.)),
            column_gap: Val::Px(15.),
        }
        BorderColor::from(color)
        BackgroundColor(Color::NONE)
        Children [
            (Node {
                width: Val::Px(110.),
                height: Val::Px(55.),
                flex_shrink: 0.0,
             }
             ImageNode { image: {thumb} }),
            (Text({label})
             TextFont { font: {font}, font_size: FontSize::Px(40.0) }
             TextColor(color))
        ]
    }
}
