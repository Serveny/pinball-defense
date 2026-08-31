use super::GameState;
use super::level::{LevelHub, PointHub};
use super::stats::GameStats;
use crate::AppState;
use crate::menu::MenuAction;
use crate::menu::tools::menu_btn::{self, ButtonStyle};
use crate::prelude::*;
use crate::utils::GameColor;
use bevy::text::{FontSize, FontSourceTemplate};

const BTN_MAX_WIDTH: f32 = 500.0;

pub fn spawn(
    mut cmds: Commands,
    assets: Res<PinballDefenseAssets>,
    stats: Res<GameStats>,
    point_hub: Res<PointHub>,
    level_hub: Res<LevelHub>,
) {
    cmds.spawn_scene(bsn! {
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BackgroundColor({Color::srgba_u8(23, 24, 26, 120)})
        GameOverScreen
    })
    .with_children(|p| {
        spawn_headline("GAME OVER", p, &assets);
        spawn_stats(p, &assets, &stats, point_hub.0, level_hub.level());
        spawn_buttons(p, &assets);
    });
}

#[derive(Component, Clone, Default)]
pub struct GameOverScreen;

fn spawn_headline(text: &str, p: &mut ChildSpawnerCommands, assets: &PinballDefenseAssets) {
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    p.spawn_empty().queue_apply_scene(bsn! {
        Text({text})
        TextFont { font: {font}, font_size: FontSize::Px(100.0) }
        TextColor(Color::srgb_u8(255, 254, 236))
    });
}

fn spawn_stats(
    p: &mut ChildSpawnerCommands,
    assets: &PinballDefenseAssets,
    stats: &GameStats,
    points: u32,
    level: u8,
) {
    let rows: [(&str, String); 6] = [
        ("Score", format!("{points}")),
        ("Level", format!("{level}")),
        ("Wave", format!("{}", stats.wave_number)),
        ("Damage Dealt", format!("{:.0}", stats.damage_dealt)),
        ("Towers Built", format!("{}", stats.towers_built)),
        ("Upgrades", format!("{}", stats.upgrades_performed)),
    ];
    p.spawn_empty()
        .queue_apply_scene(bsn! {
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                margin: UiRect::vertical(Val::Px(40.0)),
                align_items: AlignItems::Stretch,
            }
        })
        .with_children(|p| {
            for (label, value) in rows {
                let label = label.to_string();
                let value = value.clone();
                let label_font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
                let value_font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
                p.spawn_empty().queue_apply_scene(bsn! {
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        column_gap: Val::Px(60.0),
                    }
                    Children [
                        (Text({label})
                         TextFont { font: {label_font}, font_size: FontSize::Px(30.0) }
                         TextColor(GameColor::GRAY)),
                        (Text({value})
                         TextFont { font: {value_font}, font_size: FontSize::Px(30.0) }
                         TextColor(GameColor::WHITE))
                    ]
                });
            }
        });
}

fn spawn_buttons(p: &mut ChildSpawnerCommands, assets: &PinballDefenseAssets) {
    let margin = UiRect::vertical(Val::Px(10.0));
    p.spawn_empty()
        .queue_apply_scene(bsn! {
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                max_width: Val::Px({BTN_MAX_WIDTH}),
            }
        })
        .with_children(|p| {
            p.spawn_empty().queue_apply_scene(menu_btn::scene(
                MenuAction::NewGame,
                ButtonStyle::Primary,
                assets,
                margin,
            ));
            p.spawn_empty().queue_apply_scene(menu_btn::scene(
                MenuAction::LoadGame,
                ButtonStyle::Primary,
                assets,
                margin,
            ));
            p.spawn_empty().queue_apply_scene(menu_btn::scene(
                MenuAction::BackToMainMenu,
                ButtonStyle::Secondary,
                assets,
                margin,
            ));
        });
}

pub(super) fn action_handler(
    mut cmds: Commands,
    mut evr: MessageReader<MenuAction>,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for action in evr.read() {
        match action {
            MenuAction::NewGame => game_state.set(GameState::Init),
            MenuAction::BackToMainMenu => app_state.set(AppState::MainMenu),
            MenuAction::LoadGame => {
                cmds.insert_resource(crate::menu::GoToLoadGame);
                app_state.set(AppState::MainMenu);
            }
            _ => {}
        }
    }
}
