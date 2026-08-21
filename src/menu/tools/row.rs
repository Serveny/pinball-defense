use crate::prelude::*;
use crate::utils::GameColor;
use bevy::text::{FontSize, FontSourceTemplate};

#[derive(Component, Clone, Default)]
pub struct Row;

pub fn spawn(
    text: &str,
    p: &mut ChildSpawnerCommands,
    assets: &PinballDefenseAssets,
    spawn_inside: impl FnOnce(&mut ChildSpawnerCommands),
) {
    let font = FontSourceTemplate::Handle(assets.menu_font.clone().into());
    let text = text.to_string();
    let mut row = p.spawn_empty();
    row.apply_scene(bsn! {
        Name::new("UI Row")
        Row
        Node {
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::px(400.), GridTrack::flex(1.), GridTrack::px(20.)],
            align_items: AlignItems::Stretch,
            border: UiRect::bottom(Val::Px(2.)),
            height: Val::Px(65.),
            width: Val::Percent(100.),
        }
        BorderColor::from(GameColor::GRAY)
        BackgroundColor(GameColor::BACKGROUND)
        Children [
            (Node {}
             Children [
                 (Text({text})
                  TextFont { font: {font}, font_size: FontSize::Px(40.0) }
                  TextColor({row_text_color(true)})
                  Node { margin: UiRect::all(Val::Auto) })
             ])
        ]
    });
    row.with_children(|p| {
        p.spawn(Node {
            width: Val::Percent(100.),
            ..default()
        })
        .with_children(spawn_inside);
    });
}

fn row_text_color(is_active: bool) -> Color {
    match is_active {
        true => GameColor::WHITE,
        false => GameColor::WHITE.with_alpha(0.5),
    }
}
