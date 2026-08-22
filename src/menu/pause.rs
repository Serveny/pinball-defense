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
            grid_template_rows: vec![GridTrack::px(80.), GridTrack::fr(1.)],
            align_content: AlignContent::Stretch,
        }
        BackgroundColor({GameColor::BACKGROUND})
        MenuLayout
        Children [
            (headline("Pause", 80.0, &assets)),
            (Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::horizontal(Val::Percent(5.)),
             }
             Children [
                 (Node {
                     display: Display::Flex,
                     flex_direction: FlexDirection::Column,
                     align_items: AlignItems::Center,
                     justify_content: JustifyContent::Center,
                     row_gap: Val::Percent(5.),
                     flex_grow: 1.0,
                  }
                  Children [
                     (menu_btn::scene(MenuAction::Continue, ButtonStyle::Primary, &assets, UiRect::bottom(Val::Px(10.)))),
                     (menu_btn::scene(MenuAction::SaveGame, ButtonStyle::Primary, &assets, UiRect::default())),
                     (menu_btn::scene(MenuAction::Settings, ButtonStyle::Primary, &assets, UiRect::default())),
                  ]),
                 (menu_btn::scene(MenuAction::BackToMainMenu, ButtonStyle::Secondary, &assets, UiRect::bottom(Val::Px(20.))))
             ])
        ]
    });
}
