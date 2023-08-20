use super::target::AimFirstEnemy;
use crate::game::progress_bar::ProgressBarCountUpEvent;
use crate::prelude::*;

pub(super) type DamagePerSecond = f32;

#[derive(Component)]
pub(super) struct DamageOverTime(pub DamagePerSecond);

pub(super) fn afe_damage_over_time_system(
    time: Res<Time>,
    q_target: Query<(&AimFirstEnemy, &DamageOverTime)>,
    mut prog_bar_ev: EventWriter<ProgressBarCountUpEvent>,
) {
    for (target, damage) in q_target.iter() {
        if let Some(enemy_id) = target.0 {
            prog_bar_ev.send(ProgressBarCountUpEvent(
                enemy_id,
                -damage.0 * time.delta_seconds(),
            ));
        }
    }
}
