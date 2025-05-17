use crate::prelude::*;
use crate::utils::GameColor;

#[derive(Component)]
pub struct Row;

pub fn spawn(
    text: &str,
    p: &mut ChildSpawnerCommands,
    assets: &PinballDefenseAssets,
    spawn_inside: impl FnOnce(&mut ChildSpawnerCommands),
) {
    p.spawn((
        Name::new("UI Row"),
        Node {
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::px(400.), GridTrack::auto(), GridTrack::px(20.)],
            align_items: AlignItems::Stretch,
            border: UiRect::bottom(Val::Px(2.)),
            height: Val::Px(65.),
            ..default()
        },
        BorderColor(GameColor::GRAY),
        BackgroundColor(GameColor::BACKGROUND),
        Row,
    ))
    .with_children(|p| {
        p.spawn(Node::default()).with_children(|p| {
            p.spawn((
                Text(text.to_string()),
                TextFont {
                    font: assets.menu_font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(row_text_color(true)),
                Node {
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
            ));
        });
        p.spawn(Node::default()).with_children(spawn_inside);
    });
}

fn row_text_color(is_active: bool) -> Color {
    match is_active {
        true => GameColor::WHITE,
        false => GameColor::WHITE.with_alpha(0.5),
    }
}
