use crate::prelude::*;
use bevy_tweening::TweenCompleted;

pub const DESPAWN_ENTITY_AND_MENU_EVENT_ID: u64 = 0;
//pub const PINBALL_MENU_SPAWN_ANIMATION_END_EVENT: u64 = 1;

pub(super) fn tween_completed_system(mut evr: EventReader<TweenCompleted>, mut cmds: Commands) {
    for ev in evr.iter() {
        match ev.user_data {
            DESPAWN_ENTITY_AND_MENU_EVENT_ID => cmds.entity(ev.entity).despawn_recursive(),
            _ => panic!("😭 Unkown tween user event: {}", ev.user_data),
        }
    }
}
