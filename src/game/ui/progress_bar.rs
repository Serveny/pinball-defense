use super::PosToRelEntity;
use crate::{
    game::progress::Progress,
    utils::{PercentBw0And1, RelEntity},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct ProgressUiBar;

pub fn spawn(cmds: &mut Commands, rel_id: Entity, start_percent: PercentBw0And1) {
    cmds.spawn((
        Name::new("Progess UI Bar"),
        RelEntity(rel_id),
        PosToRelEntity,
        NodeBundle {
            style: Style {
                width: Val::Percent(2.),
                height: Val::Percent(1.),
                border: UiRect::all(Val::Percent(0.1)),
                padding: UiRect::all(Val::Px(0.)),
                position_type: PositionType::Absolute,
                // Pos bar on middle top of rel entity
                margin: UiRect::new(
                    Val::Percent(-1.),
                    Val::DEFAULT,
                    Val::Percent(-1.5),
                    Val::DEFAULT,
                ),
                ..default()
            },
            border_color: Color::BLACK.into(),
            background_color: Color::WHITE.into(),
            ..default()
        },
    ))
    .with_children(|p| {
        p.spawn((
            ProgressUiBar,
            Progress(start_percent),
            RelEntity(rel_id),
            NodeBundle {
                style: Style {
                    width: Val::Percent(start_percent * 100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                background_color: Color::RED.into(),
                ..default()
            },
        ));
    });
}

pub(super) fn despawn_system(
    mut cmds: Commands,
    q_bar: Query<(Entity, &RelEntity), Or<(With<ProgressUiBar>, With<RelEntity>)>>,
) {
    for (bar_id, rel_id) in q_bar.iter() {
        if cmds.get_entity(rel_id.0).is_none() {
            if let Some(bar) = cmds.get_entity(bar_id) {
                bar.despawn_recursive();
            }
        }
    }
}

// Makes progress visible
pub(super) fn show_progress_system(
    mut q_progress: Query<(&mut Style, &Progress), With<ProgressUiBar>>,
    time: Res<Time>,
) {
    for (mut style, progress) in q_progress.iter_mut() {
        let Val::Percent(mut y) = style.width else {
            return;
        };
        let p = progress.0 * 100.;
        if y < p - 0.5 {
            y += time.delta_seconds() * 100.;
            style.width = Val::Percent(y.clamp(0., 100.));
        } else if y > p + 0.5 {
            y -= time.delta_seconds() * 100.;
            style.width = Val::Percent(y.clamp(0., 100.));
        }
    }
}
