use super::ball::{self, PinBall};
use super::ball_starter::BallSpawn;
use super::enemy::Enemy;
use super::extra_field::{ActiveEffects, ExtraFieldFireEvent, ExtraFieldKind, lane_occupied};
use super::IngameTime;
use super::audio::SoundEvent;
use crate::prelude::*;
use moonshine_save::prelude::Save;

const EFFECT_SECS: f32 = 5.;

#[derive(Component)]
pub struct BonusBall;

pub fn on_extra_field_fire_system(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut evr: MessageReader<ExtraFieldFireEvent>,
    mut effects: ResMut<ActiveEffects>,
    mut sound_ev: MessageWriter<SoundEvent>,
    ig_time: Res<IngameTime>,
    ball_spawn: Res<BallSpawn>,
    q_ball: Query<&Transform, With<PinBall>>,
) {
    for ExtraFieldFireEvent(kind) in evr.read() {
        match kind {
            ExtraFieldKind::SlowDown => effects.slow_until = **ig_time + EFFECT_SECS,
            ExtraFieldKind::DoubleDamage => effects.double_damage_until = **ig_time + EFFECT_SECS,
            ExtraFieldKind::InstaKill => effects.insta_kill_until = **ig_time + EFFECT_SECS,
            ExtraFieldKind::ExtraBall => {
                let balls: Vec<Vec3> = q_ball.iter().map(|tf| tf.translation).collect();
                if lane_occupied(ball_spawn.0, &balls) {
                    // ponytail: skipped spawn when lane occupied; queue if it matters in playtesting
                    log!("🚫 Extra ball skipped: lane occupied");
                } else {
                    let ball_id = ball::spawn(&mut cmds, &mut meshes, &mut materials, ball_spawn.0);
                    cmds.entity(ball_id).insert(BonusBall).remove::<Save>();
                    sound_ev.write(SoundEvent::ExtraFieldFire);
                }
            }
        }
    }
}

pub fn ball_damage(effects: &ActiveEffects, now: f32, enemy_max_health: f32) -> f32 {
    if effects.is_active(now, ExtraFieldKind::InstaKill) {
        -enemy_max_health
    } else if effects.is_active(now, ExtraFieldKind::DoubleDamage) {
        -200.
    } else {
        -100.
    }
}

pub fn slow_reapply_system(
    effects: Res<ActiveEffects>,
    ig_time: Res<IngameTime>,
    mut q_enemy: Query<&mut Enemy>,
) {
    if !effects.is_active(**ig_time, ExtraFieldKind::SlowDown) {
        return;
    }
    for mut enemy in q_enemy.iter_mut() {
        enemy.slow_down(0.5);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extra_field_effect_active_boundary() {
        let mut effects = ActiveEffects::default();
        assert!(!effects.is_active(0.0, ExtraFieldKind::SlowDown));
        effects.slow_until = 5.;
        assert!(effects.is_active(4.9, ExtraFieldKind::SlowDown));
        assert!(!effects.is_active(5., ExtraFieldKind::SlowDown));
        assert!(!effects.is_active(5.1, ExtraFieldKind::SlowDown));
        assert!(!effects.is_active(4.9, ExtraFieldKind::ExtraBall));
    }

    #[test]
    fn extra_field_ball_damage_modes() {
        let mut effects = ActiveEffects::default();
        assert_eq!(ball_damage(&effects, 10., 300.), -100.);
        effects.double_damage_until = 5.;
        assert_eq!(ball_damage(&effects, 4.9, 300.), -200.);
        assert_eq!(ball_damage(&effects, 5., 300.), -100.);
        effects.insta_kill_until = 20.;
        assert_eq!(ball_damage(&effects, 15., 300.), -300.);
        assert_eq!(ball_damage(&effects, 20., 300.), -100.);
    }
}