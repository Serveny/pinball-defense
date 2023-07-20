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
    cmds.spawn(Camera2dBundle::default()).insert(LoadingLayout);
    cmds.spawn(
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Loading...",
            TextStyle {
                font: Default::default(),
                font_size: 200.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
    )
    .insert(LoadingLayout);
}
