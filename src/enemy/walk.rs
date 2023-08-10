use super::Enemy;
use crate::player_life::LifeBar;
use crate::prelude::*;
use crate::progress_bar::ProgressBarCountUpEvent;

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
                cmds.entity(enemy_id).despawn_recursive();
                end_reached_ev.send(RoadEndReachedEvent);
            }
        };
    }
}

#[derive(Event)]
pub(super) struct RoadEndReachedEvent;

pub(super) fn road_end_reached_system(
    mut evr: EventReader<RoadEndReachedEvent>,
    mut progress_ev: EventWriter<ProgressBarCountUpEvent>,
    q_life_bar: Query<Entity, With<LifeBar>>,
) {
    for _ in evr.iter() {
        log!("🔚 Enemy reached road end");
        progress_ev.send(ProgressBarCountUpEvent(q_life_bar.single(), -0.1));
    }
}
