use super::MenuLayout;
use super::tools::sliders;
use super::tools::{checkbox, keybox, row, Focusable};
use crate::game::KeyboardControls;
use crate::prelude::*;
use bevy::text::{FontSize, FontSourceTemplate};
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigation;
use bevy::ui_widgets::ScrollArea;
use crate::settings::{GraphicsSettings, SoundSettings};
use crate::utils::reflect::{cast, prop_name};
use crate::utils::{GameColor, Music, Sound};
use bevy::audio::Volume;
use bevy::camera::Hdr;
use bevy::post_process::bloom::Bloom;
use bevy::reflect::structs::Struct;
use std::any::TypeId;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum SettingsMenuState {
    #[default]
    None,
    KeyboardControls,
    Sound,
    Graphics,
}

const KEY_CODE: &str = "bevy_input::keyboard::KeyCode";

fn header(p: &mut ChildSpawnerCommands, assets: &PinballDefenseAssets, text: &str) {
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    let text = text.to_string();
    p.spawn_empty().apply_scene(bsn! {
        Node {
            width: Val::Percent(100.),
            height: Val::Px(50.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::bottom(Val::Px(2.)),
        }
        BorderColor::from(GameColor::GOLD)
        Children [
            (Text({text})
             TextFont { font: {font}, font_size: FontSize::Px(40.0) }
             TextColor({GameColor::GOLD}))
        ]
    });
}

fn controller_bindings() -> [(&'static str, &'static str); 6] {
    [
        ("Flipper Left", "L2"),
        ("Flipper Right", "R2"),
        ("Start", "East"),
        ("Charge / Fire", "South"),
        ("Pause", "Start"),
        ("Menu", "Start"),
    ]
}

fn controller_label(p: &mut ChildSpawnerCommands, assets: &PinballDefenseAssets, text: &str) {
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    let text = text.to_string();
    p.spawn_empty().apply_scene(bsn! {
        Node {
            width: Val::Px(195.),
            height: Val::Px(55.),
            border: UiRect::all(Val::Px(5.0)),
            margin: UiRect::all(Val::Auto),
            padding: UiRect::all(Val::Auto),
            display: Display::Flex,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
        }
        Button
        AutoDirectionalNavigation
        Focusable
        BorderColor::from(GameColor::GOLD)
        BackgroundColor(Color::NONE)
        Children [
            (Text({text})
             TextFont { font: {font}, font_size: FontSize::Px(32.0) }
             TextColor({GameColor::WHITE}))
        ]
    });
}

pub fn layout<TSettings: Resource + Struct>(
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    settings: Res<TSettings>,
) {
    let scroll_area = cmds.spawn_scene(settings_menu_layout()).id();
    super::tools::scrollbar::spawn(&mut cmds, scroll_area);
    cmds.entity(scroll_area).with_children(|p| {
        let is_controls = TypeId::of::<TSettings>() == TypeId::of::<KeyboardControls>();
        if is_controls {
            header(p, &assets, "Keyboard");
        }
        for (i, (_, field)) in settings.iter_fields().enumerate() {
            let prop_name = prop_name(settings.as_ref(), i)
                .replace('_', " ")
                .replace("is", "");
            let field = field.try_as_reflect().expect("Can't cast as reflect");
            row::spawn(&prop_name, p, &assets, |p| {
                match field.reflect_type_path() {
                    "bool" => checkbox::spawn(p, i, cast::<bool>(field)),
                    "f32" => sliders::spawn(p, i, cast::<f32>(field)),
                    KEY_CODE => keybox::spawn(p, &assets, i, cast::<KeyCode>(field)),
                    type_name => println!("🐱 Unknown type in asset struct: {}", type_name),
                }
            })
        }
        if is_controls {
            header(p, &assets, "Controller");
            for (action, button) in controller_bindings() {
                row::spawn(action, p, &assets, |p| {
                    controller_label(p, &assets, button);
                });
            }
        }
    });
}

#[derive(Component, Clone, Default)]
pub struct SettingsMenuLayout;

fn settings_menu_layout() -> impl Scene {
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
        SettingsMenuLayout
        ScrollArea
    }
}

pub fn clean_up(mut cmds: Commands, q_sett_layout: Query<Entity, With<SettingsMenuLayout>>) {
    for layout_id in q_sett_layout.iter() {
        cmds.entity(layout_id).despawn();
    }
}

pub fn on_changed_sound_settings(
    mut cmds: Commands,
    sound_sett: Res<SoundSettings>,
    assets: Res<PinballDefenseAudioAssets>,
    mut q_sound: Query<&mut AudioSink, (With<Sound>, Without<Music>)>,
    mut q_music: Query<(Entity, &mut AudioSink), (With<Music>, Without<Sound>)>,
) {
    if sound_sett.is_changed() {
        for mut sound in q_sound.iter_mut() {
            sound.set_volume(Volume::Linear(sound_sett.fx_volume));
        }
        // music player only if music volume > 0
        if sound_sett.music_volume > 0. {
            if let Ok((_, mut music)) = q_music.single_mut() {
                music.set_volume(Volume::Linear(sound_sett.music_volume));
            } else {
                cmds.spawn((
                    AudioPlayer(assets.background_music.clone()),
                    PlaybackSettings::LOOP.with_volume(Volume::Linear(sound_sett.music_volume)),
                    Music,
                ));
            }
        } else {
            for (music, _) in q_music.iter() {
                cmds.entity(music).despawn();
            }
        }
    }
}

pub fn on_changed_graphics_settings(
    mut cmds: Commands,
    g_sett: Res<GraphicsSettings>,
    q_cam: Query<(Entity, Option<&Hdr>), With<Camera>>,
    mut q_spot: Query<&mut SpotLight>,
    mut q_point: Query<&mut PointLight>,
    mut q_bloom: Query<&mut Bloom>,
) {
    if g_sett.is_changed() {
        q_point
            .iter_mut()
            .for_each(|mut light| light.shadow_maps_enabled = g_sett.is_shadows);
        q_spot
            .iter_mut()
            .for_each(|mut light| light.shadow_maps_enabled = g_sett.is_shadows);

        if let Ok((id, hdr)) = q_cam.single() {
            if g_sett.is_hdr && hdr.is_none() {
                cmds.entity(id).insert(Hdr);
            } else if !g_sett.is_hdr && hdr.is_some() {
                cmds.entity(id).remove::<Hdr>();
            }
        }
        q_bloom
            .iter_mut()
            .for_each(|mut bloom_sett| bloom_sett.intensity = g_sett.bloom_intensity);
    }
}
