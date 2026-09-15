use super::ExtraFieldFireEvent;
use super::ExtraFieldKind;
use super::field::lane_occupied;
use crate::game::IngameTime;
use crate::game::audio::SoundEvent;
use crate::game::ball::{self, PinBall};
use crate::game::ball_starter::{BallSpawn, BallStarterState};
use crate::game::cfg::CONFIG;
use crate::game::enemy::Enemy;
use crate::prelude::*;
use moonshine_save::prelude::Save;

const EFFECT_SECS: f32 = 10.;

#[derive(Component)]
pub struct BonusBall;

pub(super) fn on_extra_field_fire_system(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut evr: MessageReader<ExtraFieldFireEvent>,
    mut effects: ResMut<ActiveEffects>,
    mut sound_ev: MessageWriter<SoundEvent>,
    ig_time: Res<IngameTime>,
    ball_spawn: Res<BallSpawn>,
    mut ball_starter_state: ResMut<NextState<BallStarterState>>,
    q_ball: Query<&Transform, With<PinBall>>,
) {
    for ExtraFieldFireEvent(kind) in evr.read() {
        match kind {
            ExtraFieldKind::SlowDown => effects.slow_until = **ig_time + EFFECT_SECS,
            ExtraFieldKind::DoubleDamage => effects.double_damage_until = **ig_time + EFFECT_SECS,
            ExtraFieldKind::InstaKill => effects.insta_kill_until = **ig_time + EFFECT_SECS,
            ExtraFieldKind::ExtraBall => effects.extra_ball_until = **ig_time + EFFECT_SECS,
        }
        if let ExtraFieldKind::ExtraBall = kind {
            let balls: Vec<Vec3> = q_ball.iter().map(|tf| tf.translation).collect();
            if lane_occupied(ball_spawn.0, &balls) {
                // ponytail: skipped spawn when lane occupied; queue if it matters in playtesting
                log!("🚫 Extra ball skipped: lane occupied");
            } else {
                let ball_id = ball::spawn(&mut cmds, &mut meshes, &mut materials, ball_spawn.0);
                cmds.entity(ball_id).insert(BonusBall).remove::<Save>();
                ball_starter_state.set(BallStarterState::AutoLaunch);
                sound_ev.write(SoundEvent::ExtraFieldFire);
            }
        }
    }
}

pub fn ball_damage(
    effects: &ActiveEffects,
    now: f32,
    enemy_max_health: f32,
    level: crate::game::level::Level,
) -> f32 {
    let base = -CONFIG.ball_enemy_damage - f32::from(level) * CONFIG.ball_damage_per_level;
    if effects.is_active(now, ExtraFieldKind::InstaKill) {
        -enemy_max_health
    } else if effects.is_active(now, ExtraFieldKind::DoubleDamage) {
        2. * base
    } else {
        base
    }
}

pub(super) fn slow_reapply_system(
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

#[derive(Resource, Default)]
#[allow(clippy::struct_field_names)]
pub struct ActiveEffects {
    pub slow_until: f32,
    pub double_damage_until: f32,
    pub insta_kill_until: f32,
    pub extra_ball_until: f32,
}

impl ActiveEffects {
    pub fn is_active(&self, now: f32, which: ExtraFieldKind) -> bool {
        match which {
            ExtraFieldKind::SlowDown => now < self.slow_until,
            ExtraFieldKind::DoubleDamage => now < self.double_damage_until,
            ExtraFieldKind::InstaKill => now < self.insta_kill_until,
            ExtraFieldKind::ExtraBall => now < self.extra_ball_until,
        }
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
        effects.extra_ball_until = 5.;
        assert!(effects.is_active(4.9, ExtraFieldKind::ExtraBall));
        assert!(!effects.is_active(5., ExtraFieldKind::ExtraBall));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn extra_field_ball_damage_modes() {
        let mut effects = ActiveEffects::default();
        assert_eq!(
            ball_damage(&effects, 10., 300., 0),
            -CONFIG.ball_enemy_damage
        );
        effects.double_damage_until = 5.;
        assert_eq!(
            ball_damage(&effects, 4.9, 300., 0),
            -2. * CONFIG.ball_enemy_damage
        );
        assert_eq!(
            ball_damage(&effects, 5., 300., 0),
            -CONFIG.ball_enemy_damage
        );
        effects.insta_kill_until = 20.;
        assert_eq!(ball_damage(&effects, 15., 300., 0), -300.);
        assert_eq!(
            ball_damage(&effects, 20., 300., 0),
            -CONFIG.ball_enemy_damage
        );
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn ball_damage_scales_with_level() {
        let effects = ActiveEffects::default();
        let expected =
            |lvl: u8| -(CONFIG.ball_enemy_damage + f32::from(lvl) * CONFIG.ball_damage_per_level);
        assert_eq!(ball_damage(&effects, 10., 300., 0), expected(0));
        assert_eq!(ball_damage(&effects, 10., 300., 3), expected(3));
    }
}
