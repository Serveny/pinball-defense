use crate::pinball_menu::PinballMenuStatus;
use crate::prelude::*;
use bevy_tweening::TweenCompleted;

pub const DESPAWN_ENTITY_EVENT_ID: u64 = 0;
pub const ACTIVATE_PINBALL_MENU_EVENT_ID: u64 = 1;

pub(super) fn tween_completed_system(
    mut evr: EventReader<TweenCompleted>,
    mut cmds: Commands,
    mut q_pbm: Query<&mut PinballMenuStatus>,
) {
    for ev in evr.iter() {
        match ev.user_data {
            DESPAWN_ENTITY_EVENT_ID => cmds.entity(ev.entity).despawn_recursive(),
            ACTIVATE_PINBALL_MENU_EVENT_ID => *q_pbm.single_mut() = PinballMenuStatus::Ready,
            _ => panic!("😭 Unkown tween user event: {}", ev.user_data),
        }
    }
}
