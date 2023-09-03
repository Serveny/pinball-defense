use super::MenuLayout;
use crate::prelude::*;

pub mod graphics;
pub mod sound;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum SettingsMenuState {
    #[default]
    None,
    Controls,
    Sound,
    Graphics,
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
