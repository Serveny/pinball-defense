use crate::menu::{BACKGROUND, GRAY, WHITE};
use crate::prelude::*;

#[derive(Component)]
pub struct Row;

pub fn row<F: FnOnce(&mut ChildBuilder)>(
    text: &str,
    p: &mut ChildBuilder,
    assets: &PinballDefenseAssets,
    spawn_inside: F,
) {
    p.spawn((
        NodeBundle {
            background_color: BACKGROUND.into(),
            border_color: GRAY.into(),
            style: Style {
                display: Display::Grid,
                grid_template_columns: vec![
                    GridTrack::px(300.),
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
        p.spawn(NodeBundle { ..default() }).with_children(|p| {
            p.spawn(TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: assets.menu_font.clone(),
                        font_size: 40.0,
                        color: WHITE,
                    },
                ),
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                ..default()
            });
        });

        spawn_inside(p);
    });
}
