use super::tools::{checkbox, keybox, row};
use super::{tools::sliders, MenuLayout};
use crate::prelude::*;
use crate::utils::reflect::prop_name;

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
            row::spawn(&prop_name, p, &assets, |p| match field.type_name() {
                "bool" => checkbox::spawn(p, i, cast::<bool>(field)),
                "f32" => sliders::spawn(p, i, cast::<f32>(field)),
                KEY_CODE => keybox::spawn(p, &assets, i, cast::<KeyCode>(field)),
                type_name => println!("🐱 Unknown type in asset struct: {}", type_name),
            })
        }
    });
}

fn cast<T: Reflect + Copy>(field: &dyn Reflect) -> T {
    *field
        .downcast_ref::<T>()
        .unwrap_or_else(|| panic!("😥 Can't downcast to {}", field.type_name()))
}

#[derive(Component)]
pub struct SettingsMenuLayout;

fn settings_menu_layout() -> impl Bundle {
    (
        NodeBundle {
            background_color: Color::rgba_u8(23, 24, 26, 120).into(),
            style: Style {
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
            ..default()
        },
        MenuLayout,
        SettingsMenuLayout,
    )
}

pub fn clean_up(mut cmds: Commands, q_sett_layout: Query<Entity, With<SettingsMenuLayout>>) {
    for layout_id in q_sett_layout.iter() {
        cmds.entity(layout_id).despawn_recursive();
    }
}