use crate::prelude::*;
use crate::AppState;

pub struct LoadingScreenPlugin;

#[derive(Component)]
struct LoadingLayout;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), spawn_layout)
            .add_systems(OnExit(AppState::Loading), despawn_layout);
    }
}

fn despawn_layout(mut cmds: Commands, q_layout: Query<Entity, With<LoadingLayout>>) {
    for entity in q_layout {
        cmds.get_entity(entity).unwrap().despawn();
    }
}

fn spawn_layout(mut cmds: Commands) {
    log!("Loading Sceen 🤤");
    cmds.spawn((Camera2d::default(), LoadingLayout));
    cmds.spawn((
        Node {
            border: UiRect::percent(10., 0., 25., 25.),
            ..default()
        },
        LoadingLayout,
    ))
    .with_children(|spawner| {
        spawner.spawn((
            Text("Loading...".into()),
            TextFont {
                font_size: 100.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ));
    });
}
