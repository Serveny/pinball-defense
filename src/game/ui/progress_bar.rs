use super::PosToRelEntity;
use crate::{
    game::progress::Progress,
    utils::{PercentBw0And1, RelEntity},
};
use bevy::color::palettes::css::RED;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct ProgressUiBar {
    is_active_animation: bool,
}

pub fn spawn(cmds: &mut Commands, rel_id: Entity, start_percent: PercentBw0And1) {
    cmds.spawn((
        Name::new("Progess UI Bar"),
        RelEntity(rel_id),
        PosToRelEntity,
        Node {
            width: Val::Percent(3.),
            height: Val::Percent(1.5),
            border: UiRect::all(Val::Percent(0.1)),
            padding: UiRect::all(Val::Px(0.)),
            position_type: PositionType::Absolute,
            // Pos bar on middle top of rel entity
            margin: UiRect::new(
                Val::Percent(-1.5),
                Val::DEFAULT,
                Val::Percent(-1.5),
                Val::DEFAULT,
            ),
            ..default()
        },
        BorderColor::from(Color::BLACK),
        BackgroundColor(Color::WHITE),
    ))
    .with_children(|p| {
        p.spawn((
            ProgressUiBar::default(),
            Progress(start_percent),
            RelEntity(rel_id),
            Node {
                width: Val::Percent(start_percent * 100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(RED.into()),
        ));
    });
}

pub(super) fn despawn_system(
    mut cmds: Commands,
    q_bar: Query<(Entity, &RelEntity), Or<(With<ProgressUiBar>, With<RelEntity>)>>,
) {
    for (bar_id, rel_id) in q_bar.iter() {
        if cmds.get_entity(rel_id.0).is_err() {
            if let Ok(mut bar) = cmds.get_entity(bar_id) {
                bar.despawn();
            }
        }
    }
}

const TOLERANCE: f32 = 1.;
fn is_almost_eq(a: f32, b: f32) -> bool {
    return ((a - TOLERANCE)..(a + TOLERANCE)).contains(&b);
}

pub(super) fn activate_animation_system(
    mut q_progess: Query<&mut ProgressUiBar, Changed<Progress>>,
) {
    for mut bar in q_progess.iter_mut() {
        bar.is_active_animation = true;
    }
}

// Makes progress visible
pub(super) fn show_progress_system(
    mut q_progress: Query<(&mut Node, &Progress, &mut ProgressUiBar)>,
    time: Res<Time>,
) {
    for (mut style, progress, mut bar) in q_progress
        .iter_mut()
        .filter(|(_, _, bar)| bar.is_active_animation)
    {
        let Val::Percent(mut y) = style.width else {
            return;
        };
        let p = progress.0 * 100.;
        y += time.delta_secs() * 100. * (p - y).signum();

        if is_almost_eq(y, p) {
            y = p;
            bar.is_active_animation = false;
        }

        style.width = Val::Percent(y.clamp(0., 100.));
    }
}
