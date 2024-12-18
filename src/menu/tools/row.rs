use crate::prelude::*;
use crate::utils::GameColor;

#[derive(Component)]
pub struct Row;

pub fn spawn(
    text: &str,
    p: &mut ChildBuilder,
    assets: &PinballDefenseAssets,
    spawn_inside: impl FnOnce(&mut ChildBuilder),
) {
    p.spawn((
        Name::new("UI Row"),
        NodeBundle {
            background_color: GameColor::BACKGROUND.into(),
            border_color: GameColor::GRAY.into(),
            style: Style {
                display: Display::Grid,
                grid_template_columns: vec![
                    GridTrack::px(400.),
                    GridTrack::auto(),
                    GridTrack::px(20.),
                ],
                align_items: AlignItems::Stretch,
                border: UiRect::bottom(Val::Px(2.)),
                height: Val::Px(65.),
                ..default()
            },
            ..default()
        },
        Row,
    ))
    .with_children(|p| {
        p.spawn(NodeBundle::default()).with_children(|p| {
            p.spawn(TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: assets.menu_font.clone(),
                        font_size: 40.0,
                        color: row_text_color(true),
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                ..default()
            });
        });
        p.spawn(NodeBundle::default()).with_children(spawn_inside);
    });
}

fn row_text_color(is_active: bool) -> Color {
    match is_active {
        true => GameColor::WHITE,
        false => GameColor::WHITE.with_alpha(0.5),
    }
}
