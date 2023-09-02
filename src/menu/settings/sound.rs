use super::super::{
    actions::{MenuAction, SoundAction},
    tools::sliders::spawn_slider,
    MenuLayout,
};
use crate::prelude::*;
use crate::{menu::tools::row::row, settings::SoundSettings};

pub fn layout(mut cmds: Commands, assets: Res<PinballDefenseAssets>, settings: Res<SoundSettings>) {
    cmds.spawn((
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
    ))
    .with_children(|p| {
        row("FX Volume", p, &assets, |p| {
            let action = MenuAction::EditSound(SoundAction::FxVolume(settings.fx_volume));
            spawn_slider(action, p);
        });
        row("Music Volume", p, &assets, |p| {
            let action = MenuAction::EditSound(SoundAction::MusicVolume(settings.music_volume));
            spawn_slider(action, p);
        });
    });
}
