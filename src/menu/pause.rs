use super::utils::headline;
use super::{MenuLayout, actions::MenuAction, tools::menu_btn, tools::menu_btn::ButtonStyle};
use crate::prelude::*;
use crate::utils::GameColor;

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
        Children [
            (headline("Pause", 80.0, &assets)),
            (Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                flex_wrap: FlexWrap::NoWrap,
                row_gap: Val::Percent(5.),
                padding: UiRect::horizontal(Val::Percent(5.)),
             }
             Children [
                 (menu_btn::scene(MenuAction::Continue, ButtonStyle::Primary, &assets, UiRect::bottom(Val::Px(10.)))),
                 (menu_btn::scene(MenuAction::SaveGame, ButtonStyle::Primary, &assets, UiRect::default())),
                 (menu_btn::scene(MenuAction::Controls, ButtonStyle::Primary, &assets, UiRect::default())),
                 (menu_btn::scene(MenuAction::Graphics, ButtonStyle::Primary, &assets, UiRect::default())),
                 (menu_btn::scene(MenuAction::Sound, ButtonStyle::Primary, &assets, UiRect::default())),
                 (menu_btn::scene(MenuAction::Quit, ButtonStyle::Primary, &assets, UiRect::top(Val::Px(10.)))),
             ])
        ]
    });
}
