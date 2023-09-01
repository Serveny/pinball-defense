use super::{progress_bar::ProgressBarCountUpEvent, EventState, GameState, IngameTime};
use crate::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeHealthEvent>()
            .add_event::<HealthEmptyEvent>()
            .add_systems(
                Update,
                (health_empty_system, health_recovery_system).run_if(in_state(GameState::Ingame)),
            )
            .add_systems(
                Update,
                (on_change_health_system).run_if(in_state(EventState::Active)),
            );
    }
}
#[derive(Component)]
pub struct Health {
    current: f32,
    max: f32,
}

#[derive(Event)]
pub struct ChangeHealthEvent {
    id: Entity,
    amount: f32,
}

impl ChangeHealthEvent {
    pub fn new(id: Entity, amount: f32) -> Self {
        Self { id, amount }
    }
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn to_progress(&self, amount: f32) -> f32 {
        amount / self.max
    }

    pub fn add(&mut self, amount: f32) {
        self.current = (self.current + amount).clamp(0., self.max);
    }

    pub fn is_empty(&self) -> bool {
        self.current <= 0.
    }

    pub fn is_full(&self) -> bool {
        self.current >= self.max
    }
}

fn on_change_health_system(
    ig_time: Res<IngameTime>,
    mut evr: EventReader<ChangeHealthEvent>,
    mut q_health: Query<(&mut Health, Option<&mut HealthRecovery>)>,
    mut prog_bar_ev: EventWriter<ProgressBarCountUpEvent>,
) {
    for ev in evr.iter() {
        if let Ok((mut health, recovery)) = q_health.get_mut(ev.id) {
            health.add(ev.amount);
            prog_bar_ev.send(ProgressBarCountUpEvent::new(
                ev.id,
                health.to_progress(ev.amount),
            ));

            // Health recovery
            if let Some(mut recovery) = recovery {
                if ev.amount.is_sign_negative() {
                    recovery.set_time(ig_time.0);
                }
            }
        }
    }
}

// Remainder: Need this, because not every health has a progress bar
#[derive(Event)]
pub struct HealthEmptyEvent(pub Entity);

fn health_empty_system(
    mut empty_ev: EventWriter<HealthEmptyEvent>,
    q_health: Query<(Entity, &Health), Changed<Health>>,
) {
    for (id, health) in q_health.iter() {
        if health.is_empty() {
            empty_ev.send(HealthEmptyEvent(id));
        }
    }
}

#[derive(Component, Debug)]
pub struct HealthRecovery {
    amount_per_second: f32,
    timeout_after_damage: f32,
    time_next_recover: f32,
}

impl HealthRecovery {
    pub fn new(amount_per_second: f32, timeout_last_damage: f32) -> Self {
        Self {
            amount_per_second,
            timeout_after_damage: timeout_last_damage,
            time_next_recover: 0.,
        }
    }

    fn set_time(&mut self, ig_time: f32) {
        self.time_next_recover = ig_time + self.timeout_after_damage
    }

    fn can_recover(&self, ig_time: f32) -> bool {
        self.time_next_recover <= ig_time
    }

    fn health(&self, delta_sec: f32) -> f32 {
        self.amount_per_second * delta_sec
    }
}

fn health_recovery_system(
    time: Res<Time>,
    ig_time: Res<IngameTime>,
    q_recovery: Query<(Entity, &Health, &HealthRecovery)>,
    mut health_ev: EventWriter<ChangeHealthEvent>,
) {
    for (id, health, rec) in q_recovery.iter() {
        //log!(
        //"full: {}, can_recover: {}",
        //health.is_full(),
        //rec.can_recover(ig_time.0)
        //);
        if !health.is_full() && rec.can_recover(ig_time.0) {
            health_ev.send(ChangeHealthEvent::new(id, rec.health(time.delta_seconds())));
        }
    }
}
