use super::tools::{checkbox, row};
use super::{tools::sliders, MenuLayout};
use crate::prelude::*;
use crate::utils::reflect::prop_name;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum SettingsMenuState {
    #[default]
    None,
    Controls,
    Sound,
    Graphics,
}

pub fn layout<TSettings: Resource + Struct>(
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    settings: Res<TSettings>,
) {
    cmds.spawn(settings_menu_layout()).with_children(|p| {
        for (i, field) in settings.iter_fields().enumerate() {
            let prop_name = prop_name(settings.as_ref(), i);
            row::spawn(&prop_name, p, &assets, |p| match field.type_name() {
                "bool" => {
                    let val = *field
                        .downcast_ref::<bool>()
                        .expect("😥 Can't downcast to bool");
                    checkbox::spawn(p, i, val);
                }
                "f32" => {
                    let val = *field
                        .downcast_ref::<f32>()
                        .expect("😥 Can't downcast to f32");
                    sliders::spawn(p, i, val);
                }
                type_name => println!("🐱 Unknown type in asset struct: {}", type_name),
            })
        }
    });
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
