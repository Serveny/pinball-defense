use super::PosToRelEntity;
use avian2d::prelude::Collider;
use crate::game::enemy::Enemy;
use crate::game::extra::ExtraField;
use crate::game::health::Health;
use crate::game::progress::{Progress, ProgressBarCountUpEvent, ProgressBarResetEvent};
use crate::game::tower::Tower;
use crate::game::tower::foundation::TowerFoundation;
use crate::utils::{PercentBw0And1, RelEntity};
use bevy::color::palettes::css::RED;
use bevy::prelude::*;

#[derive(Component, Clone, Default)]
pub struct ProgressUiBar {
    is_active_animation: bool,
    // While locked the fill is not animated. Set when an upgrade resets the
    // progress to 0, so the bar never visibly drains back down. Cleared once
    // the bar is shown again (and the fill-up animation is displayed).
    is_locked: bool,
}

// Enemy health bars count down in RED. Tower/base progress counts up, so it
// gets a distinct color to communicate "progress" rather than "damage".
const PROGRESS_COLOR: Color = Color::srgb_u8(70, 200, 120);

/// How long a transient progress bar stays visible after the last hit.
const TRANSIENT_VISIBLE_SECS: f32 = 2.;

/// A UI progress bar that is hidden by default and only appears for
/// `TRANSIENT_VISIBLE_SECS` after the related entity received progress.
#[derive(Component, Clone)]
pub struct TransientProgressUiBar {
    hide_timer: Timer,
}

impl Default for TransientProgressUiBar {
    fn default() -> Self {
        Self {
            hide_timer: Timer::from_seconds(TRANSIENT_VISIBLE_SECS, TimerMode::Once),
        }
    }
}

/// Always-visible UI bar (used for enemy health). Fill counts down.
pub fn spawn(cmds: &mut Commands, rel_id: Entity, start_percent: PercentBw0And1) {
    cmds.spawn_scene(bsn! {
        Name::new("Progess UI Bar")
        RelEntity({rel_id})
        PosToRelEntity
        Node {
            width: Val::Percent(3.),
            height: Val::Percent(1.5),
            border: UiRect::all(Val::Percent(0.1)),
            padding: UiRect::all(Val::Px(0.)),
            position_type: PositionType::Absolute,
            // Pos bar on middle top of rel entity
            margin: UiRect::new(Val::Percent(-1.5), Val::DEFAULT, Val::Percent(-1.5), Val::DEFAULT),
        }
        BorderColor::from(Color::BLACK)
        BackgroundColor({Color::WHITE})
        Children [
            (ProgressUiBar
             Progress({start_percent})
             RelEntity({rel_id})
             Node { width: Val::Percent({start_percent * 100.}), height: Val::Percent(100.) }
             BackgroundColor({RED}))
        ]
    });
}

/// Hidden UI progress bar for a tower/base. It appears for
/// `TRANSIENT_VISIBLE_SECS` whenever the related entity receives progress
/// (e.g. the ball hits a tower/foundation).
pub fn spawn_transient(cmds: &mut Commands, rel_id: Entity, init_val: PercentBw0And1) {
    spawn_transient_with_color(cmds, rel_id, init_val, PROGRESS_COLOR);
}

pub fn spawn_transient_with_color(
    cmds: &mut Commands,
    rel_id: Entity,
    init_val: PercentBw0And1,
    color: Color,
) {
    cmds.spawn_scene(bsn! {
        Name::new("Progress UI Bar")
        RelEntity({rel_id})
        PosToRelEntity
        TransientProgressUiBar
        Node {
            width: Val::Percent(3.),
            height: Val::Percent(1.5),
            border: UiRect::all(Val::Percent(0.1)),
            padding: UiRect::all(Val::Px(0.)),
            position_type: PositionType::Absolute,
            // Pos bar on middle top of rel entity
            margin: UiRect::new(Val::Percent(-1.5), Val::DEFAULT, Val::Percent(-1.5), Val::DEFAULT),
        }
        BorderColor::from(Color::BLACK)
        BackgroundColor({Color::WHITE})
        Children [
            (ProgressUiBar
             Progress({init_val})
             RelEntity({rel_id})
             Node { width: Val::Percent({init_val * 100.}), height: Val::Percent(100.) }
             BackgroundColor({color}))
        ]
    })
    .insert(Visibility::Hidden);
}

pub(super) fn despawn_system(
    mut cmds: Commands,
    q_bar: Query<(Entity, &RelEntity), With<PosToRelEntity>>,
) {
    for (bar_id, rel_id) in q_bar.iter() {
        if cmds.get_entity(rel_id.0).is_err()
            && let Ok(mut bar) = cmds.get_entity(bar_id)
        {
            bar.despawn();
        }
    }
}

const TOLERANCE: f32 = 1.;
fn is_almost_eq(a: f32, b: f32) -> bool {
    ((a - TOLERANCE)..(a + TOLERANCE)).contains(&b)
}

