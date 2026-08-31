use crate::AppState;
use crate::prelude::*;
use bevy::text::FontSize;

pub struct LoadingScreenPlugin;

#[derive(Component, Clone, Default)]
struct LoadingLayout;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), spawn_layout)
            .add_systems(OnExit(AppState::Loading), despawn_layout);
    }
}

fn despawn_layout(mut cmds: Commands, q_layout: Query<Entity, With<LoadingLayout>>) {
    for entity in q_layout {
        if let Ok(mut entity_cmds) = cmds.get_entity(entity) {
            entity_cmds.despawn();
        }
    }
}

fn spawn_layout(mut cmds: Commands) {
    log!("Loading Sceen 🤡");
    cmds.spawn_scene(bsn! {
        Camera2d
        LoadingLayout
    });
    cmds.spawn_scene(bsn! {
        Node {
            border: UiRect::percent(10., 0., 25., 25.),
        }
        LoadingLayout
        Children [
            (Text("Loading...")
             TextFont { font_size: FontSize::Px(100.0) }
             TextColor({Color::WHITE})
             Node { width: Val::Percent(100.), height: Val::Percent(100.), justify_content: JustifyContent::Center })
        ]
    });
}
