use super::Enemy;
use crate::game::audio::SoundEvent;
use crate::game::health::ChangeHealthEvent;
use crate::game::player_life::LifeBar;
use crate::prelude::*;

pub(super) const WALK_SPEED: f32 = 0.2;

pub(super) fn walk_system(
    mut cmds: Commands,
    mut q_enemy: Query<(Entity, &mut Transform, &mut Enemy)>,
    mut end_reached_ev: EventWriter<RoadEndReachedEvent>,
    time: Res<Time>,
) {
    for (enemy_id, mut trans, mut enemy) in q_enemy.iter_mut() {
        match enemy.walk(trans.translation, time.delta()) {
            Some(pos) => trans.translation = pos,
            None => {
                // Reminder: If you need infos about the enemy, overgive only infos, not enemy id
                end_reached_ev.send(RoadEndReachedEvent);

                // Delete enemy here, to prevent double events
                cmds.entity(enemy_id).despawn_recursive();
            }
        };
    }
}

#[derive(Event)]
pub(super) struct RoadEndReachedEvent;

pub(super) fn on_road_end_reached_system(
    mut evr: EventReader<RoadEndReachedEvent>,
    mut health_ev: EventWriter<ChangeHealthEvent>,
    mut sound_ev: EventWriter<SoundEvent>,
    q_life_bar: Query<Entity, With<LifeBar>>,
) {
    for _ in evr.read() {
        log!("🔚 Enemy reached road end");

        health_ev.send(ChangeHealthEvent::new(q_life_bar.single(), -10., None));
        sound_ev.send(SoundEvent::EnemyReachEnd);
    }
}

pub(super) fn recover_speed_system(time: Res<Time>, mut q_enemy: Query<&mut Enemy>) {
    for mut enemy in q_enemy.iter_mut() {
        if enemy.current_speed < enemy.speed {
            enemy.current_speed += time.delta_secs() * 0.2;
            enemy.current_speed = enemy.current_speed.clamp(0., enemy.speed);
        }
    }
}
