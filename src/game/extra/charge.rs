use super::KINDS;
use super::effects::ActiveEffects;
use super::ExtraField;
use super::ExtraFieldFireEvent;
use crate::game::IngameTime;
use crate::game::audio::SoundEvent;
use crate::game::ball::CollisionWithBallEvent;
use crate::game::progress::{
    ProgressBarCountUpEvent, ProgressBarFullEvent, ProgressBarResetEvent, RadialProgressBar,
};
use crate::prelude::*;
use crate::utils::RelEntity;

fn charge_amount(hits_needed: u32) -> f32 {
    1. / f32::from(u8::try_from(hits_needed.max(1)).unwrap_or(u8::MAX))
}

pub(super) fn on_charge_system(
    mut prog_bar_ev: MessageWriter<ProgressBarCountUpEvent>,
    mut sound_ev: MessageWriter<SoundEvent>,
    mut evr: MessageReader<CollisionWithBallEvent>,
    q_field: Query<&ExtraField>,
    q_bar: Query<(&RelEntity, &RadialProgressBar)>,
    effects: Res<ActiveEffects>,
    ig_time: Res<IngameTime>,
) {
    for CollisionWithBallEvent(_, id) in evr.read() {
        if let Ok(field) = q_field.get(*id) {
            let rewinding = q_bar
                .iter()
                .any(|(rel, bar)| rel.0 == *id && bar.is_rewinding());
            if effects.is_active(**ig_time, field.kind()) || rewinding {
                continue;
            }
            prog_bar_ev.write(ProgressBarCountUpEvent::new(
                *id,
                charge_amount(field.kind().hits_needed()),
            ));
            sound_ev.write(SoundEvent::ExtraFieldHit);
        }
    }
}

pub(super) fn on_fire_system(
    mut fire_ev: MessageWriter<ExtraFieldFireEvent>,
    mut evr: MessageReader<ProgressBarFullEvent>,
    q_field: Query<&ExtraField>,
) {
    for ProgressBarFullEvent(id) in evr.read() {
        if let Ok(field) = q_field.get(*id) {
            fire_ev.write(ExtraFieldFireEvent(field.kind()));
        }
    }
}

pub(super) fn rewind_on_effect_end_system(
    mut reset_ev: MessageWriter<ProgressBarResetEvent>,
    effects: Res<ActiveEffects>,
    ig_time: Res<IngameTime>,
    mut prev_active: Local<[bool; 4]>,
    q_field: Query<(Entity, &ExtraField)>,
) {
    for (i, kind) in KINDS.into_iter().enumerate() {
        let now_active = effects.is_active(**ig_time, kind);
        let Some(prev) = prev_active.get_mut(i) else {
            continue;
        };
        if *prev && !now_active {
            for (field_id, field) in q_field.iter() {
                if field.kind() == kind {
                    reset_ev.write(ProgressBarResetEvent::new(field_id));
                }
            }
        }
        *prev = now_active;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn extra_field_charge_amount_is_inverse_of_hits() {
        assert_eq!(charge_amount(2), 0.5);
        assert_eq!(charge_amount(4), 0.25);
        assert_eq!(charge_amount(8), 0.125);
        assert!(charge_amount(0).is_finite());
    }
}