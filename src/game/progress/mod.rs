mod bar;
mod radial;

pub use bar::spawn;
pub use radial::{RadialProgressCasing, RadialProgressBar, spawn_radial};

use super::{EventState, GameState};
use crate::prelude::*;
use crate::utils::RelEntity;

pub type QueryProgressBar<'w, 's, 'a> =
    Query<'w, 's, (&'a RelEntity, &'a mut Progress, Option<&'a mut RadialProgressBar>)>;
pub struct ProgressPlugin;

impl Plugin for ProgressPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ProgressBarCountUpEvent>()
            .add_message::<ProgressBarFullEvent>()
            .add_message::<ProgressBarResetEvent>()
            .register_type::<Progress>()
            .add_systems(
                Update,
                (
                    bar::scale_system,
                    radial::radial_rotation_system,
                    bar_full_system,
                    bar::activate_animation_system,
                )
                    .run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (on_count_up_system, reset_on_upgrade_system)
                    .run_if(in_state(EventState::Active)),
            );
    }
}

#[derive(Component, Clone, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct Progress(pub f32);

impl Progress {
    fn is_full(&self) -> bool {
        self.0 >= 1.
    }
}

#[derive(Message)]
pub struct ProgressBarCountUpEvent {
    rel_id: Entity,
    amount: f32,
}

impl ProgressBarCountUpEvent {
    pub fn new(rel_id: Entity, amount: f32) -> Self {
        Self { rel_id, amount }
    }

    pub fn rel_id(&self) -> Entity {
        self.rel_id
    }
}

#[allow(clippy::float_cmp)]
fn on_count_up_system(
    mut evr: MessageReader<ProgressBarCountUpEvent>,
    mut q_progress: QueryProgressBar,
) {
    for ev in evr.read() {
        for (_, mut progress, radial) in
            q_progress.iter_mut().filter(|(p, _, _)| p.0 == ev.rel_id)
        {
            let new = (progress.0 + ev.amount).clamp(0., 1.);
            if new != progress.0 {
                progress.0 = new;
                if let Some(mut bar) = radial {
                    bar.stop_fast_forward();
                }
            }
        }
    }
}

fn reset_on_upgrade_system(
    mut evr: MessageReader<ProgressBarResetEvent>,
    mut q_progress: QueryProgressBar,
) {
    for ev in evr.read() {
        for (_, mut progress, radial) in
            q_progress.iter_mut().filter(|(r, _, _)| r.0 == ev.rel_id())
        {
            progress.0 = 0.;
            if let Some(mut bar) = radial {
                bar.start_fast_forward();
            }
        }
    }
}

#[derive(Message)]
pub struct ProgressBarFullEvent(pub Entity);

#[derive(Message)]
pub struct ProgressBarResetEvent {
    rel_id: Entity,
}

impl ProgressBarResetEvent {
    pub fn new(rel_id: Entity) -> Self {
        Self { rel_id }
    }

    pub fn rel_id(&self) -> Entity {
        self.rel_id
    }
}

fn bar_full_system(
    mut full_ev: MessageWriter<ProgressBarFullEvent>,
    q_bar: Query<
        (&RelEntity, &Progress),
        (
            Changed<Progress>,
            Or<(With<bar::ProgressBar>, With<RadialProgressBar>)>,
        ),
    >,
) {
    for (rel_id, bar) in q_bar.iter() {
        if bar.is_full() {
            full_ev.write(ProgressBarFullEvent(rel_id.0));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Resource, Default)]
    struct Collected(Vec<Entity>);

    fn collect_full_events(
        mut evr: MessageReader<ProgressBarFullEvent>,
        mut col: ResMut<Collected>,
    ) {
        for ProgressBarFullEvent(id) in evr.read() {
            col.0.push(*id);
        }
    }

    #[test]
    fn bar_full_system_fires_only_for_physical_bars() {
        let mut app = App::new();
        app.add_message::<ProgressBarFullEvent>();
        app.init_resource::<Collected>();
        app.add_systems(Update, (bar_full_system, collect_full_events).chain());

        let parent = app.world_mut().spawn_empty().id();

        // 3D radial bar
        app.world_mut().spawn((
            RelEntity(parent),
            Progress(1.0),
            RadialProgressBar::default(),
        ));

        // Non-physical bar (e.g. UI bar) without ProgressBar or RadialProgressBar
        app.world_mut().spawn((RelEntity(parent), Progress(1.0)));

        app.update();

        let events = &app.world().resource::<Collected>().0;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], parent);
    }
}
