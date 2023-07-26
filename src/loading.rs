use crate::prelude::*;
use crate::GameState;

pub struct LoadingScreenPlugin;

#[derive(Component)]
struct LoadingLayout;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), spawn_layout)
            .add_systems(OnExit(GameState::Loading), despawn_layout);
    }
}

fn despawn_layout(mut cmds: Commands, q_layout: Query<Entity, With<LoadingLayout>>) {
    for entity in q_layout.iter() {
        cmds.get_entity(entity).unwrap().despawn_recursive();
    }
}

fn spawn_layout(mut cmds: Commands) {
    println!("Loading Sceen 🤤");
    cmds.spawn((Camera2dBundle::default(), LoadingLayout));
    cmds.spawn((
        NodeBundle {
            style: Style {
                border: UiRect::percent(10., 0., 25., 25.),
                ..default()
            },
            ..default()
        },
        LoadingLayout,
    ))
    .with_children(|parent| {
        parent.spawn((
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Loading...",
                TextStyle {
                    font_size: 100.0,
                    color: Color::WHITE,
                    ..default()
                },
            ) // Set the alignment of the Text
            .with_style(Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            })
            .with_text_alignment(TextAlignment::Center), // Set the style of the TextBundle itself.
        ));
    });
}
