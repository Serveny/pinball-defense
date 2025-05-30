use super::tools::{checkbox, keybox, row};
use super::{tools::sliders, MenuLayout};
use crate::prelude::*;
use crate::settings::{GraphicsSettings, SoundSettings};
use crate::utils::reflect::{cast, prop_name};
use crate::utils::{Music, Sound};
use bevy::audio::Volume;
use bevy::core_pipeline::bloom::Bloom;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum SettingsMenuState {
    #[default]
    None,
    KeyboardControls,
    Sound,
    Graphics,
}

const KEY_CODE: &str = "bevy_input::keyboard::KeyCode";

pub fn layout<TSettings: Resource + Struct>(
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    settings: Res<TSettings>,
) {
    cmds.spawn(settings_menu_layout()).with_children(|p| {
        for (i, field) in settings.iter_fields().enumerate() {
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
    });
}

#[derive(Component)]
pub struct SettingsMenuLayout;

fn settings_menu_layout() -> impl Bundle {
    (
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(300.),
            right: Val::Px(0.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_content: AlignContent::FlexStart,
            ..default()
        },
        BackgroundColor(Color::srgba_u8(23, 24, 26, 120).into()),
        MenuLayout,
        SettingsMenuLayout,
    )
}

pub fn clean_up(mut cmds: Commands, q_sett_layout: Query<Entity, With<SettingsMenuLayout>>) {
    for layout_id in q_sett_layout.iter() {
        cmds.entity(layout_id).despawn();
    }
}

pub fn on_changed_sound_settings(
    sound_sett: Res<SoundSettings>,
    mut q_sound: Query<&mut AudioSink, (With<Sound>, Without<Music>)>,
    mut q_music: Query<&mut AudioSink, (With<Music>, Without<Sound>)>,
) {
    if sound_sett.is_changed() {
        for mut sound in q_sound.iter_mut() {
            sound.set_volume(Volume::Linear(sound_sett.fx_volume));
        }
        for mut music in q_music.iter_mut() {
            music.set_volume(Volume::Linear(sound_sett.music_volume));
        }
    }
}

pub fn on_changed_graphics_settings(
    g_sett: Res<GraphicsSettings>,
    mut q_spot: Query<&mut SpotLight>,
    mut q_point: Query<&mut PointLight>,
    mut q_cam: Query<&mut Camera>,
    mut q_bloom: Query<&mut Bloom>,
) {
    if g_sett.is_changed() {
        q_point
            .iter_mut()
            .for_each(|mut light| light.shadows_enabled = g_sett.is_shadows);
        q_spot
            .iter_mut()
            .for_each(|mut light| light.shadows_enabled = g_sett.is_shadows);
        q_cam.iter_mut().for_each(|mut cam| cam.hdr = g_sett.is_hdr);
        q_bloom
            .iter_mut()
            .for_each(|mut bloom_sett| bloom_sett.intensity = g_sett.bloom_intensity);
    }
}