pub(super) fn activate_animation_system(
    mut q_progess: Query<&mut ProgressUiBar, Changed<Progress>>,
) {
    for mut bar in q_progess.iter_mut() {
        if bar.is_locked {
            continue;
        }
        bar.is_active_animation = true;
    }
}

// Makes progress visible
pub(super) fn show_progress_system(
    mut q_progress: Query<(&mut Node, &Progress, &mut ProgressUiBar)>,
    time: Res<Time>,
) {
    for (mut style, progress, mut bar) in q_progress.iter_mut() {
        if bar.is_locked || !bar.is_active_animation {
            continue;
        }
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

/// Reveal transient bars for `TRANSIENT_VISIBLE_SECS` when their related entity
/// receives progress (ball hit, enemy killed, ...). If the bar was locked
/// (after an upgrade reset) it is unlocked and snaps back to 0 so the fill-up
/// animation can run fresh.
pub(super) fn show_on_hit_system(
    mut evr: MessageReader<ProgressBarCountUpEvent>,
    mut q_bar: Query<(&RelEntity, &mut Visibility, &mut TransientProgressUiBar)>,
    mut q_fill: Query<(&RelEntity, &mut Node, &mut ProgressUiBar)>,
) {
    for ev in evr.read() {
        let id = ev.rel_id();
        if let Some((_, mut node, mut fill)) = q_fill.iter_mut().find(|(r, _, _)| r.0 == id)
            && fill.is_locked
        {
            fill.is_locked = false;
            fill.is_active_animation = true;
            node.width = Val::Percent(0.);
        }
        if let Some((_, mut vis, mut bar)) = q_bar.iter_mut().find(|(r, _, _)| r.0 == id) {
            *vis = Visibility::Visible;
            bar.hide_timer.reset();
        }
    }
}

/// Hide a transient bar one frame after its fill animation reached 100%, so the
/// player sees it fill up completely before it disappears. (Runs before
/// `show_progress_system`, therefore it reacts to the previous frame's
/// completed animation, giving the full 100% frame time to render.)
pub(super) fn hide_when_fill_complete_system(
    q_fill: Query<(&RelEntity, &Progress, &ProgressUiBar)>,
    mut q_bar: Query<(&RelEntity, &mut Visibility), With<TransientProgressUiBar>>,
) {
    for (bar_rel, mut vis) in q_bar.iter_mut() {
        let done = q_fill
            .iter()
            .any(|(f_rel, p, fill)| f_rel.0 == bar_rel.0 && p.0 >= 1. && !fill.is_active_animation);
        if done {
            *vis = Visibility::Hidden;
        }
    }
}

/// Hide transient bars again once their visibility window elapsed.
pub(super) fn hide_after_timeout_system(
    time: Res<Time>,
    mut q_bar: Query<(&mut Visibility, &mut TransientProgressUiBar)>,
) {
    for (mut vis, mut bar) in q_bar.iter_mut() {
        bar.hide_timer.tick(time.delta());
        if bar.hide_timer.just_finished() {
            *vis = Visibility::Hidden;
        }
    }
}

/// On upgrade the progress resets to 0. Lock the fill (so the bar does not
/// update/animate toward 0 and never looks like it is draining) and hide the
/// bar. The lock is released by `show_on_hit_system` once the bar is shown
/// again and the fill-up animation is displayed.
pub(super) fn reset_on_upgrade_system(
    mut evr: MessageReader<ProgressBarResetEvent>,
    mut q_bar: Query<(&RelEntity, &mut Visibility), With<TransientProgressUiBar>>,
    mut q_fill: Query<(&RelEntity, &mut Progress, &mut ProgressUiBar)>,
) {
    for ev in evr.read() {
        let id = ev.rel_id();
        for (rel_id, mut vis) in q_bar.iter_mut() {
            if rel_id.0 == id {
                *vis = Visibility::Hidden;
            }
        }
        for (rel_id, mut progress, mut fill) in q_fill.iter_mut() {
            if rel_id.0 == id {
                progress.0 = 0.;
                fill.is_locked = true;
                fill.is_active_animation = false;
            }
        }
    }
}

#[allow(clippy::float_cmp)]
pub(super) fn sync_progress_to_entities(
    q_bars: Query<(&RelEntity, &Progress), With<ProgressUiBar>>,
    mut q_entities: Query<&mut Progress, Without<ProgressUiBar>>,
) {
    for (rel, bar_progress) in q_bars.iter() {
        if let Ok(mut entity_progress) = q_entities.get_mut(rel.0)
            && entity_progress.0 != bar_progress.0
        {
            entity_progress.0 = bar_progress.0;
        }
    }
}

pub(super) fn ensure_bars_on_load(
    mut cmds: Commands,
    q_entities: Query<
        (
            Entity,
            Option<&Progress>,
            Option<&Health>,
            Option<&ExtraField>,
        ),
        Or<(
            With<Tower>,
            With<TowerFoundation>,
            With<Enemy>,
            (With<ExtraField>, With<Collider>),
        )>,
    >,
    q_bars: Query<&RelEntity, With<ProgressUiBar>>,
) {
    for (entity, progress, health, extra) in q_entities.iter() {
        if q_bars.iter().any(|r| r.0 == entity) {
            continue;
        }
        if let Some(extra) = extra {
            spawn_transient_with_color(&mut cmds, entity, 0., extra.kind().color());
        } else if let Some(progress) = progress {
            spawn_transient(&mut cmds, entity, progress.0);
        } else if let Some(health) = health {
            spawn(&mut cmds, entity, health.fraction());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::extra::ExtraFieldKind;
    use bevy::ecs::system::RunSystemOnce;
    use std::time::Duration;

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            bevy::asset::AssetPlugin::default(),
            bevy::scene::ScenePlugin::default(),
        ));
        app
    }

    #[test]
    fn transient_bar_spawns_with_custom_color_and_hidden() {
        let mut app = test_app();
        let entity = app.world_mut().spawn_empty().id();
        let color = ExtraFieldKind::SlowDown.color();

        let mut cmds = app.world_mut().commands();
        spawn_transient_with_color(&mut cmds, entity, 0., color);
        app.update();

        let mut q_bar = app
            .world_mut()
            .query::<(&RelEntity, &Visibility, &TransientProgressUiBar)>();
        let (rel, vis, _) = q_bar
            .iter(app.world())
            .find(|(r, _, _)| r.0 == entity)
            .expect("Transient bar should exist");
        assert_eq!(rel.0, entity);
        assert_eq!(*vis, Visibility::Hidden);

        let mut q_fill = app
            .world_mut()
            .query::<(&RelEntity, &BackgroundColor, &ProgressUiBar)>();
        let (fill_rel, bg, _) = q_fill
            .iter(app.world())
            .find(|(r, _, _)| r.0 == entity)
            .expect("ProgressUiBar child should exist");
        assert_eq!(fill_rel.0, entity);
        assert_eq!(bg.0, color);
    }

    #[test]
    fn transient_bar_shown_on_hit_and_resets_timer() {
        let mut app = test_app();
        app.add_message::<ProgressBarCountUpEvent>();
        app.add_systems(Update, show_on_hit_system);

        let entity = app.world_mut().spawn_empty().id();
        let mut cmds = app.world_mut().commands();
        spawn_transient_with_color(&mut cmds, entity, 0., ExtraFieldKind::ExtraBall.color());
        app.update();

        app.world_mut()
            .write_message(ProgressBarCountUpEvent::new(entity, 0.25));
        app.update();

        let mut q_bar = app
            .world_mut()
            .query::<(&RelEntity, &Visibility, &TransientProgressUiBar)>();
        let (_, vis, _) = q_bar
            .iter(app.world())
            .find(|(r, _, _)| r.0 == entity)
            .unwrap();
        assert_eq!(*vis, Visibility::Visible);
    }

    #[test]
    fn transient_bar_hides_after_timeout() {
        let mut app = test_app();

        let entity = app.world_mut().spawn_empty().id();
        let mut cmds = app.world_mut().commands();
        spawn_transient_with_color(&mut cmds, entity, 0., ExtraFieldKind::DoubleDamage.color());
        app.update();

        let (bar_id, mut vis, _) = app
            .world_mut()
            .query::<(Entity, &mut Visibility, &mut TransientProgressUiBar)>()
            .single_mut(app.world_mut())
            .unwrap();
        *vis = Visibility::Visible;

        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(Duration::from_secs_f32(TRANSIENT_VISIBLE_SECS));
        }
        let _ = app.world_mut().run_system_once(hide_after_timeout_system);

        let vis = app.world().get::<Visibility>(bar_id).unwrap();
        assert_eq!(*vis, Visibility::Hidden);
    }

    #[test]
    fn transient_bar_hidden_and_reset_on_progress_reset_event() {
        let mut app = test_app();
        app.add_message::<ProgressBarResetEvent>();
        app.add_systems(Update, reset_on_upgrade_system);

        let entity = app.world_mut().spawn_empty().id();
        let mut cmds = app.world_mut().commands();
        spawn_transient_with_color(&mut cmds, entity, 0.5, ExtraFieldKind::InstaKill.color());
        app.update();

        for mut vis in app
            .world_mut()
            .query::<&mut Visibility>()
            .iter_mut(app.world_mut())
        {
            *vis = Visibility::Visible;
        }

        app.world_mut()
            .write_message(ProgressBarResetEvent::new(entity));
        app.update();

        let mut q_bar = app
            .world_mut()
            .query::<(&RelEntity, &Visibility, &TransientProgressUiBar)>();
        let (_, vis, _) = q_bar
            .iter(app.world())
            .find(|(r, _, _)| r.0 == entity)
            .unwrap();
        assert_eq!(*vis, Visibility::Hidden);

        let mut q_fill = app
            .world_mut()
            .query::<(&RelEntity, &Progress, &ProgressUiBar)>();
        let (_, progress, fill) = q_fill
            .iter(app.world())
            .find(|(r, _, _)| r.0 == entity)
            .unwrap();
        assert_eq!(progress.0, 0.);
        assert!(fill.is_locked);
    }
}
